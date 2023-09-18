use ggez::graphics::{self, Color, Image, Mesh, Text, DrawParam};
use ggez::{Context, GameResult};
use ggez::glam::*;
use ggez::event::{self};
use ggez::audio::{SoundSource, SoundData};
use ggez::audio::Source;
use ggez::audio;
use std::path::Path;
use ggez::conf::{WindowMode,WindowSetup};
use ggez::input::keyboard::{KeyCode, KeyInput};
use rand::Rng;

struct MainState {
    pos_x: f32,
    pos_y: f32,
    acc: f32,
    gas: f32,
    img: graphics::Image,
  //  circle: graphics::Mesh,
    base: graphics::Mesh,
    base_x: f32,
    base_y: f32,
    pause: bool,
    write_win: bool,
    write_lose: bool,
}


impl MainState {

    fn new(ctx: &mut Context) -> GameResult<MainState>{
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(20.0..1240.0) ;
/* 
        let circle = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(0.0,0.0,50.0,100.0),
            Color::WHITE,
        )?;*/

        let img = Image::from_path(ctx, "/foguete.png")?;
        let base = Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(10.0,10.0,100.0,20.0),
            Color::WHITE,
        )?;

        Ok(MainState { 
            pos_x: 615.0,
            pos_y: 0.0, 
            acc: 0.0, 
            gas: 50.0, 
         //   circle, 
            base, 
            base_x:value, 
            base_y: 680.0, 
            pause: true, 
            write_win: false,
            write_lose: false,
            img
            })
    }
}

impl event::EventHandler<ggez::GameError> for MainState {

    fn update(&mut self, _ctx: &mut Context) -> GameResult {   
        const FPS: u32 = 60;

    
        while _ctx.time.check_update_time(FPS) && self.pause == false{
            if (self.pos_y >= 580.0 && self.pos_y <= 590.0){
                //ganhou
                if self.pos_x <= self.base_x + 60.0 && self.pos_x >= self.base_x - 40.0{
                    
                        self.pause = true;
                        self.write_win = true;
                    
                }
            }
            if self.pos_x > 1300.0 || self.pos_x < -20.0 || self.pos_y > 750.0 || self.pos_y < -200.0{
                //game over
                self.pause = true;
                self.write_lose = true;
            }
            if self.gas > 0.1 {
                if self.acc < 12.0 {                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                      
                    self.acc = self.acc + 1.0; 
                }
                if self.acc < -20.0 {
                    self.acc = -20.0;
                }
            }
            self.pos_y += self.acc;
        }

        Ok(())
    }


    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));

        //canvas.draw(&self.circle, Vec2::new(self.pos_x, self.pos_y));
        canvas.draw(&self.img, Vec2::new(self.pos_x, self.pos_y));
        if self.write_lose{
            canvas.draw(
                &Text::new(format!("VOCÊ PERDEU \nPressione P para reiniciar \nPressione SPACE para começar")),
                 DrawParam::from(Vec2::new(200.0,200.0)).color(Color::WHITE));
        }
        if self.write_win{
            canvas.draw(
                &Text::new(format!("VOCÊ GANHOU!!!\n Pressione P para reiniciar \nPressione SPACE para começar")),
                 DrawParam::from(Vec2::new(200.0,200.0)).color(Color::WHITE));
        }
        canvas.draw(
            &Text::new(format!("Gas: {}", self.gas)),
             DrawParam::from(Vec2::new(0.0,0.0)).color(Color::WHITE));
        canvas.draw(
                &Text::new(format!("Acc: {}", self.acc)),
                 DrawParam::from(Vec2::new(100.0,0.0)).color(Color::WHITE));
        
        canvas.draw(&self.base, Vec2::new(self.base_x, self.base_y));
        canvas.finish(ctx)?;

        Ok(())
    }
    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput,_repeat: bool) -> GameResult {
        match input.keycode {
            Some(KeyCode::Space) =>{
                if self.pause == true{
                    self.pause = false;
                }else{
                    self.pause = true;
                }
                self.write_lose = false;
                self.write_win = false;
            }
            Some(KeyCode::P) =>{
                let mut rng = rand::thread_rng();
                let value = rng.gen_range(20.0..1240.0);
                self.base_x = value;
                self.gas = 50.0;
                self.pos_x = 680.0;
                self.pos_y = 0.0;
                self.acc = 0.0;
            }
            Some(KeyCode::A) => {
                // Mova o jogador para a esquerda
                if self.pos_x > -20.0 {
                    self.pos_x -= 10.0;
                }
            }
            Some(KeyCode::D) => {
                // Mova o jogador para a direita
                if self.pos_x < 1300.0 {
                    self.pos_x += 10.0;   
                }
            }
            Some(KeyCode::W) => {
                // Mova o jogador para cima
                if self.pos_y > 0.0 && self.gas > 0.0{
                    self.pos_y -= 10.0;
                    self.gas -= 1.0;
                    if self.acc < 30.0 {
                        self.acc -= 10.0;
                    }
                }
                if self.acc > 0.0 {
                    self.acc = self.acc - 10.0;
                }
            }
            Some(KeyCode::S) => {
                // Mova o jogador para baixo
                if self.pos_y < 750.0 {
                    self.pos_y += 10.0;
                }
            }
            _ => (),
        }


        Ok(())
    }
}


pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").window_setup(WindowSetup::default().title("Meu Jogo"))
    .window_mode(WindowMode::default().dimensions(1280.0, 700.0));
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}