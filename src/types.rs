use std::cmp::Ordering;

#[derive(Debug)]
pub enum Player {
    One,
    Two,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, PartialOrd)]
pub enum Choice {
    #[default]
    None,
    Tool(Tool),
    Location(Location),
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub enum Tool {
    #[default]
    Toilet,
    Underwear,
    Lighter,
}

impl Ord for Tool {
    fn cmp(&self, other: &Self) -> Ordering {
        use Tool::*;
        match (self, other) {
            (Toilet, Toilet) => Ordering::Equal,
            (Underwear, Underwear) => Ordering::Equal,
            (Lighter, Lighter) => Ordering::Equal,
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
    Library,
    Classroom,
    Gymnasium,
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        use Location::*;
        match (self, other) {
            (Library, Library) => Ordering::Equal,
            (Classroom, Classroom) => Ordering::Equal,
            (Gymnasium, Gymnasium) => Ordering::Equal,
            (Library, Classroom) => Ordering::Less,
            (Library, Gymnasium) => Ordering::Greater,
            (Classroom, Library) => Ordering::Greater,
            (Classroom, Gymnasium) => Ordering::Less,
            (Gymnasium, Library) => Ordering::Less,
            (Gymnasium, Classroom) => Ordering::Greater,
        }
    }
}
