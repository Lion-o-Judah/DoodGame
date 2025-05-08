use std::fmt;
use crate::world::tile::{TileType,Tile};
use crate::world::error::TileError;
use rand::Rng;

pub struct Map {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,

}

impl Map { 

    fn blank_map(height:usize, width:usize, tile_type:TileType) -> Self {
        let mut map = Self{
            width: width,
            height: height,
            tiles: Vec::new(),
        };

        for _ in 0..(width*height){
            map.tiles.push(Tile{tile_type:tile_type.clone(),tile_symbol:'.'});
        }

        map
    }
    pub fn get_tile(&mut self, x:usize, y:usize) -> Result<&mut Tile, TileError>{
        if x > self.width {
            return Err(TileError::OutOfBoundsX);
        }
        if y > self.width {
            return Err(TileError::OutOfBoundsY);
        }

        let index: usize = (self.width * y) + x;

        if let Some(tile) = self.tiles.get_mut(index){
            Ok(tile)
        } else {
            return Err(TileError::TileMapIncomplete);
        }
        
    }

    pub fn set_tile_type(&mut self, x:usize, y:usize, new_type:TileType) -> Result<(), TileError>{
        let tile: &mut Tile = self.get_tile(x,y)?;
        tile.tile_type = new_type;
        tile.tile_symbol = tile.get_char();
        Ok(())
    }

    pub fn random_map() -> Result<Self, TileError>{ 
        let mut blank_map = Self::blank_map(30,30,TileType::Grass);
        let mut rng = rand::rng();
        let number_of_walls = rng.random_range(6..=12);
        for _ in 0..number_of_walls{
            let x = rng.random_range(0..blank_map.width);
            let y = rng.random_range(0..blank_map.height);
            blank_map.set_tile_type(x,y,TileType::Wall)?;
        }
        Ok(blank_map)
    }
}
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width:{}\nheight:{}\n", self.width, self.height)?;
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f,"{}", self.tiles[(y*self.height) + x].tile_symbol)?;
            }
            writeln!(f,"")?;
        }
        Ok(())
    }
}
