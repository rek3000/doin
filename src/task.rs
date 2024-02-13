use std::fs;
use crate::types::Item;
use ratatui::{prelude::*, widgets::*};

impl Item {
    pub fn new() {}
    fn create(&self, frame: &mut Frame) {}
    fn edit(&self, frame: &mut Frame) {}
    fn delete(&self, frame: &mut Frame) {}
    fn save(&self, frame: &mut Frame) {}
}
