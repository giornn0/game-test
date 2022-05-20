use sdl2::rect::{Point, Rect};

#[derive(Debug)]
pub struct Player{
  position: Point,
  sprite: Rect,
  speed: i32,
}

impl Player{
  pub fn new(position: Point, sprite: Rect,speed: i32)->Player{
    Player{position, sprite, speed}
  }
  pub fn position(&self)->Point{
    self.position
  }
  pub fn sprite(&self)->Rect{
    self.sprite
  }
  pub fn move_y(mut self , direction: Option<bool>)->Player{
      let moving: i32 = if direction.unwrap_or(true) {0 - self.speed} else {0+self.speed} ;
      self.position = self.position.offset(0, moving);
      self
  }
  pub fn move_x(mut self , direction: Option<bool>)->Player{
      let moving: i32 = if direction.unwrap_or(true) {0 - self.speed} else {0+self.speed} ;
      self.position = self.position.offset(moving, 0);
      self
  }
}
