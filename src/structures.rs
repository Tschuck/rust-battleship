pub enum Direction {
    HORIZONTAL,
    VERTICAL,
}

#[derive(PartialEq)]
pub enum UserType {
    BOT,
    USER,
}

#[derive(PartialEq, Clone, Copy)]
pub enum FieldStatus {
    EMPTY,
    FAIL,
    HIT,
    SHIP,
}

pub struct Ship {
    pub length: usize,
    pub name: &'static str,
}

pub struct PlacedShip {
    pub ship: &'static Ship,
    pub x: usize,
    pub y: usize,
    pub hits: usize,
}
