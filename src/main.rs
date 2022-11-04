use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window,enums::*,image::PngImage};
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::Write;
use chrono::Local;

const BG_COLOR: u32 = 0x118ab2;
const BURCH_BLUE: u32 = 0x073b4c;
const BOARD_COLOR: u32 = 0x073b4c;
const COIN_RED: u32 = 0xef476f;
const COIN_YELLOW: u32 = 0xffd166;

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

fn draw_logo(){
    let mut logo_place_holder = Frame::new(20, 30, 100, 50, "");
    let mut logo: PngImage = PngImage::load(&std::path::Path::new("logo.png")).unwrap();
    logo.scale(100,50,true,true);
    logo_place_holder.set_image(Some(logo));
}

fn draw_ui(){
    draw_back_board();
    draw_logo();
}
fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 800, 600, "Connect Four || IBU IT || PL Project");

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

    //Playing Buttons emits
    let (s1, r1) = app::channel::<String>();
    let (s2, r2) = app::channel::<String>();
    let (s3, r3) = app::channel::<String>();
    let (s4, r4) = app::channel::<String>();
    let (s5, r5) = app::channel::<String>();
    let (s6, r6) = app::channel::<String>();
    let (s7, r7) = app::channel::<String>();
    place_button1.emit(s1,"1".to_string());
    place_button2.emit(s2,"2".to_string());
    place_button3.emit(s3,"3".to_string());
    place_button4.emit(s4,"4".to_string());
    place_button5.emit(s5,"5".to_string());
    place_button6.emit(s6,"6".to_string());
    place_button7.emit(s7,"7".to_string());

    //Playing Buttons colors
    place_button1.set_color(Color::from_u32(BURCH_BLUE));
    place_button2.set_color(Color::from_u32(BURCH_BLUE));
    place_button3.set_color(Color::from_u32(BURCH_BLUE));
    place_button4.set_color(Color::from_u32(BURCH_BLUE));
    place_button5.set_color(Color::from_u32(BURCH_BLUE));
    place_button6.set_color(Color::from_u32(BURCH_BLUE));
    place_button7.set_color(Color::from_u32(BURCH_BLUE));
    window.set_color(Color::from_u32(BG_COLOR));
    //MAIN STARTS HERE
    draw_ui();
    let mut game = Game::new();
    game.start_game();
    window.end();
    window.show();
    while app.wait() {
        if let Some(msg) = r1.recv() {
            game.place_coin(msg.parse::<i32>().unwrap());
            app::redraw();
        }
        if let Some(msg) = r2.recv() {
            game.place_coin(msg.parse::<i32>().unwrap());
            app::redraw();
        }
        if let Some(msg) = r3.recv() {
            game.place_coin(msg.parse::<i32>().unwrap());
            app::redraw();
        }
        if let Some(msg) = r4.recv() {
            game.place_coin(msg.parse::<i32>().unwrap());
            app::redraw();
        } 
        if let Some(msg) = r5.recv() {
            game.place_coin(msg.parse::<i32>().unwrap());
            app::redraw();
        }
        if let Some(msg) = r6.recv() {
            game.place_coin(msg.parse::<i32>().unwrap());
            app::redraw();
        }
        if let Some(msg) = r7.recv() {
            game.place_coin(msg.parse::<i32>().unwrap());
            app::redraw();
        }
    }

}


struct Game{
    player: String,
    state: Vec<Vec<(Frame,String)>>,
    label: Frame
}

impl Game{
    pub fn new()->Game{
        Game{
            player:"RED".to_string(),
            state: (0..6).map(|_| Vec::new()).collect(),
            label: Frame::new(260, 540, 400, 50, "")
        }
    }
    pub fn start_game(&mut self){
        for row in 0..6{
            for column in 0..7{
                if row==0{
                    let mut circle=Frame::new(150+column*94, 90, 50, 50, "");
                    circle.set_frame(FrameType::OvalBox);
                    circle.set_color(Color::from_u32(BG_COLOR));
                    self.state[row].push((circle,"EMPTY".to_string()));
                }
                else{
                    let mut circle=Frame::new(150+column*94, ((row+1)*79) as i32, 50, 50, "");
                    circle.set_frame(FrameType::OvalBox);
                    circle.set_color(Color::from_u32(BG_COLOR));
                    self.state[row].push((circle,"EMPTY".to_string()));
                }
            }
        }
        self.label.set_label("RED PLAYER IS ON THE MOVE");
        self.label.set_label_size(30);
        self.label.set_label_color(Color::from_u32(BURCH_BLUE));
    }
    pub fn is_move_valid(&mut self,column:i32)->i32{
        let mut row_place=7;
        for row in (0..6).rev(){
            if self.state[row][(column-1) as usize].1=="EMPTY"{
                row_place=row;
                break;
            }
        }
        return row_place as i32;
    }
    pub fn place_coin(&mut self,column:i32){
        let row_place=self.is_move_valid(column);
        if row_place==7{
            return;
        }
        if self.player=="RED"{
            self.state[row_place as usize][(column-1) as usize].0.set_color(Color::from_u32(COIN_RED));
            self.state[row_place as usize][(column-1) as usize].1="RED".to_string();
            self.player="YELLOW".to_string();
            self.label.set_label("YELLOW PLAYER IS ON THE MOVE");
        }
        else{
            self.state[row_place as usize][(column-1) as usize].0.set_color(Color::from_u32(COIN_YELLOW));
            self.state[row_place as usize][(column-1) as usize].1="YELLOW".to_string();
            self.player="RED".to_string();
            self.label.set_label("RED PLAYER IS ON THE MOVE");
        } 
    }
}