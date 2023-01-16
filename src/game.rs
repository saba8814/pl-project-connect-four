use fltk::{app, button::Button, frame::Frame, prelude::*,enums::*};
use std::fs::File;
use std::io::Write;
use chrono::Local;
use rfd::FileDialog;
use std::str::Split;
use crate::constants::MAX_SIZE;
use crate::constants::BG_ACTIVE;
use crate::constants::BG_COLOR;
use crate::constants::COIN_RED;
use crate::constants::COIN_YELLOW;
use crate::static_ui_ux::play_coin_sound;
pub struct Game{
    pub player: String,
    pub row_size: usize,
    pub column_size: usize,
    pub state: Vec<Vec<(Frame,String)>>,
    pub history_of_moves: Vec<i32>,
    pub label: Frame,
    pub red_moves: Frame,
    pub yellow_moves: Frame,
    pub winner: String,
    pub buttons: Vec<Button>
}

impl Game{
    pub fn new(rows:i32,columns:i32)->Game{
        Game{
            player:"RED".to_string(),
            row_size: rows as usize,
            column_size: columns as usize,
            state: (0..MAX_SIZE).map(|_| Vec::new()).collect(),
            label: Frame::new(260, 540, 400, 50, ""),
            red_moves: Frame::new(20, 435, 80, 80, "RED"),
            yellow_moves: Frame::new(20, 505, 80, 80, "YELLOW"),
            history_of_moves: Vec::new(),
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
        self.red_moves.set_label("RED");
        self.yellow_moves.set_label("YELLOW");
        self.update_buttons();
        self.clear_board();
        self.label.set_label_color(Color::from_u32(BG_ACTIVE));
        self.history_of_moves.clear();
        let coin_radius:i32=self.calculate_coin_radius();
        for row in 0..self.row_size{
            for column in 0..self.column_size{
                if row==0{
                    self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90, coin_radius, coin_radius);
                }
                else{
                    self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+12)*(row) as i32, coin_radius, coin_radius);
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
        self.red_moves.set_label_color(Color::from_u32(COIN_RED));
        self.yellow_moves.set_label_color(Color::from_u32(COIN_YELLOW));
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
        let coin_radius:i32=self.calculate_coin_radius();
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
                    let mut circle=Frame::new((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+12)*(row) as i32, coin_radius, coin_radius, "");
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
        self.history_of_moves.push(column);       
        self.label.set_label_color(Color::from_u32(BG_ACTIVE));
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
        self.update_moves();
        self.check_winner();
    }

    pub fn update_moves(&mut self) {
        let mut red_moves_string: String = "".to_string();
        let mut yellow_moves_string: String = "".to_string();
        let mut yellow_counter: i32 = 0;
        let mut red_counter: i32 = 0;
        for i in 0..self.history_of_moves.len() {
            if i % 2 == 0 {
                red_counter+=1;
                red_moves_string = format!("{}{}", red_moves_string, self.history_of_moves[i].to_string());
                red_moves_string = format!("{}{}", red_moves_string, " ".to_string());
                if red_counter == 5 {
                    red_counter = 0;
                    red_moves_string = format!("{}{}", red_moves_string, "\n".to_string());
                }
            }
            else {
                yellow_counter += 1;
                yellow_moves_string = format!("{}{}", yellow_moves_string, self.history_of_moves[i].to_string());
                yellow_moves_string = format!("{}{}", yellow_moves_string, " ".to_string());
                if yellow_counter == 5 {
                    yellow_counter = 0;
                    yellow_moves_string = format!("{}{}", yellow_moves_string, "\n".to_string());
                }
            }
        }
        red_moves_string = format!("{} {}", "RED\n".to_string(), red_moves_string);
        yellow_moves_string = format!("{} {}", "YELLOW\n".to_string(), yellow_moves_string);
        self.red_moves.set_label(&red_moves_string);
        self.yellow_moves.set_label(&yellow_moves_string);
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
        match file {
            Some(value) => {
                let mut file = File::create(value.as_path().display().to_string()).expect("create failed");
                file.write_all(format!("{}\n",self.player).to_string().as_bytes()).expect("write failed");
                file.write_all(format!("{}\n",self.row_size).to_string().as_bytes()).expect("write failed");
                file.write_all(format!("{}\n",self.column_size).to_string().as_bytes()).expect("write failed");
                for i in &self.history_of_moves {
                    // iterate by-value
                    let i:&i32 = i; // elements are values
                    file.write_all(format!("{} ",i).to_string().as_bytes()).expect("write failed");
                }
                file.write_all("\n".to_string().as_bytes()).expect("write failed");

                for row in 0..self.row_size{
                    for column in 0..self.column_size{
                        file.write_all(format!("{} ",self.state[row as usize][column as usize].1).to_string().as_bytes()).expect("write failed");
                    }
                    file.write_all("\n".to_string().as_bytes()).expect("write failed");
                }
            }
            None => {return;}
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
        self.restart_game();
        self.change_player(data[0].to_string());
        self.row_size=(data[1].parse::<i32>().unwrap()) as usize;
        self.column_size=(data[2].parse::<i32>().unwrap()) as usize;
        self.history_of_moves.clear();
        for token in data[3].split_whitespace(){
            self.history_of_moves.push(token.parse::<i32>().unwrap());
         }
        self.update_buttons();
        let coin_radius:i32=self.calculate_coin_radius();
        for row in 0..self.row_size{
            let lines: Split<&str> = data[row+4].split(" ");
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
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+12)*(row) as i32, coin_radius, coin_radius);
                    }
                }
                if row_data[column]=="RED"{
                    self.state[row][column].0.set_color(Color::from_u32(COIN_RED));
                    if row==0{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90, coin_radius, coin_radius);
                    }
                    else{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+12)*(row) as i32, coin_radius, coin_radius);
                    }
                }
                if row_data[column]=="YELLOW"{
                    self.state[row][column].0.set_color(Color::from_u32(COIN_YELLOW));
                    if row==0{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90, coin_radius, coin_radius);
                    }
                    else{
                        self.state[row][column].0.resize((150+column*(650/self.column_size)).try_into().unwrap(), 90+(coin_radius+12)*(row) as i32, coin_radius, coin_radius);
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
        self.update_moves();
    }

    pub fn calculate_coin_radius(&mut self)->i32{
        let mut coin_radius:i32=((650/((self.column_size+self.row_size)/2))).try_into().unwrap();
        coin_radius=coin_radius-35;
        if coin_radius>55{
            coin_radius=55;
        }
        if coin_radius<10{
            coin_radius=10;
        }

        return coin_radius;
    }
}