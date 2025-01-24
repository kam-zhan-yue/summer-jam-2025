use rand::Rng;
use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum Player {
    #[default]
    One,
    Two,
}

#[derive(Debug, Default, Eq, PartialEq, Copy, Clone)]
pub enum Choice {
    #[default]
    None,
    Action(Action),
    Element(Element),
}

impl PartialOrd for Choice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Choice::None, Choice::None) => Some(Ordering::Equal),
            (Choice::None, _) => Some(Ordering::Less),
            (_, Choice::None) => Some(Ordering::Greater),
            (Choice::Action(a1), Choice::Action(a2)) => a1.partial_cmp(a2),
            (Choice::Element(e1), Choice::Element(e2)) => e1.partial_cmp(e2),
            _ => Some(Ordering::Equal),
        }
    }
}

impl Choice {
    pub fn get_complement(choice: &Choice) -> Self {
        match *choice {
            Choice::Action(Action::Hand) => Choice::Element(Element::Fire),
            Choice::Action(Action::Toilet) => Choice::Element(Element::Water),
            Choice::Action(Action::Underwear) => Choice::Element(Element::Grass),
            Choice::Element(Element::Fire) => Choice::Action(Action::Hand),
            Choice::Element(Element::Water) => Choice::Action(Action::Toilet),
            Choice::Element(Element::Grass) => Choice::Action(Action::Underwear),
            _ => Choice::None,
        }
    }
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

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub enum Action {
    #[default]
    Toilet,
    Underwear,
    Hand,
}

impl Action {
    pub fn weighted(element: &Choice) -> Self {
        let mut rng = rand::thread_rng();
        let roll = rng.gen_range(0..4);
        match element {
            Choice::Element(Element::Water) => match roll {
                0 => Action::Toilet,
                1 => Action::Toilet,
                2 => Action::Underwear,
                _ => Action::Hand,
            },
            Choice::Element(Element::Fire) => match roll {
                0 => Action::Hand,
                1 => Action::Hand,
                2 => Action::Underwear,
                _ => Action::Toilet,
            },
            Choice::Element(Element::Grass) => match roll {
                0 => Action::Underwear,
                1 => Action::Underwear,
                2 => Action::Toilet,
                _ => Action::Hand,
            },
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

impl PartialOrd for Action {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Action::*;
        match (self, other) {
            (Toilet, Toilet) => Some(Ordering::Equal),
            (Underwear, Underwear) => Some(Ordering::Equal),
            (Hand, Hand) => Some(Ordering::Equal),
            (Toilet, Hand) => Some(Ordering::Greater),
            (Hand, Toilet) => Some(Ordering::Less),
            (Hand, Underwear) => Some(Ordering::Greater),
            (Underwear, Hand) => Some(Ordering::Less),
            (Underwear, Toilet) => Some(Ordering::Greater),
            (Toilet, Underwear) => Some(Ordering::Less),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
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

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use Element::*;
        match (self, other) {
            (Fire, Fire) => Some(Ordering::Equal),
            (Water, Water) => Some(Ordering::Equal),
            (Grass, Grass) => Some(Ordering::Equal),
            (Fire, Water) => Some(Ordering::Less),
            (Fire, Grass) => Some(Ordering::Greater),
            (Water, Fire) => Some(Ordering::Greater),
            (Water, Grass) => Some(Ordering::Less),
            (Grass, Fire) => Some(Ordering::Less),
            (Grass, Water) => Some(Ordering::Greater),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
pub enum Outcome {
    PlayerOne,
    PlayerTwo,
    Draw,
}
