use rkit::{math::Vec2, prelude::*};

#[derive(Component, Clone, Copy, Deref, Debug)]
pub struct Pos(pub Vec2);
