use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window,enums::*,image::PngImage};
const BURCH_BLUE: u32 = 0x00396d;
const BOARD_COLOR: u32 = 0xf4fa04;
const BG_COLOR: u32 = 0xadd8e6;
const COIN_RED: u32 = 0xa3180a;
fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 800, 600, "Connect Four || IBU IT || PL Project");
    let mut board_back = Frame::new(130, 88, 650, 437, "");
    let mut logo_place_holder = Frame::new(10, -20, 100, 150, "");
    board_back.set_frame(FrameType::RFlatBox);
    board_back.set_color(Color::from_u32(BOARD_COLOR));
    for n in 0..7 {
        for m in 0..6{
            if(m==0){
                let mut circle=Frame::new(150+n*94, 90, 50, 50, "");
                circle.set_frame(FrameType::OvalBox);
                circle.set_color(Color::White);
            }
            else{
                let mut circle=Frame::new(150+n*94, (m+1)*79, 50, 50, "");
                circle.set_frame(FrameType::OvalBox);
                circle.set_color(Color::White);
            } 
        }
    }
    let mut load_button = Button::new(20, 90, 95, 50, "LOAD");
    let mut save_button = Button::new(20, 155, 95, 50, "SAVE");
    let mut place_button1 = Button::new(130, 20, 80, 50, "PLACE");
    let mut place_button2  = Button::new(225, 20, 80, 50, "PLACE");
    let mut place_button3  = Button::new(320, 20, 80, 50, "PLACE");
    let mut place_button4  = Button::new(415, 20, 80, 50, "PLACE");
    let mut place_button5  = Button::new(510, 20, 80, 50, "PLACE");
    let mut place_button6  = Button::new(605, 20, 80, 50, "PLACE");
    let mut place_button7  = Button::new(700, 20, 80, 50, "PLACE");
    place_button1.set_color(Color::from_u32(BURCH_BLUE));
    place_button2.set_color(Color::from_u32(BURCH_BLUE));
    place_button3.set_color(Color::from_u32(BURCH_BLUE));
    place_button4.set_color(Color::from_u32(BURCH_BLUE));
    place_button5.set_color(Color::from_u32(BURCH_BLUE));
    place_button6.set_color(Color::from_u32(BURCH_BLUE));
    place_button7.set_color(Color::from_u32(BURCH_BLUE));
    load_button.set_color(Color::from_u32(BURCH_BLUE));
    save_button.set_color(Color::from_u32(BURCH_BLUE));
    window.set_color(Color::from_u32(BG_COLOR));
    let mut logo: PngImage = PngImage::load(&std::path::Path::new("logo.png")).unwrap();
    logo.scale(100,50,true,true);
    logo_place_holder.set_image(Some(logo));
    window.end();
    window.show();
    app.run().unwrap();
}