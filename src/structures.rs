pub enum Direction {
    HORIZONTAL,
    VERTICAL,
}

pub enum UserType {
    BOT,
    USER,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum FieldStatus {
    EMPTY,
    FAIL,
    HIT,
    SHIP,
}

// TODO: Is their another approach for specifying the ship name lifetime?
#[derive(Debug)]
pub struct Ship {
    pub length: usize,
    pub name: &'static str,
}

// TODO: How can i reference the ship? Or is this correct?
#[derive(Debug)]
pub struct PlacedShip {
    pub ship: &'static Ship,
    pub x: usize,
    pub y: usize,
    pub hits: usize,
}
