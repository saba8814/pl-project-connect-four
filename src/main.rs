use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window,enums::*,image::PngImage};
use std::io::{BufRead, BufReader};
use std::str::FromStr;
use rand::Rng;
use std::fs::File;
use std::io::Write;
use chrono::Local;

const BG_COLOR: u32 = 0x118ab2;
const BURCH_BLUE: u32 = 0x073b4c;
const BOARD_COLOR: u32 = 0x073b4c;
const COIN_RED: u32 = 0xef476f;
const COIN_BLUE: u32 = 0xffd166;

fn make_move(mut move_nr:i32){
    println!("move is {}",move_nr);
}
fn game_restart(){
    println!("game has been restarted");
}
fn load_save_game(){
    let reader = BufReader::new(File::open("test.txt").expect("Cannot open file.txt"));

    for line in reader.lines() {
        for word in line.unwrap().split_whitespace() {
            println!("{}", word);
        }
    }
}

fn save_game(){
    let date = Local::now();
    let mut file = File::create(format!("{}{}",date.format("%Y-%m-%d-%H-%M-%S").to_string(),"save-game.txt")).expect("create failed");
    file.write_all("test test".as_bytes()).expect("write failed");
}

fn draw_back_board(){
    let mut board_back = Frame::new(130, 88, 650, 437, "");
    board_back.set_frame(FrameType::RFlatBox);
    board_back.set_color(Color::from_u32(BOARD_COLOR));
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

fn draw_UI(){
    draw_back_board();
    draw_buttons();
    draw_logo();
    draw_text_label();
}
fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 800, 600, "Connect Four || IBU IT || PL Project");
    let mut grid = Grid::new();
    draw_UI();
    grid.print();
    window.set_color(Color::from_u32(BG_COLOR));
    window.end();
    window.show();
    app.run().unwrap(); 
}

struct Grid {
    column_count: i32,
    row_count: i32,
    rows: Vec<Row>,
}
impl Grid {
    pub fn new() -> Grid {
        Grid {
            column_count: 7,
            row_count: 6,
            rows: (0..7).map(|_| Row::new(6)).collect(),
        }
    }
    pub fn print(&self) {
        for column in 0..self.column_count {
            for row in 0..self.row_count{
                if row==0{
                    let mut circle=Frame::new(150+column*94, 90, 50, 50, "");
                    circle.set_frame(FrameType::OvalBox);
                    if(self.rows[column as usize].tiles[row as usize].value=="RED"){
                        circle.set_color(Color::from_u32(COIN_RED));
                    }
                    else{
                        circle.set_color(Color::from_u32(COIN_BLUE));
                    }
                }
                else{
                    let mut circle=Frame::new(150+column*94, (row+1)*79, 50, 50, "");
                    circle.set_frame(FrameType::OvalBox);
                    if(self.rows[column as usize].tiles[row as usize].value=="RED"){
                        circle.set_color(Color::from_u32(COIN_RED));
                    }
                    else{
                        circle.set_color(Color::from_u32(COIN_BLUE));
                    }
                } 
            }
        }
    }
}
struct Row {
    tiles: Vec<Tile>,
}
impl Row {
    pub fn new(column_count: usize) -> Row {
        let mut rng = rand::thread_rng();
        Row {
            tiles: (0..column_count).map(|_| (if rng.gen_range(0..2)==1{Tile::new("RED".to_string())}else{Tile::new("BLUE".to_string())})).collect(),
        }
    }
    pub fn len(&self) -> usize {
        self.tiles.len() as usize
    }

}

struct Tile{
    value: String
}

impl Tile{
    pub fn new(tile_value: String) -> Tile{
        Tile{
            value: tile_value
        }
    }
}