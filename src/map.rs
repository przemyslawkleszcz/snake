use specs::World;

use crate::{
    components::Position,
    entities::{create_floor, create_wall},
};

const MAP: &str = "W W W W W W W W W W W W W W W W W W W W W W W W W W W W W W W W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . W
    W W W W W W W W W W W W W W W W W W W W W W W W W W W W W W W W";

pub fn initialize_level(world: &mut World) {
    let rows: Vec<&str> = MAP.split("\n").map(|x| x.trim()).collect();
    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(" ").collect();
        for (x, column) in columns.iter().enumerate() {
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0,
            };

            match *column {
                "." => create_floor(world, position),
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
