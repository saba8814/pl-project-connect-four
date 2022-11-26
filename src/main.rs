use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window,enums::*,image::PngImage,input::IntInput};
use std::io::{BufReader};
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::Write;
use chrono::Local;
use rfd::FileDialog;
use std::io::Read;
use std::str::Split;

const MAX_SIZE: usize = 32;
const BG_COLOR: u32 = 0x121212;
const BG_ACTIVE: u32 = 0x1e1e1e;
const BOARD_COLOR: u32 = 0x1e1e1e;
const COIN_RED: u32 = 0xa30000;
const COIN_YELLOW: u32 = 0xff9800;

fn play_coin_sound(){
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

fn draw_ui(){
    draw_back_board();
    draw_logo();
}
fn main() {
    let app = app::App::default();
    let mut window = Window::new(100, 100, 800, 600, "Connect Four || IBU IT || PL Project");
    let logo: PngImage = PngImage::load(&std::path::Path::new("logo.png")).unwrap();
    window.set_icon(Some(logo));
    //Game Control Buttons
    let mut save_button = Button::new(20, 155, 95, 50, "SAVE");
    let mut restart_button = Button::new(20, 220, 95, 50, "PLAY");
    let mut load_button = Button::new(20, 90, 95, 50, "LOAD");
    let mut input_rows = IntInput::new(20,295,95,50,"");
    let mut input_columns = IntInput::new(20,365,95,50,"");
    let mut input_rows_label =Frame::new(20, 280, 95, 15, "# of ROWS");
    let mut input_column_label =Frame::new(20, 350, 95, 15, "# of COLUMNS");
    input_rows_label.set_label_color(Color::White);
    input_column_label.set_label_color(Color::White);
    input_rows.set_value("6");
    input_columns.set_value("7");
    input_rows.set_frame(FrameType::RFlatBox);
    input_columns.set_frame(FrameType::RFlatBox);
    input_rows.set_color(Color::from_u32(BG_ACTIVE));
    input_columns.set_color(Color::from_u32(BG_ACTIVE));
    input_rows.set_text_color(Color::White);
    input_columns.set_text_color(Color::White);
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

    window.set_color(Color::from_u32(BG_COLOR));
    //MAIN STARTS HERE
    draw_ui();
    let mut game = Game::new(6,7);
    game.start_game();
    window.end();
    window.show();
    while app.wait() {
        if let Some(msg) = _r1.recv() {
            if msg=="RESTART"{
                let rows:i32=input_rows.value().parse().unwrap();
                let columns:i32=input_columns.value().parse().unwrap();
                if rows as usize ==game.row_size && columns as usize==game.column_size{
                    game.restart_game();
                }
                else{
                    let mut diff=rows-columns;
                    diff=diff.abs();
                    if diff>2 || rows<6 || columns<6{
                        input_rows.set_value(&game.row_size.to_string());
                        input_columns.set_value(&game.column_size.to_string());
                        game.label.set_label_color(Color::from_u32(COIN_RED));
                        game.label.set_label("INVALID BOARD SIZE");
                    }
                    else{
                        game.column_size=columns as usize;
                        game.row_size=rows as usize;
                        game.restart_game();
                    }
                }
            }
            if msg=="LOAD"{
                let file_path=game.pick_save_game();
                if file_path!="NOT FOUND"{
                    let mut file = File::open(&file_path).expect("error");
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).expect("something went wrong reading the file");
                    let lines: Split<&str> = contents.split("\n");
                    let mut data: Vec<String> = Vec::new();
                    for line in lines{
                        data.push(line.to_string());
                    }
                    game.load_save_game(data);
                    input_rows.set_value(&game.row_size.to_string());
                    input_columns.set_value(&game.column_size.to_string());
                }
            }
            if msg=="SAVE"{
                game.save_game();
            }
            if (msg!="RESTART") && (msg!="LOAD") && (msg!="SAVE"){  
                game.place_coin(msg.parse::<i32>().unwrap());
                if game.winner=="RED"{
                    game.label.set_label_color(Color::from_u32(COIN_RED));
                }
                if game.winner=="YELLOW"{
                    game.label.set_label_color(Color::from_u32(COIN_YELLOW));
                }
            }
            app::redraw();
        }
    }

}


struct Game{
    player: String,
    row_size: usize,
    column_size: usize,
    state: Vec<Vec<(Frame,String)>>,
    label: Frame,
    winner: String,
    buttons: Vec<Button>
}

