// TODO there should be a cleaner way to do this
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PriorityData<T> {
    pub first: Option<T>,
    pub second: Option<T>,
    pub third: Option<T>,
}

impl<T> PriorityData<T> {
    pub fn highest(self) -> Option<T> {
        if self.first.is_some() {
            return self.first;
        }
        if self.second.is_some() {
            return self.second;
        }
        self.third
    }
}

impl<T> Default for PriorityData<T> {
    fn default() -> Self {
        Self {
            first: None,
            second: None,
            third: None,
        }
    }
}
