use crate::prelude::*;

pub struct RootedTheme {}

impl RootedTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for RootedTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::FloorVar1 => to_cp437(':'), //rubble
            TileType::FloorVar2 => to_cp437(','), //smalfoilage
            TileType::FloorVar3 => to_cp437(';'), //bigfoilage
            TileType::Wall => to_cp437('#'),
            TileType::WallVar1 => to_cp437('⌠'), //tree
            TileType::WallVar2 => to_cp437('%'),   //roots
            TileType::WallVar3 => to_cp437('♣'), //bush
            TileType::Exit => to_cp437('>'),
            TileType::Shelve => to_cp437('█'),
        }
    }

    fn in_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
        match tile_type {
            TileType::Floor => ColorPair::new(
                RGB::from_hex("#86949F").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            TileType::FloorVar1 => ColorPair::new(
                RGB::from_hex("#716E61").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            TileType::FloorVar2 => ColorPair::new(
                RGB::from_hex("#71AA34").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),

            TileType::FloorVar3 => ColorPair::new(
                RGB::from_hex("#215E21").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),

            TileType::Wall => ColorPair::new(
                RGB::from_hex("#4e4a4e").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),

            TileType::WallVar1 => ColorPair::new(
                RGB::from_hex("#215E21").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),

            TileType::WallVar2 => ColorPair::new(
                RGB::from_hex("#462428").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            TileType::WallVar3 => ColorPair::new(
                RGB::from_hex("#814D30").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),

            TileType::Exit => ColorPair::new(
                RGB::from_hex("#D4A798").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            TileType::Shelve => ColorPair::new(
                RGB::from_hex("#4e4a4e").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
        }
    }

    fn out_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
        match tile_type {
            // TileType::Floor => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
            // TileType::FloorVar1 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
            // TileType::FloorVar2 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
            // TileType::FloorVar3 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
            // TileType::Wall => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
            // TileType::WallVar1 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
            // TileType::WallVar2 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
            // TileType::WallVar3 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
            TileType::Exit => ColorPair::new(
                RGB::from_hex("#D4A798").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            _ => ColorPair::new(
                RGB::from_hex("#333366").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
        }
    }
}

pub struct TiledTheme {}

impl TiledTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for TiledTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::FloorVar1 => to_cp437(':'), //rubble
            TileType::FloorVar2 => to_cp437(','), //smalfoilage
            TileType::FloorVar3 => to_cp437(';'), //bigfoilage
            TileType::Wall => to_cp437('#'),
            TileType::WallVar1 => to_cp437('⌠'), //tree
            TileType::WallVar2 => to_cp437('%'),   //roots
            TileType::WallVar3 => to_cp437('♣'), //bush
            TileType::Exit => to_cp437('>'),
            TileType::Shelve => to_cp437('▓'),
        }
    }

    fn in_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
        match tile_type {
            TileType::Floor
            | TileType::FloorVar1
            | TileType::FloorVar2
            | TileType::FloorVar3
            | TileType::Wall
            | TileType::WallVar1
            | TileType::WallVar2
            | TileType::WallVar3
            | TileType::Exit
            | TileType::Shelve => ColorPair::new(WHITE, BLACK),
        }
    }

    fn out_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
        match tile_type {
            TileType::Floor
            | TileType::FloorVar1
            | TileType::FloorVar2
            | TileType::FloorVar3
            | TileType::Wall
            | TileType::WallVar1
            | TileType::WallVar2
            | TileType::WallVar3
            | TileType::Exit
            | TileType::Shelve => ColorPair::new(DARK_GRAY, BLACK),
        }
    }
}

// pub struct DungeonTheme {}

// impl DungeonTheme {
//     pub fn new() -> Box<dyn MapTheme> {
//         Box::new(Self {})
//     }
// }

// impl MapTheme for DungeonTheme {
//     fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
//         match tile_type {
//             TileType::Floor => to_cp437('.'),
//             TileType::FloorVar1 => to_cp437(':'), //rubble
//             TileType::FloorVar2 => to_cp437(','), //smalfoilage
//             TileType::FloorVar3 => to_cp437(';'), //bigfoilage
//             TileType::Wall | TileType::WallVar1 | TileType::WallVar3 => to_cp437('#'),
//             // TileType::WallVar1 => to_cp437('#'), //stony wall
//             TileType::WallVar2 => to_cp437('%'), //roots
//             // TileType::WallVar3 => to_cp437('#'), //
//             TileType::Exit => to_cp437('>'),
//             TileType::Shelve => to_cp437('▓'),

