//! One persistent child, serialized requests, bounded waits, no automatic retry of failed inputs.
use super::{BackendError, Decoded, Operation, read_response, write_request};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
    mpsc,
};
use std::time::{Duration, Instant};

type Reply = Result<Decoded, BackendError>;
struct Task {
    operation: Operation,
    bytes: Arc<[u8]>,
    reply: mpsc::Sender<Reply>,
}
struct Worker {
    child: Arc<Mutex<Child>>,
    tasks: mpsc::Sender<Task>,
}
impl Drop for Worker {
    fn drop(&mut self) {
        let mut child = self.child.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
        let _ = child.kill();
        let _ = child.wait();
    }
}

pub struct Client {
    path: PathBuf,
    worker: Mutex<Option<Worker>>,
    timeout: Duration,
}
impl Client {
    pub fn new(path: PathBuf) -> Self {
        Self { path, worker: Mutex::new(None), timeout: Duration::from_secs(120) }
    }
    pub fn installed() -> Result<Self, String> {
        if let Some(path) = std::env::var_os("LIGHTCRAFT_RAW_WORKER") {
            return Ok(Self::new(path.into()));
        }
        let exe = std::env::current_exe().map_err(|e| e.to_string())?;
        let parent = exe.parent().ok_or("Cannot locate RAW worker beside application")?;
        // Cargo integration tests execute from target/<profile>/deps.
        let parent = if parent.file_name().is_some_and(|n| n == "deps") { parent.parent().unwrap_or(parent) } else { parent };
        Ok(Self::new(parent.join("lightcraft-raw-worker")))
    }
    pub fn request(&self, operation: Operation, bytes: Arc<[u8]>) -> Reply {
        self.request_with_cancel(operation, bytes, &AtomicBool::new(false))
    }
    pub fn request_with_cancel(&self, operation: Operation, bytes: Arc<[u8]>, cancelled: &AtomicBool) -> Reply {
        if bytes.is_empty() || bytes.len() > super::MAX_INPUT {
            return Err(BackendError::decode("RAW input is empty or exceeds 1 GiB"));
        }
        // Cancellable queue admission, without spawning an unbounded number of IO threads.
        let mut state = loop {
            if cancelled.load(Ordering::Relaxed) {
                return Err(BackendError::decode("RAW decoding cancelled"));
            }
            match self.worker.try_lock() {
                Ok(state) => break state,
                Err(std::sync::TryLockError::Poisoned(e)) => break e.into_inner(),
                Err(std::sync::TryLockError::WouldBlock) => std::thread::sleep(Duration::from_millis(20)),
            }
        };
        if state.is_none() {
            *state = Some(spawn(&self.path)?);
        }
        let (tx, rx) = mpsc::channel();
        let worker = state.as_ref().ok_or_else(|| BackendError::decode("RAW worker unavailable"))?;
        if worker.tasks.send(Task { operation, bytes, reply: tx }).is_err() {
            *state = None;
            return Err(BackendError::decode("RAW worker stopped; retry explicitly"));
        }
        let start = Instant::now();
        loop {
            if cancelled.load(Ordering::Relaxed) || start.elapsed() >= self.timeout {
                *state = None;
                return Err(BackendError::decode(if cancelled.load(Ordering::Relaxed) {
                    "RAW decoding cancelled"
                } else {
                    "Core Image RAW decoding exceeded 120 seconds; worker restarted on next request"
                }));
            }
            match rx.recv_timeout(Duration::from_millis(20)) {
                Ok(result) => {
                    if result.as_ref().is_err_and(|e| e.kind == super::ErrorKind::Protocol) {
                        *state = None;
                    }
                    return result;
                }
                Err(mpsc::RecvTimeoutError::Timeout) => (),
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    *state = None;
                    return Err(BackendError::decode("RAW worker exited; retry explicitly"));
                }
            }
        }
    }
}
fn spawn(path: &Path) -> Result<Worker, BackendError> {
    let mut child = Command::new(path).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn().map_err(|e| {
        BackendError::decode(format!("Cannot start Core Image RAW worker {}: {e}. Install the matching helper alongside LightCraft.", path.display()))
    })?;
    let stdin = child.stdin.take();
    let stdout = child.stdout.take();
    let child = Arc::new(Mutex::new(child));
    let (tx, rx) = mpsc::channel::<Task>();
    let worker = Worker { child, tasks: tx };
    let (Some(mut stdin), Some(mut stdout)) = (stdin, stdout) else {
        return Err(BackendError::decode("RAW worker pipes unavailable"));
    };
    std::thread::Builder::new()
        .name("raw-worker-io".into())
        .spawn(move || {
            while let Ok(task) = rx.recv() {
                let result = write_request(&mut stdin, task.operation, &task.bytes).and_then(|()| read_response(&mut stdout, task.operation));
                let failed = result.is_err();
                let reply = result.unwrap_or_else(|message| {
                    Err(BackendError { kind: super::ErrorKind::Protocol, message: format!("Core Image worker failed: {message}") })
                });
                let _ = task.reply.send(reply);
                if failed {
                    break;
                }
            }
        })
        .map_err(|e| BackendError::decode(e.to_string()))?;
    Ok(worker)
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    #[test]
    fn missing_or_crashing_helper_returns_error_and_allows_next_request() {
        for path in ["/nonexistent/lightcraft-raw-worker", "/usr/bin/false"] {
            let client = Client::new(path.into());
            for _ in 0..2 {
                assert!(client.request(Operation::Probe, Arc::from(&b"input"[..])).is_err());
            }
        }
    }
    #[test]
    fn hung_worker_is_killed_and_the_next_request_can_start() {
        use std::os::unix::fs::PermissionsExt;
        let path = std::env::temp_dir().join(format!("lightcraft-worker-hang-{}", std::process::id()));
        std::fs::write(&path, "#!/bin/sh\nexec sleep 30\n").unwrap();
        std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o700)).unwrap();
        let mut client = Client::new(path.clone());
        client.timeout = Duration::from_millis(100);
        let began = Instant::now();
        for _ in 0..2 {
            assert!(client.request(Operation::Probe, Arc::from(&b"input"[..])).is_err());
        }
        assert!(began.elapsed() < Duration::from_secs(5));
        client.timeout = Duration::from_secs(30);
        let cancelled = AtomicBool::new(false);
        std::thread::scope(|scope| {
            scope.spawn(|| {
                std::thread::sleep(Duration::from_millis(100));
                cancelled.store(true, Ordering::Relaxed);
            });
            let start = Instant::now();
            let error = client.request_with_cancel(Operation::Decode, Arc::from(&b"input"[..]), &cancelled).unwrap_err();
            assert!(error.message.contains("cancelled"));
            assert!(start.elapsed() < Duration::from_secs(5));
        });
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn timeout_and_cancellation_are_bounded() {
        // cat echoes our request, which cannot be decoded as a response; this exercises bad IPC.
        let client = Client::new("/bin/cat".into());
        assert!(client.request(Operation::Probe, Arc::from(&b"input"[..])).is_err());
        let cancelled = AtomicBool::new(true);
        assert!(client.request_with_cancel(Operation::Probe, Arc::from(&b"input"[..]), &cancelled).is_err());
    }
}
