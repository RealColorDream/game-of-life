use std::cell::RefCell;
use std::rc::Rc;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, Window, OutputSettingsBuilder};
use sdl2::keyboard::Keycode;

const HEIGHT : u32 = 100;
const WIDTH: u32 = HEIGHT;

struct Game {
    delay: Rc<RefCell<u32>>,
    array: [[u8; HEIGHT as usize]; WIDTH as usize],
    display: SimulatorDisplay<BinaryColor>,
    mid_x: usize,
    mid_y: usize,
    is_freeze: bool,
    window: Window,
    cursor_x:u8, // cursor x position
    cursor_y:u8, // cursor y position
}

impl Game {
    fn new() -> Game {
        let output_settings = OutputSettingsBuilder::new()
            .theme(BinaryColorTheme::OledBlue)
            .build();

        let window = Window::new("Conway's Game Of life", &output_settings);

        Game {
            delay: Rc::new(RefCell::new(20)),
            array: [[0; HEIGHT as usize]; WIDTH as usize],
            display: SimulatorDisplay::<BinaryColor>::new(Size::new(HEIGHT, WIDTH)),
            mid_x: (WIDTH as usize / 2),
            mid_y: HEIGHT as usize / 2,
            is_freeze: false,
            window,
            cursor_x: 0,
            cursor_y: 0,
        }
    }

    fn run(&mut self) -> Result<(), core::convert::Infallible> {
        let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

        self.display.clear(BinaryColor::Off).unwrap();

        Text::new("Conway's", Point::new(self.mid_x as i32 - 27, self.mid_y as i32 - 10), text_style)
            .draw(&mut self.display)
            .unwrap();

        Text::new("Game of Life", Point::new(self.mid_x as i32 - 30, self.mid_y as i32 + 5), text_style)
            .draw(&mut self.display)
            .unwrap();

        self.window.update(&mut self.display);
        std::thread::sleep(std::time::Duration::from_secs(2));

        'game: loop {
            self.update_screen();
            if self.is_freeze == false {
                self.array = self.evolve();
            }

            std::thread::sleep(std::time::Duration::from_millis(*self.delay.borrow_mut() as u64));
            self.window.update(&mut self.display);

            let events: Vec<_> = self.window.events().collect();

            for event in events {
                match event {
                    embedded_graphics_simulator::SimulatorEvent::MouseButtonUp {point, ..} => {
                        let x = point.x as usize;
                        let y = point.y as usize;

                        self.create_a_lwss_at_xy(x, y);
                    }
                    embedded_graphics_simulator::SimulatorEvent::KeyDown {keycode, ..} => {
                        match keycode {
                            Keycode::Space => {
                                self.is_freeze = !self.is_freeze;
                            }
                            Keycode::Tab => {
                                self.array = [[0; HEIGHT as usize]; WIDTH as usize];
                            }
                            Keycode::Up => {
                                if self.cursor_y > 0 {
                                    self.cursor_y -= 1;
                                }
                            }
                            Keycode::Down => {
                                if self.cursor_y < HEIGHT as u8 - 1 {
                                    self.cursor_y += 1;
                                }
                            }
                            Keycode::Left => {
                                if self.cursor_x > 0 {
                                    self.cursor_x -= 1;
                                }
                            }
                            Keycode::Right => {
                                if self.cursor_x < WIDTH as u8 - 1 {
                                    self.cursor_x += 1;
                                }
                            }
                            Keycode::Return => {
                                self.place_pixel(self.cursor_x as usize, self.cursor_y as usize);
                            }
                            Keycode::Escape => break 'game,
                            _ => {}
                        }
                    }
                    embedded_graphics_simulator::SimulatorEvent::Quit => break 'game,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn update_screen(&mut self) {
        for i in 0..self.array.len() {
            for j in 0..self.array[i].len() {
                if self.array[i][j] == 1 {
                    Pixel(Point::new(i as i32, j as i32), BinaryColor::On)
                        .draw(&mut self.display)
                        .expect("Failed to draw pixel");
                } else {
                    Pixel(Point::new(i as i32, j as i32), BinaryColor::Off)
                        .draw(&mut self.display)
                        .expect("Failed to draw pixel");
                }
            }
        }
    }

    fn evolve(&self) -> [[u8; HEIGHT as usize]; WIDTH as usize] {
        let mut new_array = self.array;
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let nb_neighbors = self.number_of_neighbors(i as usize, j as usize);
                if self.array[i as usize][j as usize] == 1 {
                    if nb_neighbors < 2 || nb_neighbors > 3 {
                        new_array[i as usize][j as usize] = 0;
                    }
                } else {
                    if nb_neighbors == 3 {
                        new_array[i as usize][j as usize] = 1;
                    }
                }
            }
        }
        new_array
    }

    fn number_of_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count:u8 = 0;
        let x_range = if x > 0 { x-1 } else { x }..x+2;
        let y_range = if y > 0 { y-1 } else { y }..y+2;

        for i in x_range {
            for j in y_range.clone() {
                if i == x && j == y {
                    continue;
                }
                if i < 0 || j < 0 || i >= HEIGHT as usize || j >= WIDTH as usize {
                    continue;
                }
                if self.array[i][j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }

    fn create_a_lwss_at_xy(&mut self, x: usize, y: usize) {
        self.array[x][y] = 1;
        self.array[x][y+1] = 1;
        self.array[x][y+2] = 1;
        self.array[x][y+3] = 1;
        self.array[x+1][y-1] = 1;
        self.array[x+1][y+3] = 1;
        self.array[x+2][y+3] = 1;
        self.array[x+3][y-1] = 1;
        self.array[x+3][y+2] = 1;
    }

    fn place_pixel(&mut self, x: usize, y: usize) {
        if self.array[x][y] == 1 {
            self.array[x][y] = 0;
        }
        else {
            self.array[x][y] = 1;
        }
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut game = Game::new();
    game.run()
}