//         }
//     }

//     fn in_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
//         match tile_type {
//             TileType::Floor => ColorPair::new(RGB::named(SADDLEBROWN), RGB::named(BLACK)),
//             TileType::FloorVar1 => ColorPair::new(RGB::named(PERU), RGB::named(BLACK)),
//             TileType::FloorVar2 => ColorPair::new(RGB::named(DARKOLIVEGREEN), RGB::named(BLACK)),
//             TileType::FloorVar3 => ColorPair::new(RGB::named(SIENNA), RGB::named(BLACK)),
//             TileType::Wall | TileType::WallVar3 => {
//                 ColorPair::new(RGB::from_u8(43, 29, 14), RGB::named(BLACK))
//             }
//             TileType::WallVar1 => ColorPair::new(RGB::named(DIMGRAY), RGB::named(BLACK)),
//             TileType::WallVar2 => ColorPair::new(RGB::named(MAROON), RGB::named(BLACK)),
//             // TileType::WallVar3 => ColorPair::new(RGB::from_u8(43, 29, 14), RGB::named(BLACK)),
//             TileType::Exit => ColorPair::new(RGB::named(RED), RGB::named(BLACK)),
//         }
//     }

//     fn out_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
//         match tile_type {
//             // TileType::Floor => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::FloorVar1 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::FloorVar2 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::FloorVar3 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::Wall => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::WallVar1 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::WallVar2 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::WallVar3 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             TileType::Exit => ColorPair::new(RGB::named(DARKRED), RGB::named(BLACK)),
//             _ => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//         }
//     }
// }

// pub struct ForestTheme {}

// impl MapTheme for ForestTheme {
//     fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
//         match tile_type {
//             TileType::Floor => to_cp437('.'),
//             TileType::FloorVar1 => to_cp437(':'),
//             TileType::FloorVar2 => to_cp437(','),
//             TileType::FloorVar3 => to_cp437(';'),
//             TileType::Wall | TileType::WallVar3 => to_cp437('‼'),
//             TileType::WallVar1 => to_cp437('♣'),
//             TileType::WallVar2 => to_cp437('♠'),
//             // TileType::WallVar3 => to_cp437('‼'),
//             TileType::Exit => to_cp437('>'),
//         }
//     }

//     fn in_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
//         match tile_type {
//             TileType::Floor => ColorPair::new(RGB::named(SADDLEBROWN), RGB::named(BLACK)),
//             TileType::FloorVar1 => ColorPair::new(RGB::named(PERU), RGB::named(BLACK)),
//             TileType::FloorVar2 => ColorPair::new(RGB::named(DARKOLIVEGREEN), RGB::named(BLACK)),
//             TileType::FloorVar3 => ColorPair::new(RGB::named(SIENNA), RGB::named(BLACK)),
//             TileType::Wall | TileType::WallVar3 => {
//                 ColorPair::new(RGB::from_u8(43, 29, 14), RGB::named(BLACK))
//             }
//             TileType::WallVar1 => ColorPair::new(RGB::named(DIMGRAY), RGB::named(BLACK)),
//             TileType::WallVar2 => ColorPair::new(RGB::named(MAROON), RGB::named(BLACK)),
//             // TileType::WallVar3 => ColorPair::new(RGB::from_u8(43, 29, 14), RGB::named(BLACK)),
//             TileType::Exit => ColorPair::new(RGB::named(RED), RGB::named(BLACK)),
//         }
//     }

//     fn out_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
//         match tile_type {
//             // TileType::Floor => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::FloorVar1 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::FloorVar2 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::FloorVar3 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::Wall => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::WallVar1 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::WallVar2 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             // TileType::WallVar3 => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//             TileType::Exit => ColorPair::new(RGB::named(DARKRED), RGB::named(BLACK)),
//             _ => ColorPair::new(RGB::named(DARKBLUE), RGB::named(BLACK)),
//         }
//     }
// }

// impl ForestTheme {
//     pub fn new() -> Box<dyn MapTheme> {
//         Box::new(Self {})
//     }
// }
