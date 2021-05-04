use crate::structures::Ship;

pub const GRID_COUNT: usize = 8;

pub const SHIPS: [Ship; 5] = [
    Ship { name: "Carrier", length: 5 },
    Ship { name: "Battleship", length: 4 },
    Ship { name: "Cruiser", length: 3 },
    Ship { name: "Submarine", length: 3 },
    Ship { name: "Destroyer", length: 2 },
];
