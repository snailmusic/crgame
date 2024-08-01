use bevy::prelude::*;
use serde::{Serialize, Deserialize};
use crate::Actions;

#[derive(Asset, TypePath, Debug, Serialize, Deserialize)]
pub struct Level {
    pub title: String,
    pub artist:  String,
    pub creator: String,
    pub data: Box<[Note]>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Note {
    S(Actions, u32),
    L(Actions, u32, u32)
}