// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! All native pointers and framework calls are contained here. No Objective-C object crosses IPC.
use lightcraft_raw_backend::{BackendError, Decoded, Info, Operation, RECIPE, pixel_count};
use objc2::{
    AnyThread, ClassType, msg_send,
    rc::{Retained, autoreleasepool},
    runtime::{AnyObject, NSObjectProtocol},
    sel,
};
use objc2_core_foundation::CGPoint;
use objc2_core_graphics::{CGColorSpace, kCGColorSpaceExtendedLinearITUR_2020};
use objc2_core_image::{
    CIContext, CIContextOption, CIRAWFilter, CIRenderDestination, kCIContextCacheIntermediates, kCIContextWorkingFormat, kCIFormatRGBAf,
};
use objc2_foundation::{NSData, NSDictionary, NSNumber, NSProcessInfo};
use std::ptr::NonNull;

pub struct Decoder {
    context: Retained<CIContext>,
    os_build: String,
}
impl Decoder {
    pub fn new() -> Result<Self, BackendError> {
        let process = NSProcessInfo::processInfo();
        if process.operatingSystemVersion().majorVersion < 12 {
            return Err(BackendError::unsupported("Core Image RAW requires macOS 12 or newer"));
        }
        // SAFETY: documented CIContext option with an NSNumber carrying a supported pixel format.
        // The context is confined to this worker thread for its entire lifetime.
        let context = unsafe {
            let format = NSNumber::new_i32(kCIFormatRGBAf);
            let cache = NSNumber::new_bool(false);
            let options =
                NSDictionary::<CIContextOption, AnyObject>::from_slices(&[kCIContextWorkingFormat, kCIContextCacheIntermediates], &[&format, &cache]);
            let context: Option<Retained<CIContext>> = msg_send![CIContext::class(), contextWithOptions: &*options];
            context.ok_or_else(|| BackendError::decode("Core Image context unavailable (check graphics/session access)"))?
        };
        Ok(Self { context, os_build: process.operatingSystemVersionString().to_string() })
    }
    pub fn process(&self, operation: Operation, bytes: &[u8]) -> Result<Decoded, BackendError> {
        autoreleasepool(|_| self.process_inner(operation, bytes))
    }
    fn process_inner(&self, operation: Operation, bytes: &[u8]) -> Result<Decoded, BackendError> {
        let data = NSData::with_bytes(bytes);
        // SAFETY: NSData owns a copy; filter/context remain on this single worker thread. All scalar
        // settings are inside documented ranges; newer selectors are checked before sending them.
        // The retained graph lives until its render task has completed. Native failures stay in
        // the supervised process (the parent has an independent timeout and validates the reply).
        unsafe {
            let filter = CIRAWFilter::filterWithImageData_identifierHint(&data, None)
                .ok_or_else(|| BackendError::unsupported("Apple Core Image does not support this RAW file or its camera decoder is unavailable"))?;
            filter.setDraftModeEnabled(false);
            filter.setScaleFactor(1.0);
            filter.setExposure(0.0);
            filter.setBaselineExposure(0.0);
            filter.setShadowBias(0.0);
            filter.setBoostAmount(0.0);
            filter.setBoostShadowAmount(0.0);
            filter.setGamutMappingEnabled(false);
            filter.setExtendedDynamicRangeAmount(1.0);
            if filter.isLensCorrectionSupported() {
                filter.setLensCorrectionEnabled(false);
            }
            if filter.isLuminanceNoiseReductionSupported() {
                filter.setLuminanceNoiseReductionAmount(0.0);
            }
            if filter.isColorNoiseReductionSupported() {
                filter.setColorNoiseReductionAmount(0.0);
            }
            if filter.isSharpnessSupported() {
                filter.setSharpnessAmount(0.0);
            }
            if filter.isContrastSupported() {
                filter.setContrastAmount(0.0);
            }
            if filter.isDetailSupported() {
                filter.setDetailAmount(0.0);
            }
            if filter.isMoireReductionSupported() {
                filter.setMoireReductionAmount(0.0);
            }
            if filter.isLocalToneMapSupported() {
                filter.setLocalToneMapAmount(0.0);
            }
            if filter.respondsToSelector(sel!(isHighlightRecoverySupported)) && filter.isHighlightRecoverySupported() {
                filter.setHighlightRecoveryEnabled(false);
            }
            let image = filter.outputImage().ok_or_else(|| BackendError::decode("Core Image RAW filter produced no image"))?;
            let extent = image.extent();
            let dimension = |v: f64| -> Result<usize, BackendError> {
                if !v.is_finite() || !(1.0..=65536.0).contains(&v) || v.fract() != 0.0 {
                    return Err(BackendError::decode(format!("Invalid Core Image RAW extent ({v}); this DNG/RAW variant may be unsupported")));
                }
                Ok(v as usize)
            };
            let width = dimension(extent.size.width)?;
            let height = dimension(extent.size.height)?;
            if !extent.origin.x.is_finite() || !extent.origin.y.is_finite() {
                return Err(BackendError::decode("Invalid Core Image RAW origin"));
            }
            let count = pixel_count(width, height).map_err(BackendError::decode)?;
            let xy = filter.neutralChromaticity();
            let info = Info {
                width,
                height,
                neutral_xy: [xy.x, xy.y],
                decoder: filter.decoderVersion().to_string(),
                os_build: self.os_build.clone(),
                recipe: RECIPE.into(),
            };
            info.validate().map_err(BackendError::decode)?;
            if operation == Operation::Probe {
                return Ok(Decoded { info, image: None });
            }
            let space = CGColorSpace::with_name(Some(kCGColorSpaceExtendedLinearITUR_2020))
                .ok_or_else(|| BackendError::decode("Extended linear Rec.2020 colour space unavailable"))?;
            let mut rgba = Vec::<[f32; 4]>::new();
            rgba.try_reserve_exact(count).map_err(|e| BackendError::decode(e.to_string()))?;
            rgba.resize(count, [f32::NAN; 4]);
            let pointer = NonNull::new(rgba.as_mut_ptr().cast()).ok_or_else(|| BackendError::decode("Empty RAW bitmap"))?;
            let stride = width.checked_mul(16).ok_or_else(|| BackendError::decode("RAW stride overflow"))?;
            // SAFETY: allocation is exactly height*stride bytes, correctly aligned, exclusive and
            // stable until waitUntilCompleted returns; no Rust access while Core Image writes it.
            let destination = CIRenderDestination::initWithBitmapData_width_height_bytesPerRow_format(
                CIRenderDestination::alloc(),
                pointer,
                width,
                height,
                stride,
                kCIFormatRGBAf,
            );
            destination.setColorSpace(Some(&space));
            destination.setFlipped(true);
            let task = self
                .context
                .startTaskToRender_fromRect_toDestination_atPoint_error(&image, extent, &destination, CGPoint::new(0.0, 0.0))
                .map_err(|e| BackendError::decode(format!("Core Image render failed: {e}")))?;
            task.waitUntilCompletedAndReturnError().map_err(|e| BackendError::decode(format!("Core Image render failed: {e}")))?;
            drop(task);
            drop(destination);
            let mut rgb = Vec::new();
            rgb.try_reserve_exact(count).map_err(|e| BackendError::decode(e.to_string()))?;
            for [r, g, b, a] in rgba {
                if ![r, g, b, a].iter().all(|v| v.is_finite()) || a <= 0.0 {
                    return Err(BackendError::decode("Core Image returned incomplete or nonfinite RAW pixels"));
                }
                rgb.push([r / a, g / a, b / a]);
            }
            Ok(Decoded { info, image: Some(lightcraft_raster::Rgb32f { width, height, data: rgb }) })
        }
    }
}
