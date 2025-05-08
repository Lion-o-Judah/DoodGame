#[derive(Clone)]
pub enum TileType {
    Grass,
    Wall,
}



pub struct Tile{
    pub tile_type: TileType,
    pub tile_symbol: char
}

impl Tile{
    pub fn get_char(&self) -> char{
        match self.tile_type{
            TileType::Grass => {'.'},
            TileType::Wall => {'\u{1f333}'},
        }       
    }
}