impl Game{
    pub fn new(rows:i32,columns:i32)->Game{
        Game{
            player:"RED".to_string(),
            row_size: rows as usize,
            column_size: columns as usize,
            state: (0..MAX_SIZE).map(|_| Vec::new()).collect(),
            label: Frame::new(260, 540, 400, 50, ""),
            winner: "EMPTY".to_string(),
            buttons: Vec::new()
        }
    }
    pub fn clear_board(&mut self){
        for row in 0..MAX_SIZE{
            for column in 0..MAX_SIZE{  
                self.state[row][column].1="EMPTY".to_string();
                self.state[row][column].0.resize(0,0,0,0);
                self.state[row][column].0.set_color(Color::from_u32(BG_COLOR));
            }
        }
    }
    pub fn restart_game(&mut self){
        self.update_buttons();
        self.clear_board();
        self.label.set_label_color(Color::from_u32(BG_ACTIVE));
        let mut coin_radius:i32=((650/self.column_size)).try_into().unwrap();
        coin_radius=coin_radius-40;
        if coin_radius>55{
            coin_radius=55;
        }
        if coin_radius<10{
            coin_radius=10;
        }
        for row in 0..self.row_size{
            for column in 0..self.column_size{
                if row==0{
                    self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90, coin_radius, coin_radius);
                }
                else{
                    self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+20)*(row) as i32, coin_radius, coin_radius);
                }
                self.state[row][column].0.set_color(Color::from_u32(BG_COLOR));
                self.state[row][column].1="EMPTY".to_string();
            }
        }
        self.change_player("RED".to_string());
        self.winner="EMPTY".to_string();
    }
    pub fn change_player(&mut self, player:String){
        self.player=player;
        if self.player=="RED"{
            self.label.set_label("RED PLAYER IS ON THE MOVE");
        }
        else{
            self.label.set_label("YELLOW PLAYER IS ON THE MOVE");
        }
    }
    pub fn check_diagonal_win(&mut self)->String{
        let mut winner="EMPTY";
        for row in 0..(self.row_size-3){
            for column in 0..(self.column_size-3){
                let val1=self.state[row as usize][column as usize].1.to_string();
                let val2=self.state[(row+1) as usize][(column+1) as usize].1.to_string();
                let val3=self.state[(row+2) as usize][(column+2) as usize].1.to_string();
                let val4=self.state[(row+3) as usize][(column+3)as usize].1.to_string();
                if val1=="RED" && val2=="RED" && val3=="RED" && val4=="RED"{
                    winner="RED";
                }
                if val1=="YELLOW" && val2=="YELLOW" && val3=="YELLOW" && val4=="YELLOW"{
                    winner="YELLOW";
                }
            }
        }
        for row in 0..(self.row_size-3){
            for column in 3..(self.column_size){
                let val1=self.state[row as usize][column as usize].1.to_string();
                let val2=self.state[(row+1) as usize][(column-1) as usize].1.to_string();
                let val3=self.state[(row+2) as usize][(column-2) as usize].1.to_string();
                let val4=self.state[(row+3) as usize][(column-3)as usize].1.to_string();
                if val1=="RED" && val2=="RED" && val3=="RED" && val4=="RED"{
                    winner="RED";
                }
                if val1=="YELLOW" && val2=="YELLOW" && val3=="YELLOW" && val4=="YELLOW"{
                    winner="YELLOW";
                }
            }
        }
        return winner.to_string();
    }
    pub fn check_vertical_win(&mut self)->String{
        let mut winner="EMPTY";
        for column in 0..self.column_size{
            for row in 0..(self.row_size-3){
                let val1=self.state[row as usize][column as usize].1.to_string();
                let val2=self.state[(row+1) as usize][column as usize].1.to_string();
                let val3=self.state[(row+2) as usize][column as usize].1.to_string();
                let val4=self.state[(row+3) as usize][column as usize].1.to_string();
                if val1=="RED" && val2=="RED" && val3=="RED" && val4=="RED"{
                    winner="RED";
                }
                if val1=="YELLOW" && val2=="YELLOW" && val3=="YELLOW" && val4=="YELLOW"{
                    winner="YELLOW";
                }
            }
        }
        return winner.to_string();
    }
    pub fn check_horizontal_win(&mut self)->String{
        let mut winner="EMPTY";
        for row in 0..self.row_size{
            for column in 0..(self.column_size-3){
                let val1=self.state[row as usize][column as usize].1.to_string();
                let val2=self.state[row as usize][(column+1) as usize].1.to_string();
                let val3=self.state[row as usize][(column+2) as usize].1.to_string();
                let val4=self.state[row as usize][(column+3) as usize].1.to_string();
                if val1=="RED" && val2=="RED" && val3=="RED" && val4=="RED"{
                    winner="RED";
                }
                if val1=="YELLOW" && val2=="YELLOW" && val3=="YELLOW" && val4=="YELLOW"{
                    winner="YELLOW";
                }
            }
        }
        return winner.to_string();
    }
    pub fn check_draw(&self)->String{
        for row in 0..(self.row_size){
            for column in 0..(self.column_size){
                if self.state[row as usize][column as usize].1.to_string()=="EMPTY"{
                    return "EMPTY".to_string();
                }
            }
        }
        return "DRAW".to_string();
    }
    pub fn check_winner(&mut self){
        if self.check_diagonal_win()!="EMPTY"{
            self.winner=self.check_diagonal_win().to_string();
            let winner_string=self.check_diagonal_win().to_string()+ " player wins!";
            self.label.set_label(&winner_string);
            return;
        }
        if self.check_vertical_win()!="EMPTY"{
            self.winner=self.check_vertical_win().to_string();
            let winner_string=self.check_vertical_win().to_string()+ " player wins!";
            self.label.set_label(&winner_string);
            return;
        }
        if self.check_horizontal_win()!="EMPTY"{
            self.winner=self.check_horizontal_win().to_string();
            let winner_string=self.check_horizontal_win().to_string()+ " player wins!";
            self.label.set_label(&winner_string);
            return;
        }
        if self.check_draw()!="EMPTY"{
            self.winner="DRAW".to_string();
            self.label.set_label("GAME FINISHED AS DRAW");
            return;
        }
    }
    pub fn start_game(&mut self){
        for i in 0..MAX_SIZE{
            let mut but1;
            if i >= self.column_size{
                but1=Button::new(0, 0, 0, 0, "PUT");
            }
            else{
                but1=Button::new((130+(i*650/self.column_size)).try_into().unwrap(), 20, ((650/self.column_size)-10).try_into().unwrap(), 50, "PUT");
            }
            but1.set_color(Color::from_u32(BG_ACTIVE));
            but1.set_frame(FrameType::RFlatBox);
            but1.set_label_color(Color::White);
            but1.clear_visible_focus();
            but1.set_selection_color(Color::from_u32(BG_COLOR));
            let (s1, _r1) = app::channel::<String>();
            let str_value = format!("{}", i+1); 
            but1.emit(s1,str_value.to_string());
            self.buttons.push(but1);
        }
        let mut coin_radius:i32=((650/self.column_size)).try_into().unwrap();
        coin_radius=coin_radius-40;
        if coin_radius>55{
            coin_radius=55;
        }
        if coin_radius<10{
            coin_radius=10;
        }
        for row in 0..MAX_SIZE{
            for column in 0..MAX_SIZE{
                if row>=self.row_size || column>=self.column_size{
                    let mut circle=Frame::new(0, 0, 0, 0, "");
                    circle.set_frame(FrameType::OvalBox);
                    circle.set_color(Color::from_u32(BG_COLOR));
                    self.state[row].push((circle,"EMPTY".to_string()));
                    continue;
                }
                if row==0{
                    let mut circle=Frame::new((150+column*(650/self.column_size)).try_into().unwrap(), 90, coin_radius, coin_radius, "");
                    circle.set_frame(FrameType::OvalBox);
                    circle.set_color(Color::from_u32(BG_COLOR));
                    self.state[row].push((circle,"EMPTY".to_string()));
                }
                else{
                    let mut circle=Frame::new((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+20)*(row) as i32, coin_radius, coin_radius, "");
                    circle.set_frame(FrameType::OvalBox);
                    circle.set_color(Color::from_u32(BG_COLOR));
                    self.state[row].push((circle,"EMPTY".to_string()));
                }
            }
        }
        self.label.set_label("RED PLAYER IS ON THE MOVE");
        self.label.set_label_size(30);
        self.label.set_label_color(Color::from_u32(BG_ACTIVE));
    }
    pub fn is_move_valid(&mut self,column:i32)->i32{
        let mut row_place=self.row_size+1;
        for row in (0..self.row_size).rev(){
            if self.state[row][(column-1) as usize].1=="EMPTY"{
                row_place=row;
                break;
            }
        }
        return row_place as i32;
    }
  
    pub fn place_coin(&mut self,column:i32){
        let row_place=self.is_move_valid(column);
        if row_place==(self.row_size+1) as i32 || self.winner!="EMPTY"{
            return;
        }
        if self.player=="RED"{
            self.state[row_place as usize][(column-1) as usize].0.set_color(Color::from_u32(COIN_RED));
            self.state[row_place as usize][(column-1) as usize].1="RED".to_string();
            self.change_player("YELLOW".to_string());
        }
        else{
            self.state[row_place as usize][(column-1) as usize].0.set_color(Color::from_u32(COIN_YELLOW));
            self.state[row_place as usize][(column-1) as usize].1="YELLOW".to_string();
            self.change_player("RED".to_string());
        }

        play_coin_sound();
        self.check_winner();
    }
    pub fn save_game(&mut self){
        if self.winner!="EMPTY"{
            self.label.set_label("You can't save finished game!");
            return;
        }
        let date = Local::now();
        let file = FileDialog::new()
        .set_directory("/")
        .add_filter("text", &["txt"])
        .set_file_name(&format!("{}{}",date.format("%Y-%m-%d-%H-%M-%S").to_string()," save-game.txt"))
        .save_file();
        let mut file = File::create(file.unwrap().as_path().display().to_string()).expect("create failed");
        file.write_all(format!("{}\n",self.player).to_string().as_bytes()).expect("write failed");
        file.write_all(format!("{}\n",self.row_size).to_string().as_bytes()).expect("write failed");
        file.write_all(format!("{}\n",self.column_size).to_string().as_bytes()).expect("write failed");
        for row in 0..self.row_size{
            for column in 0..self.column_size{
                file.write_all(format!("{} ",self.state[row as usize][column as usize].1).to_string().as_bytes()).expect("write failed");
            }
            file.write_all("\n".to_string().as_bytes()).expect("write failed");
        }
    }
    pub fn pick_save_game(&mut self)->String
    {
        let file = FileDialog::new()
        .add_filter("text", &["txt"])
        .set_directory("/")
        .pick_file(); 
        match file {
            Some(value) => {return value.display().to_string();}
            None => {return "NOT FOUND".to_string();}
        }
    }
    pub fn update_buttons(&mut self){
        for i in 0..MAX_SIZE{
            if i<self.column_size{
                self.buttons[i].resize((130+(i*650/self.column_size)).try_into().unwrap(), 20, ((650/self.column_size)-10).try_into().unwrap(),50);
                self.buttons[i].set_label("PUT");
            }
            else{
                self.buttons[i].resize(0,0,0,0);
                self.buttons[i].set_label("");
            }
        }
    }
    pub fn load_save_game(&mut self,data:Vec<String>){
        self.change_player(data[0].to_string());
        self.row_size=(data[1].parse::<i32>().unwrap()) as usize;
        self.column_size=(data[2].parse::<i32>().unwrap()) as usize;
        self.update_buttons();
        self.restart_game();
        let mut coin_radius:i32=((650/self.column_size)).try_into().unwrap();
        coin_radius=coin_radius-40;
        if coin_radius>55{
            coin_radius=55;
        }
        if coin_radius<10{
            coin_radius=10;
        }
        for row in 0..self.row_size{
            let lines: Split<&str> = data[row+3].split(" ");
            let mut row_data: Vec<String> = Vec::new();

            for line in lines{
                row_data.push(line.to_string());
            }

            for column in 0..self.column_size{
                self.state[row][column].1=row_data[column].to_string();
                if row_data[column]=="EMPTY"{
                    self.state[row][column].0.set_color(Color::from_u32(BG_COLOR));
                    if row==0{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90, coin_radius, coin_radius);
                    }
                    else{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+20)*(row) as i32, coin_radius, coin_radius);
                    }
                }
                if row_data[column]=="RED"{
                    self.state[row][column].0.set_color(Color::from_u32(COIN_RED));
                    if row==0{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90, coin_radius, coin_radius);
                    }
                    else{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+20)*(row) as i32, coin_radius, coin_radius);
                    }
                }
                if row_data[column]=="YELLOW"{
                    self.state[row][column].0.set_color(Color::from_u32(COIN_YELLOW));
                    if row==0{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90, coin_radius, coin_radius);
                    }
                    else{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+20)*(row) as i32, coin_radius, coin_radius);
                    }
                }
            }
        }
        for row in 0..MAX_SIZE{
            for column in 0..MAX_SIZE{
                if row>=self.row_size || column>=self.column_size{
                    self.state[row][column].1="EMPTY".to_string();
                    self.state[row][column].0.resize(0,0,0,0);
                    self.state[row][column].0.set_color(Color::from_u32(BG_COLOR));
                }   
            }
        }
        self.winner="EMPTY".to_string();
    }
}