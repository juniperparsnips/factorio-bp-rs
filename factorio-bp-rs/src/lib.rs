#![warn(missing_docs)]

//! Library for analyzing the performance of simple blueprints

use blueprint::Position;

/// Structures for decoding blueprint strings. See https://wiki.factorio.com/Blueprint_string_format for more
pub mod blueprint;

#[cfg(feature = "cli")]
pub mod cli;

pub struct Size {
    pub w: usize,
    pub h: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Item {
    name: String,
    stack_size: usize,
}

impl Item {
    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn stack_size(&self) -> usize {
        self.stack_size
    }
}

pub struct Fluid {
    name: String,
    stack_size: usize,
}

impl Fluid {
    pub fn name(&self) -> &String {
        &self.name
    }
}

/// Any structure on the factorio world
pub struct Entity {
    name: String,
    position: Position,
    size: Size,
}

/// Used as a common trait between 'Item' and 'Fluid'
pub trait RecipeIO {}

pub trait Recipe {
    fn can_use_productivity(&self) -> bool;
}
