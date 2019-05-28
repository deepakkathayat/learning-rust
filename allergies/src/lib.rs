use std::slice::Iter;
pub struct Allergies {
    score: u32,
}

use Allergen::*;
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    pub fn iterator() -> Iter<'static, Allergen> {
        [
            Eggs,
            Peanuts,
            Shellfish,
            Strawberries,
            Tomatoes,
            Chocolate,
            Pollen,
            Cats,
        ]
        .into_iter()
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        (self.score & *allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::iterator()
            .cloned()
            .filter(|&allergen| self.is_allergic_to(&allergen))
            .collect()
    }
}
