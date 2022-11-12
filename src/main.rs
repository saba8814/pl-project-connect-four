use fltk::{app, button::Button, frame::Frame, prelude::*, window::Window,enums::*,image::PngImage};
use std::io::{BufReader};
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::Write;
use chrono::Local;
use rfd::FileDialog;
use std::io::Read;
use std::str::Split;

const BG_COLOR: u32 = 0x121212;
const BURCH_BLUE: u32 = 0x1e1e1e;
const BOARD_COLOR: u32 = 0x1e1e1e;
const COIN_RED: u32 = 0xcf6679;
const COIN_YELLOW: u32 = 0x03dac6;

fn play_coin_sound(){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("coin_place.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_millis(150));
}
fn play_win_sound(){
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open("winner.mp3").unwrap());
    let source = Decoder::new(file).unwrap();
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(7));
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
    load_button.set_selection_color(Color::from_u32(BG_COLOR));
    let mut save_button = Button::new(20, 155, 95, 50, "SAVE");
    let mut restart_button = Button::new(20, 220, 95, 50, "RESTART");

    //Playing Buttons
    let mut place_button1 = Button::new(130, 20, 80, 50, "PLACE");
    let mut place_button2  = Button::new(225, 20, 80, 50, "PLACE");
    let mut place_button3  = Button::new(320, 20, 80, 50, "PLACE");
    let mut place_button4  = Button::new(415, 20, 80, 50, "PLACE");
    let mut place_button5  = Button::new(510, 20, 80, 50, "PLACE");
    let mut place_button6  = Button::new(605, 20, 80, 50, "PLACE");
    let mut place_button7  = Button::new(700, 20, 80, 50, "PLACE");


    //Game Control Buttons Style
    load_button.set_color(Color::from_u32(BURCH_BLUE));
    save_button.set_color(Color::from_u32(BURCH_BLUE));
    restart_button.set_color(Color::from_u32(BURCH_BLUE));
    load_button.clear_visible_focus();
    save_button.clear_visible_focus();
    restart_button.clear_visible_focus();
    load_button.set_frame(FrameType::RFlatBox);
    save_button.set_frame(FrameType::RFlatBox);
    restart_button.set_frame(FrameType::RFlatBox);
    load_button.set_label_color(Color::White);
    save_button.set_label_color(Color::White);
    restart_button.set_label_color(Color::White);


    //Sending and recieving channels
    let (s1, _r1) = app::channel::<String>();
    let (s2, _r1) = app::channel::<String>();
    let (s3, _r1) = app::channel::<String>();
    let (s4, _r1) = app::channel::<String>();
    let (s5, _r1) = app::channel::<String>();
    let (s6, _r1) = app::channel::<String>();
    let (s7, _r1) = app::channel::<String>();
    let (s8, _r1) = app::channel::<String>();
    let (s9, _r1) = app::channel::<String>();
    let (s10, _r1) = app::channel::<String>();

    //Playing buttons emits
    place_button1.emit(s1,"1".to_string());
    place_button2.emit(s2,"2".to_string());
    place_button3.emit(s3,"3".to_string());
    place_button4.emit(s4,"4".to_string());
    place_button5.emit(s5,"5".to_string());
    place_button6.emit(s6,"6".to_string());
    place_button7.emit(s7,"7".to_string());
    //Game Control Buttons emits
    restart_button.emit(s8,"RESTART".to_string());
    load_button.emit(s9,"LOAD".to_string());
    save_button.emit(s10,"SAVE".to_string());

    //Playing Buttons style
    place_button1.set_color(Color::from_u32(BURCH_BLUE));
    place_button2.set_color(Color::from_u32(BURCH_BLUE));
    place_button3.set_color(Color::from_u32(BURCH_BLUE));
    place_button4.set_color(Color::from_u32(BURCH_BLUE));
    place_button5.set_color(Color::from_u32(BURCH_BLUE));
    place_button6.set_color(Color::from_u32(BURCH_BLUE));
    place_button7.set_color(Color::from_u32(BURCH_BLUE));
    place_button1.set_frame(FrameType::RFlatBox);
    place_button2.set_frame(FrameType::RFlatBox);
    place_button3.set_frame(FrameType::RFlatBox);
    place_button4.set_frame(FrameType::RFlatBox);
    place_button5.set_frame(FrameType::RFlatBox);
    place_button6.set_frame(FrameType::RFlatBox);
    place_button7.set_frame(FrameType::RFlatBox);
    place_button1.set_selection_color(Color::from_u32(BG_COLOR));
    place_button2.set_selection_color(Color::from_u32(BG_COLOR));
    place_button3.set_selection_color(Color::from_u32(BG_COLOR));
    place_button4.set_selection_color(Color::from_u32(BG_COLOR));
    place_button5.set_selection_color(Color::from_u32(BG_COLOR));
    place_button6.set_selection_color(Color::from_u32(BG_COLOR));
    place_button7.set_selection_color(Color::from_u32(BG_COLOR));
    place_button1.set_label_color(Color::White);
    place_button2.set_label_color(Color::White);
    place_button3.set_label_color(Color::White);
    place_button4.set_label_color(Color::White);
    place_button5.set_label_color(Color::White);
    place_button6.set_label_color(Color::White);
    place_button7.set_label_color(Color::White);
    place_button1.clear_visible_focus();
    place_button2.clear_visible_focus();
    place_button3.clear_visible_focus();
    place_button4.clear_visible_focus();
    place_button5.clear_visible_focus();
    place_button6.clear_visible_focus();
    place_button7.clear_visible_focus();

    window.set_color(Color::from_u32(BG_COLOR));
    window.make_resizable(true);
    //MAIN STARTS HERE
    draw_ui();
    let mut game = Game::new();
    game.start_game();
    window.end();
    window.show();
    while app.wait() {
        if let Some(msg) = _r1.recv() {
            if msg=="RESTART"{
                game.restart_game();
            }
            if msg=="LOAD"{
                let mut file = File::open(game.pick_save_game()).expect("error");
                let mut contents = String::new();
                file.read_to_string(&mut contents).expect("something went wrong reading the file");
                let lines: Split<&str> = contents.split("\n");
                let mut data: Vec<String> = Vec::new();

                for line in lines{
                    data.push(line.to_string());
                }
                game.load_save_game(data);
            }
            if msg=="SAVE"{
                game.save_game();
            }
            if (msg!="RESTART") && (msg!="LOAD") && (msg!="SAVE"){
                game.place_coin(msg.parse::<i32>().unwrap());
            }
            app::redraw();
        }
    }

}


