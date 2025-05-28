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
    pub fn add(&mut self, clip: &Clip) -> bool {
        match self.clip.as_mut() {
            Some(current) => {
                if clip.timestamp > current.timestamp && clip.text != current.text {
                    *current = clip.clone();
                    true
                } else {
                    false
                }
            }
            None => {
                self.clip = Some(clip.clone());
                true
            }
        }
    }
}
