mod models;

use std::time::Duration;
use models::player::Player;
use sdl2::event::Event;
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::pixels::Color;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::render::{WindowCanvas, Texture};

fn render(canvas: &mut WindowCanvas, color: Color, texture: &Texture, player: &Player )-> Result<(), String>{
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let sprite = player.sprite();

    let screen_position = player.position() + Point::new(width as i32 /2 , height as i32 /2);
    let screen_rect = Rect::from_center(screen_position, sprite.width(), sprite.height());
    
    canvas.copy(texture,sprite, screen_rect)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let vide_subsystem = sdl_context.video()?;

    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

   

    let window = vide_subsystem
        .window("game-tuto", 800, 600)
        .position_centered()
        .build()
        .expect("Unable to start video subsystem");
    let mut canvas = window.into_canvas().build().expect("Error creatin canvas");

    let texture_creator = canvas.texture_creator();

    let bard = texture_creator.load_texture("assets/bardo.png")?;
    let reaper = texture_creator.load_texture("assets/reaper.png")?;

    let mut player = Player::new(Point::new(0,0),Rect::new(0,0,26,36),5);
    
    
    let mut reaper_pos = Point::new(-45, 150); // reaper in the left-bottom quadrant
    //src position in the sprites;
    let mut reaper_sprite = Rect::new(0,0,26,36);

    let mut even_pump = sdl_context.event_pump()?;

    let mut i = 0;

    'running: loop{
      //Event handler
      for event in even_pump.poll_iter(){
        match event{
            Event::Quit{..}|
            Event::KeyDown{keycode: Some(Keycode::Escape), ..}=>{
              break 'running;
            },
            Event::KeyDown { keycode: Some(Keycode::Down), .. }=>{
              player = player.move_y(Some(false));
            },
            Event::KeyDown { keycode: Some(Keycode::Left), .. }=>{
              player = player.move_x(None);
            },
            Event::KeyDown { keycode: Some(Keycode::Right), .. }=>{
              player = player.move_x(Some(false));
            },
            Event::KeyDown { keycode: Some(Keycode::Up), .. }=>{
              player = player.move_y(None);
            },
            _=>{}
          }
        }
        //Update
        i = (i+1) % 255;
        let color = Color::RGB(i,64, 255-i);
        reaper_pos = Point::new(-45 +i as i32, 65);
        
        //Render
        // render(&mut canvas, color.to_owned(), &reaper, reaper_pos, reaper_sprite)?;
        render(&mut canvas, color.to_owned(), &bard, &player)?;

        //Time management
        ::std::thread::sleep(Duration::new(0,1_000_000_000u32/60));
    }
    Ok(())
}