struct Game{
    player: String,
    state: Vec<Vec<(Frame,String)>>,
    label: Frame,
    winner: String
}

impl Game{
    pub fn new()->Game{
        Game{
            player:"RED".to_string(),
            state: (0..6).map(|_| Vec::new()).collect(),
            label: Frame::new(260, 540, 400, 50, ""),
            winner: "EMPTY".to_string()
        }
    }
    pub fn restart_game(&mut self){
        for row in 0..6{
            for column in 0..7{
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
        for row in 0..3{
            for column in 0..4{
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
        for row in 0..3{
            for column in 3..7{
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
        for column in 0..7{
            for row in 0..3{
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
        for row in 0..6{
            for column in 0..4{
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
        for row in 0..6{
            for column in 0..7{
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
            //play_win_sound();
            return;
        }
        if self.check_vertical_win()!="EMPTY"{
            self.winner=self.check_vertical_win().to_string();
            let winner_string=self.check_vertical_win().to_string()+ " player wins!";
            self.label.set_label(&winner_string);
            //play_win_sound();
            return;
        }
        if self.check_horizontal_win()!="EMPTY"{
            self.winner=self.check_horizontal_win().to_string();
            let winner_string=self.check_horizontal_win().to_string()+ " player wins!";
            self.label.set_label(&winner_string);
            //play_win_sound();
            return;
        }
        if self.check_draw()!="EMPTY"{
            self.winner="DRAW".to_string();
            self.label.set_label("GAME FINISHED AS DRAW");
            return;
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
        if row_place==7 || self.winner!="EMPTY"{
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
        for row in 0..6{
            for column in 0..7{
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

        return file.unwrap().as_path().display().to_string();
    }
    pub fn load_save_game(&mut self,data:Vec<String>){
        self.restart_game();
        self.change_player(data[0].to_string());
        for row in 0..6{
            let lines: Split<&str> = data[row+1].split(" ");
            let mut row_data: Vec<String> = Vec::new();

            for line in lines{
                row_data.push(line.to_string());
            }

            for column in 0..7{
                self.state[row][column].1=row_data[column].to_string();
                if row_data[column]=="EMPTY"{
                    self.state[row][column].0.set_color(Color::from_u32(BG_COLOR));
                }
                if row_data[column]=="RED"{
                    self.state[row][column].0.set_color(Color::from_u32(COIN_RED));
                }
                if row_data[column]=="YELLOW"{
                    self.state[row][column].0.set_color(Color::from_u32(COIN_YELLOW));
                }
            }
        }
        self.winner="EMPTY".to_string();
    }
}