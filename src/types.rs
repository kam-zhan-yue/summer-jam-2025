use std::cmp::Ordering;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Tool {
    #[default]
    None,
    Toilet,
    Underwear,
    Lighter,
}

impl PartialOrd for Tool {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Tool::*;
        match (self, other) {
            (None, _) | (_, None) => Some(Ordering::Equal),
            (Toilet, Lighter) => Some(Ordering::Greater),
            (Lighter, Toilet) => Some(Ordering::Less),
            (Lighter, Underwear) => Some(Ordering::Greater),
            (Underwear, Lighter) => Some(Ordering::Less),
            (Underwear, Toilet) => Some(Ordering::Greater),
            (Toilet, Underwear) => Some(Ordering::Less),
            (a, b) => a.partial_cmp(b),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Location {
    #[default]
    None,
    Library,
    Classroom,
    Gymnasium,
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Location::*;
        match (self, other) {
            (None, _) | (_, None) => Some(Ordering::Equal),
            (Library, Gymnasium) => Some(Ordering::Greater),
            (Gymnasium, Library) => Some(Ordering::Less),
            (Gymnasium, Classroom) => Some(Ordering::Greater),
            (Classroom, Gymnasium) => Some(Ordering::Less),
            (Classroom, Library) => Some(Ordering::Greater),
            (Library, Classroom) => Some(Ordering::Less),
            (a, b) => a.partial_cmp(b),
        }
    }
}
