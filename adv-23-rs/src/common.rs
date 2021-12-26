#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(crate) enum Color {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Color {
    pub(crate) fn get_idx(&self) -> usize {
        match self {
            Color::Amber => 0,
            Color::Bronze => 1,
            Color::Copper => 2,
            Color::Desert => 3
        }
    }

    pub(crate) fn from_idx(idx: usize) -> Self {
        [Color::Amber, Color::Bronze, Color::Copper, Color::Desert][idx]
    }

    pub(crate) fn get_cost(&self) -> u64 {
        match self {
            Color::Amber => 1,
            Color::Bronze => 10,
            Color::Copper => 100,
            Color::Desert => 1000
        }
    }

    pub(crate) fn get_letter(&self) -> char {
        match self {
            Color::Amber => 'A',
            Color::Bronze => 'B',
            Color::Copper => 'C',
            Color::Desert => 'D'
        }
    }
}