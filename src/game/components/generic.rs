use crate::*;

#[derive(Component, Default, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32
}
impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "({}, {})", self.x, self.y);
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct ServerID (pub u32);

impl fmt::Display for ServerID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.0);
    }
}