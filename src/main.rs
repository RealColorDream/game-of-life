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

fn main() -> Result<(), core::convert::Infallible> {
    let mut delay = Rc::new(RefCell::new(20)); // enable to change the delay in the event loop
    let mut array :[[u8; HEIGHT as usize]; WIDTH as usize]= [[0; HEIGHT as usize]; WIDTH as usize];
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(HEIGHT, WIDTH));
    let mid_x = (WIDTH as usize / 2);
    let mid_y = HEIGHT as usize / 2;
    let mut is_freeze = false;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    let mut window = Window::new("Conway's Game Of life", &output_settings);

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    display.clear(BinaryColor::Off).unwrap();
    
    Text::new("Conway's", Point::new(mid_x as i32 - 27, mid_y as i32 - 10), text_style)
        .draw(&mut display)
        .unwrap();

    Text::new("Game of Life", Point::new(mid_x as i32 - 30, mid_y as i32 + 5), text_style)
        .draw(&mut display)
        .unwrap();

    window.update(&mut display);
    std::thread::sleep(std::time::Duration::from_secs(3)); // delay in seconds

    'game: loop{
        update_screen(array, &mut display);
        if (is_freeze == false){
            array = evolve(array);
        }

        std::thread::sleep(std::time::Duration::from_millis(*delay.borrow_mut() as u64)); // delay in ms
        window.update(&mut display);

        for events in window.events(){
            match events {
                embedded_graphics_simulator::SimulatorEvent::MouseButtonUp {point, ..} => {
                    let x = point.x as usize;
                    let y = point.y as usize;

                    create_a_lwss_at_xy(&mut array, x, y);
                }
                embedded_graphics_simulator::SimulatorEvent::KeyDown {keycode, ..} => {
                    match keycode{
                        Keycode::Space => {
                            if is_freeze == false{
                                is_freeze = true;
                            }
                            else {
                                is_freeze = false;
                            }
                        }
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

fn print_array(array: [[u8; HEIGHT as usize]; WIDTH as usize]){ // used for debugging
    for i in 0..array.len(){
        for j in 0..array[i].len(){
            print!("{}", array[i][j]);
        }
        println!();
    }
}

fn update_screen(array: [[u8; HEIGHT as usize]; WIDTH as usize], display: &mut SimulatorDisplay<BinaryColor>){
    for i in 0..array.len(){
        for j in 0..array[i].len(){
            if array[i][j] == 1{
                Pixel(Point::new(i as i32, j as i32), BinaryColor::On)
                    .draw(display)
                    .expect("Failed to draw pixel");
            }
            else {
                Pixel(Point::new(i as i32, j as i32), BinaryColor::Off)
                    .draw(display)
                    .expect("Failed to draw pixel");
            }
        }
    }
}

fn number_of_neighbors(array: [[u8; HEIGHT as usize]; WIDTH as usize], x: usize, y: usize) -> u8 {
    let mut count:u8 = 0;
    let x_range = if x > 0 { x-1 } else { x }..x+2;
    let y_range = if y > 0 { y-1 } else { y }..y+2;

    for i in x_range {
        for j in y_range.clone() {
            if i == x && j == y{
                continue;
            }
            if i < 0 || j < 0 || i >= HEIGHT as usize || j >= WIDTH as usize{
                continue;
            }
            if array[i][j] == 1{
                count += 1;
            }
        }
    }
    return count;
}

fn evolve(array: [[u8; HEIGHT as usize]; WIDTH as usize]) -> [[u8; HEIGHT as usize]; WIDTH as usize] {
    let mut new_array = array;
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let nb_neighbors = number_of_neighbors(array, i as usize, j as usize);
            if array[i as usize][j as usize] == 1 {
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

fn create_a_lwss_at_xy(array: &mut [[u8; HEIGHT as usize]; WIDTH as usize], x: usize, y: usize){
    array[x][y] = 1;
    array[x][y+1] = 1;
    array[x][y+2] = 1;
    array[x][y+3] = 1;
    array[x+1][y-1] = 1;
    array[x+1][y+3] = 1;
    array[x+2][y+3] = 1;
    array[x+3][y-1] = 1;
    array[x+3][y+2] = 1;
}
