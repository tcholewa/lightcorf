// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! Original ISO-inspired starting points, shared by commands and the edit panel.
use crate::Grain;

#[derive(Clone, Copy, Debug)]
pub struct GrainPreset {
    pub iso: u32,
    pub amount: f64,
    pub size: f64,
    pub roughness: f64,
}

pub const PRESETS: [GrainPreset; 3] = [
    GrainPreset { iso: 400, amount: 20.0, size: 20.0, roughness: 40.0 },
    GrainPreset { iso: 800, amount: 35.0, size: 35.0, roughness: 55.0 },
    GrainPreset { iso: 1600, amount: 50.0, size: 50.0, roughness: 70.0 },
];

impl GrainPreset {
    /// Applying a profile deliberately preserves the renderer and repeatable seed.
    pub fn apply(self, grain: &mut Grain) {
        grain.amount = self.amount;
        grain.size = self.size;
        grain.roughness = self.roughness;
    }

    pub fn matches(self, grain: &Grain) -> bool {
        grain.amount == self.amount && grain.size == self.size && grain.roughness == self.roughness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{DevelopSettings, SettingsGroup, apply_partial, extract_groups};
    use serde_json::json;

    #[test]
    fn profiles_keep_mode_and_seed_and_custom_values_do_not_match() {
        let mut g = Grain { advanced: true, seed: 42, ..Default::default() };
        for p in PRESETS {
            p.apply(&mut g);
            assert!(p.matches(&g));
            assert!(g.advanced);
            assert_eq!(g.seed, 42);
            g.amount += 1.0;
            assert!(!p.matches(&g));
        }
    }

    #[test]
    fn legacy_json_and_copying_off_have_distinct_requirements() {
        let old = json!({"grain":{"amount":23.0,"size":31.0,"roughness":62.0,"seed":9}});
        let s = DevelopSettings::from_json(&old).unwrap();
        assert!(!s.grain.advanced);
        assert_eq!(s.to_json()["grain"], old["grain"]);
        let patch = extract_groups(&s, &[SettingsGroup::Grain]);
        assert_eq!(patch["grain"]["advanced"], false);
        let mut target = s.clone();
        target.grain.advanced = true;
        assert!(!apply_partial(&target, &patch, 1.0).grain.advanced);
        assert!(apply_partial(&target, &old, 1.0).grain.advanced, "old partials preserve mode");
        for amount in [0.0, 0.25, 0.5, 1.0, 2.0] {
            let result = apply_partial(&target, &patch, amount);
            assert_eq!(result.grain.advanced, amount < 0.5);
            let on = apply_partial(&s, &json!({"grain":{"advanced":true,"amount":43.0}}), amount);
            assert_eq!(on.grain.advanced, amount >= 0.5);
            assert_eq!(on.grain.amount, 23.0 + 20.0 * amount);
        }
        target.reset_section(crate::Section::Grain);
        assert!(!target.grain.advanced);
        assert_eq!(target.grain.seed, 9);
    }
}
