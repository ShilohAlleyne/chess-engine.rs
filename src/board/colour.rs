use std::ops::Index;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour<T> {
    White(T),
    Black(T),
}

// === Index trait for easy composition ===
// Implement Index for slices
impl<T> Index<&Colour<()>> for [T] {
    type Output = T;

    fn index(&self, colour: &Colour<()>) -> &Self::Output {
        match colour {
            Colour::White(()) => &self[0],
            Colour::Black(()) => &self[1],
        }
    }
}

impl<T> Index<Colour<()>> for [T] {
    type Output = T;

    fn index(&self, colour: Colour<()>) -> &Self::Output {
        match colour {
            Colour::White(()) => &self[0],
            Colour::Black(()) => &self[1],
        }
    }
}

// Indexing Vec<T> with &Colour<()>
impl<T> Index<&Colour<()>> for Vec<T> {
    type Output = T;

    fn index(&self, colour: &Colour<()>) -> &Self::Output {
        match colour {
            Colour::White(()) => &self[0],
            Colour::Black(()) => &self[1],
        }
    }
}

// Implement Index for Vec<T>
impl<T> Index<Colour<()>> for Vec<T> {
    type Output = T;

    fn index(&self, colour: Colour<()>) -> &Self::Output {
        match colour {
            Colour::White(()) => &self[0],
            Colour::Black(()) => &self[1],
        }
    }
}

impl Colour<()> {
    pub fn opp(&self) -> Self {
        match self {
            Colour::Black(()) => Colour::White(()),
            Colour::White(()) => Colour::Black(()),
        }
    }
}
