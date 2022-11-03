use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window,enums::*,image::PngImage};


const BURCH_BLUE: u32 = 0x00396d;
const BOARD_COLOR: u32 = 0xf4fa04;
const BG_COLOR: u32 = 0xadd8e6;
const COIN_RED: u32 = 0xa3180a;

fn make_move(mut move_nr:i32){
    println!("move is {}",move_nr);
}
fn game_restart(){
    println!("game has been restarted");
}
fn load_save_game(){
    println!("game save has been loaded");
}

fn save_game(){
    println!("game progress has been saved");
}

fn draw_board(){
    let mut board_back = Frame::new(130, 88, 650, 437, "");
    board_back.set_frame(FrameType::RFlatBox);
    board_back.set_color(Color::from_u32(BOARD_COLOR));
    for n in 0..7 {
        for m in 0..6{
            if m==0{
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
}
fn draw_buttons(){
    //Game Control Buttons
    let mut load_button = Button::new(20, 90, 95, 50, "LOAD");
    let mut save_button = Button::new(20, 155, 95, 50, "SAVE");
    let mut restart_button = Button::new(20, 220, 95, 50, "RESTART");

    //Playing Buttons
    let mut place_button1 = Button::new(130, 20, 80, 50, "PLACE\n|\nv");
    let mut place_button2  = Button::new(225, 20, 80, 50, "PLACE\n|\nv");
    let mut place_button3  = Button::new(320, 20, 80, 50, "PLACE\n|\nv");
    let mut place_button4  = Button::new(415, 20, 80, 50, "PLACE\n|\nv");
    let mut place_button5  = Button::new(510, 20, 80, 50, "PLACE\n|\nv");
    let mut place_button6  = Button::new(605, 20, 80, 50, "PLACE\n|\nv");
    let mut place_button7  = Button::new(700, 20, 80, 50, "PLACE\n|\nv");
    
    //Game Control Buttons Callbacks
    load_button.set_callback(|_| load_save_game());
    save_button.set_callback(|_| save_game());
    restart_button.set_callback(|_| game_restart());

    //Game Control Buttons Style
    load_button.set_color(Color::from_u32(BURCH_BLUE));
    save_button.set_color(Color::from_u32(BURCH_BLUE));
    restart_button.set_color(Color::from_u32(BURCH_BLUE));

    //Playing Buttons callbacks
    place_button1.set_callback(|_| make_move(1));
    place_button2.set_callback(|_| make_move(2));
    place_button3.set_callback(|_| make_move(3));
    place_button4.set_callback(|_| make_move(4));
    place_button5.set_callback(|_| make_move(5));
    place_button6.set_callback(|_| make_move(6));
    place_button7.set_callback(|_| make_move(7));

    //Playing Buttons colors
    place_button1.set_color(Color::from_u32(BURCH_BLUE));
    place_button2.set_color(Color::from_u32(BURCH_BLUE));
    place_button3.set_color(Color::from_u32(BURCH_BLUE));
    place_button4.set_color(Color::from_u32(BURCH_BLUE));
    place_button5.set_color(Color::from_u32(BURCH_BLUE));
    place_button6.set_color(Color::from_u32(BURCH_BLUE));
    place_button7.set_color(Color::from_u32(BURCH_BLUE));
}
fn draw_logo(){
    let mut logo_place_holder = Frame::new(20, 30, 100, 50, "");
    let mut logo: PngImage = PngImage::load(&std::path::Path::new("logo.png")).unwrap();
    logo.scale(100,50,true,true);
    logo_place_holder.set_image(Some(logo));
}
fn draw_text_label(){
    let mut text_place_holder = Frame::new(260, 540, 400, 50, "YELLOW PLAYER WINS");
    text_place_holder.set_label_size(30);
    text_place_holder.set_label_color(Color::from_u32(BURCH_BLUE));
}
fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 800, 600, "Connect Four || IBU IT || PL Project");
    draw_board();
    draw_buttons();
    draw_logo();
    draw_text_label();
    window.set_color(Color::from_u32(BG_COLOR));
    window.end();
    window.show();
    app.run().unwrap();
}


