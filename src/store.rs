use crate::Clip;

#[derive(Default)]
pub struct Store {
    clip: Option<Clip>,
}

impl Store {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn current(&self) -> Option<Clip> {
        self.clip.clone()
    }

    #[must_use]
    pub fn add(&mut self, clip: Clip) -> bool {
        match self.clip.as_mut() {
            Some(current) => {
                if clip.timestamp > current.timestamp {
                    *current = clip;
                    true
                } else {
                    false
                }
            }
            None => {
                self.clip = Some(clip);
                true
            }
        }
    }
}
