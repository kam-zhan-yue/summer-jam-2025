use std::cmp::Ordering;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub enum Tool {
    #[default]
    None,
    Toilet,
    Underwear,
    Lighter,
}

impl Ord for Tool {
    fn cmp(&self, other: &Self) -> Ordering {
        use Tool::*;
        match (self, other) {
            (None, None) => Ordering::Equal,
            (Toilet, Toilet) => Ordering::Equal,
            (Underwear, Underwear) => Ordering::Equal,
            (Lighter, Lighter) => Ordering::Equal,
            (Toilet, None) => Ordering::Greater,
            (Lighter, None) => Ordering::Greater,
            (Underwear, None) => Ordering::Greater,
            (None, Toilet) => Ordering::Less,
            (None, Lighter) => Ordering::Less,
            (None, Underwear) => Ordering::Less,
            (Toilet, Lighter) => Ordering::Greater,
            (Lighter, Toilet) => Ordering::Less,
            (Lighter, Underwear) => Ordering::Greater,
            (Underwear, Lighter) => Ordering::Less,
            (Underwear, Toilet) => Ordering::Greater,
            (Toilet, Underwear) => Ordering::Less,
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub enum Location {
    #[default]
    None,
    Library,
    Classroom,
    Gymnasium,
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        use Location::*;
        match (self, other) {
            (None, None) => Ordering::Equal,
            (Library, Library) => Ordering::Equal,
            (Classroom, Classroom) => Ordering::Equal,
            (Gymnasium, Gymnasium) => Ordering::Equal,
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
            (Library, Classroom) => Ordering::Less,
            (Library, Gymnasium) => Ordering::Greater,
            (Classroom, Library) => Ordering::Greater,
            (Classroom, Gymnasium) => Ordering::Less,
            (Gymnasium, Library) => Ordering::Less,
            (Gymnasium, Classroom) => Ordering::Greater,
        }
    }
}
