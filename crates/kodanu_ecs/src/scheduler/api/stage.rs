#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stage {
    TimeUpdate,
    PreInputUpdate,
    Startup,
    PreFixedUpdate,
    FixedUpdate,
    PostFixedUpdate,
    PreUpdate,
    Update,
    LateUpdate,
    PreRender,
    Render,
    PostRender,
    PostInputUpdate,
}

impl Stage {
    pub const COUNT: usize = 13;

    pub fn as_usize(self) -> usize {
        self as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const ALL: [Stage; 13] = [
        Stage::TimeUpdate,
        Stage::PreInputUpdate,
        Stage::Startup,
        Stage::PreFixedUpdate,
        Stage::FixedUpdate,
        Stage::PostFixedUpdate,
        Stage::PreUpdate,
        Stage::Update,
        Stage::LateUpdate,
        Stage::PreRender,
        Stage::Render,
        Stage::PostRender,
        Stage::PostInputUpdate,
    ];

    #[test]
    fn stage_has_expected_index() {
        assert_eq!(Stage::TimeUpdate.as_usize(), 0);
        assert_eq!(Stage::PreInputUpdate.as_usize(), 1);
        assert_eq!(Stage::Startup.as_usize(), 2);
        assert_eq!(Stage::PreFixedUpdate.as_usize(), 3);
        assert_eq!(Stage::FixedUpdate.as_usize(), 4);
        assert_eq!(Stage::PostFixedUpdate.as_usize(), 5);
        assert_eq!(Stage::PreUpdate.as_usize(), 6);
        assert_eq!(Stage::Update.as_usize(), 7);
        assert_eq!(Stage::LateUpdate.as_usize(), 8);
        assert_eq!(Stage::PreRender.as_usize(), 9);
        assert_eq!(Stage::Render.as_usize(), 10);
        assert_eq!(Stage::PostRender.as_usize(), 11);
        assert_eq!(Stage::PostInputUpdate.as_usize(), 12);
    }

    #[test]
    fn count_matches_variant_list() {
        assert_eq!(Stage::COUNT, 13);
        assert_eq!(Stage::COUNT, ALL.len());
    }

    #[test]
    fn indices_are_unique_and_below_count() {
        let mut seen = [false; Stage::COUNT];

        for stage in ALL {
            let index = stage.as_usize();

            assert!(index < Stage::COUNT, "{stage:?} index out of range");
            assert!(!seen[index], "{stage:?} reuses index {index}");

            seen[index] = true;
        }

        assert!(seen.iter().all(|&occupied| occupied));
    }
}
