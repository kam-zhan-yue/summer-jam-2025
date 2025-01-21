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
    Location(Element),
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
pub enum Element {
    #[default]
    Fire,
    Water,
    Grass,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Element::Fire => write!(f, "Fire"),
            Element::Water => write!(f, "Water"),
            Element::Grass => write!(f, "Grass"),
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        use Element::*;
        match (self, other) {
            (Fire, Fire) => Ordering::Equal,
            (Water, Water) => Ordering::Equal,
            (Grass, Grass) => Ordering::Equal,
            (Fire, Water) => Ordering::Less,
            (Fire, Grass) => Ordering::Greater,
            (Water, Fire) => Ordering::Greater,
            (Water, Grass) => Ordering::Less,
            (Grass, Fire) => Ordering::Less,
            (Grass, Water) => Ordering::Greater,
        }
    }
}

#[derive(Debug)]
pub enum Result {
    Swirlie,
    Whirlie,
    Wedgie,
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Outcome {
    PlayerOne,
    PlayerTwo,
    Draw,
}
