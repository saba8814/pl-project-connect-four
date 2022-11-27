use fltk::{app, button::Button, frame::Frame, prelude::*, enums::*,image::PngImage};
use std::io::{BufReader};
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use crate::constants::BG_ACTIVE;
use crate::constants::BG_COLOR;
use crate::constants::BOARD_COLOR;

pub fn play_coin_sound(){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("coin_place.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(150));
}
fn draw_back_board(){
    let mut board_back = Frame::new(130, 88, 650, 437, "");
    board_back.set_frame(FrameType::RFlatBox);
    board_back.set_color(Color::from_u32(BOARD_COLOR));
}

fn draw_logo(){
    let mut logo_place_holder = Frame::new(20, 30, 100, 50, "");
    let mut logo: PngImage = PngImage::load(&std::path::Path::new("logo.png")).unwrap();
    logo.scale(100,50,true,true);
    logo_place_holder.set_image(Some(logo));
}
fn draw_game_control_buttons(){
    //Game Control Buttons
    let mut save_button = Button::new(20, 155, 95, 50, "SAVE");
    let mut restart_button = Button::new(20, 220, 95, 50, "PLAY");
    let mut load_button = Button::new(20, 90, 95, 50, "LOAD");
    //Game Control Buttons Style
    load_button.set_selection_color(Color::from_u32(BG_COLOR));
    load_button.set_color(Color::from_u32(BG_ACTIVE));
    load_button.clear_visible_focus();
    load_button.set_frame(FrameType::RFlatBox);
    load_button.set_label_color(Color::White);
    save_button.set_color(Color::from_u32(BG_ACTIVE));
    restart_button.set_color(Color::from_u32(BG_ACTIVE));
    save_button.clear_visible_focus();
    restart_button.clear_visible_focus();
    save_button.set_frame(FrameType::RFlatBox);
    restart_button.set_frame(FrameType::RFlatBox);
    save_button.set_label_color(Color::White);
    restart_button.set_label_color(Color::White);
    //Sending and recieving channels
    let (s2, _r1) = app::channel::<String>();
    let (s3, _r1) = app::channel::<String>();
    let (s4, _r1) = app::channel::<String>();

    //Game Control Buttons emits
    restart_button.emit(s2,"RESTART".to_string());
    save_button.emit(s3,"SAVE".to_string());
    load_button.emit(s4,"LOAD".to_string());
}
pub fn draw_ui(){
    draw_back_board();
    draw_logo();
    draw_game_control_buttons();
}