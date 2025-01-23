use rand::Rng;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Player {
    #[default]
    One,
    Two,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone, PartialOrd)]
pub enum Choice {
    #[default]
    None,
    Action(Action),
    Element(Element),
}

impl fmt::Display for Choice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choice::None => write!(f, "None"),
            Choice::Action(tool) => write!(f, "{}", tool),
            Choice::Element(location) => write!(f, "{}", location),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, PartialOrd)]
pub enum Action {
    #[default]
    Toilet,
    Underwear,
    Hand,
}

impl Action {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Action::Toilet,
            1 => Action::Underwear,
            _ => Action::Hand,
        }
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Toilet => write!(f, "Toilet"),
            Action::Underwear => write!(f, "Underwear"),
            Action::Hand => write!(f, "Hand"),
        }
    }
}

impl Ord for Action {
    fn cmp(&self, other: &Self) -> Ordering {
        use Action::*;
        match (self, other) {
            (Toilet, Toilet) => Ordering::Equal,
            (Underwear, Underwear) => Ordering::Equal,
            (Hand, Hand) => Ordering::Equal,
            (Toilet, Hand) => Ordering::Greater,
            (Hand, Toilet) => Ordering::Less,
            (Hand, Underwear) => Ordering::Greater,
            (Underwear, Hand) => Ordering::Less,
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

impl Element {
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..3) {
            0 => Element::Fire,
            1 => Element::Water,
            _ => Element::Grass,
        }
    }
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

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Outcome {
    PlayerOne,
    PlayerTwo,
    Draw,
}
