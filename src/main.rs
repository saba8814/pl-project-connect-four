use fltk::{app, frame::Frame, prelude::*, window::Window,enums::*,image::PngImage,input::IntInput};
use std::fs::File;
use std::io::Read;
use std::str::Split;
mod game;
mod constants;
mod static_ui_ux;

fn main() {
    //Variables needed at runtime
    let app = app::App::default();
    let logo: PngImage = PngImage::load(&std::path::Path::new("logo.png")).unwrap();
    let mut window = Window::new(100, 100, 800, 600, "Connect Four || IBU IT || PL Project");
    let mut input_rows = IntInput::new(20,295,95,50,"");
    let mut input_columns = IntInput::new(20,365,95,50,"");
    let mut input_rows_label =Frame::new(20, 280, 95, 15, "# of ROWS");
    let mut input_column_label =Frame::new(20, 350, 95, 15, "# of COLUMNS");
    let mut game = game::Game::new(6,7);
    let (_s1, _r1) = app::channel::<String>();
    
    //Window styling
    window.set_icon(Some(logo));
    window.set_color(Color::from_u32(constants::BG_COLOR));

    //input rows styling and value setting
    input_rows.set_text_color(Color::White);
    input_rows.set_color(Color::from_u32(constants::BG_ACTIVE));
    input_rows.set_frame(FrameType::RFlatBox);
    input_rows_label.set_label_color(Color::White);
    input_rows.set_value("6");

    //input columns styling and value setting
    input_column_label.set_label_color(Color::White);
    input_columns.set_frame(FrameType::RFlatBox);
    input_columns.set_color(Color::from_u32(constants::BG_ACTIVE));
    input_columns.set_text_color(Color::White);
    input_columns.set_value("7");

    //Drawing static UI
    static_ui_ux::draw_ui();

    //starting game || actal MAIN begins
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
                    if diff>2 || rows<6 || columns<6 || rows as usize>constants::MAX_SIZE || columns as usize>constants::MAX_SIZE{
                        input_rows.set_value(&game.row_size.to_string());
                        input_columns.set_value(&game.column_size.to_string());
                        game.label.set_label_color(Color::from_u32(constants::COIN_RED));
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
                    game.label.set_label_color(Color::from_u32(constants::COIN_RED));
                }
                if game.winner=="YELLOW"{
                    game.label.set_label_color(Color::from_u32(constants::COIN_YELLOW));
                }
            }
            app::redraw(); //used to redraw game only after every change
        }
    }

}


