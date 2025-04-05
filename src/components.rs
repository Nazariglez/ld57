use rkit::math::Vec2;
use rkit::prelude::*;

#[derive(Component, Clone, Copy, Deref, Debug)]
pub struct Pos(pub Vec2);
