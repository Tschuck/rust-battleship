pub enum Direction {
    HORIZONTAL,
    VERTICAL,
}

pub enum UserType {
    USER,
    BOT,
}

#[derive(Debug)]
pub enum FieldStatus {
    EMPTY,
    SHIP,
    HIT,
    FAIL,
}

// TODO: Is their another approach for specifying the ship name lifetime?
#[derive(Debug)]
pub struct Ship<'a> {
    pub length: u8,
    pub name: &'a str,
}

// TODO: How can i reference the ship? Or is this correct?
#[derive(Debug)]
pub struct PlacedShip<'a> {
    pub ship: Ship<'a>,
    pub x: usize,
    pub y: usize,
}
