use std::fmt::Display;

use arrow::datatypes::{Field, Schema};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Column {
    name: String
}

impl Column {
    pub fn new(name: impl Into<String>) -> Self {
        Self {name : name.into() }
    }
}