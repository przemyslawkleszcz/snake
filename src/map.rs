use crate::components::*;
use crate::entities::*;
use specs::World;

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
    W . . . . . . . . . . . . . . . P . . . . . . . . . . . . . . W
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
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
