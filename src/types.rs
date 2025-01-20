use std::cmp::Ordering;
use std::fmt;

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

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::None => write!(f, "None"),
            Choice::Tool(tool) => write!(f, "Tool: {}", tool),
            Choice::Location(location) => write!(f, "Location: {}", location),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub enum Tool {
    #[default]
    Toilet,
    Underwear,
    Lighter,
}

impl fmt::Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tool::Toilet => write!(f, "Toilet"),
            Tool::Underwear => write!(f, "Underwear"),
            Tool::Lighter => write!(f, "Lighter"),
        }
    }
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

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Location::Library => write!(f, "Library"),
            Location::Classroom => write!(f, "Classroom"),
            Location::Gymnasium => write!(f, "Gymnasium"),
        }
    }
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
