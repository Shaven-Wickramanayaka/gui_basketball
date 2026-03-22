use eframe::egui;
use serde::Deserialize;
use reqwest::header::AUTHORIZATION;
use chrono::{DateTime,Utc};    
use std::time::SystemTime;
use std::collections::HashMap;

#[derive(Deserialize)]
struct AllData{
    data: Vec<Stats>
}
#[derive(Deserialize)]
struct Stats{
    home_team: HomeTeam,
    visitor_team: VisitorTeam,
    home_team_score: u64,
    visitor_team_score:u64,
}
#[derive(Deserialize)]
struct HomeTeam{
    full_name: String,
    abbreviation: String,
}
#[derive(Deserialize)]
struct VisitorTeam{
    full_name: String,
    abbreviation: String,   
}
// fn main() {
//     println!("NBA Games Today\n");
//     let url = format!("https://api.balldontlie.io/nba/v1/games");
//     let client = reqwest::blocking::Client::new();
//     let now = SystemTime::now();
//     let now:    DateTime<Utc> = now.into();
//     let now = now.to_rfc3339();
//     let mut params = HashMap::new();
//     params.insert("dates[]", now);
//     let data: AllData = client.get(url).header(AUTHORIZATION, "65ab2ce7-ad41-43bf-a304-0f7ec82a392f").query(&params).send().expect("Error getting api").json().expect("Error parsing Json");

//     for game in data.data{
//             println!("{}(Home Team) vs {}(Away Team)", game.home_team.full_name, game.visitor_team.full_name);
//             println!("{}:{} | {}:{}",game.home_team.abbreviation, game.home_team_score, game.visitor_team.abbreviation, game.visitor_team_score);
//             println!("\n");
//     }
// }
// struct TodoApp {
//         todos: Vec<Todo>,
//     new_todo_text: String,
// }
struct BasketballGamesToday{
    games: Vec<Stats>,
}
impl BasketballGamesToday {
    fn new(_cc: &eframe::CreationContext<'_>, games_sent: Vec<Stats>) -> Self {
        Self {
            games: games_sent
        }
    }
}
// impl TodoApp {
//     fn new(_cc: &eframe::CreationContext<'_>) -> Self {
//         Self {
//             todos:
//         }
//     }
//     fn add_games(_cc:&eframe::CreationContext<'_>, games: Vec<Stats>) ->{
//         Self{
//             todos
//         }
//     }
// }

impl eframe::App for BasketballGamesToday {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Basketball Games Today");
            ui.add_space(10.0);
            for (_i, game) in self.games.iter_mut().enumerate() {
                let title_text = format!("{} vs {}", &game.home_team.full_name, &game.visitor_team.full_name);
                let score_text = format!("{} ({})   {} ({})", &game.home_team_score, &game.home_team.abbreviation, &game.visitor_team_score, &game.visitor_team.abbreviation);
                ui.label(title_text);
                ui.add_space(4.0);
                ui.label(score_text);
                ui.add_space(8.0);
            }
        });
    }
}

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    let url = format!("https://api.balldontlie.io/nba/v1/games");
    let client = reqwest::blocking::Client::new();
    let now = SystemTime::now();
    let now:    DateTime<Utc> = now.into();
    let now = now.to_rfc3339();
    let mut params = HashMap::new();
    params.insert("dates[]", now);
    let mut games_list = Vec::<Stats>::new();
    let data: AllData = client.get(url).header(AUTHORIZATION, "65ab2ce7-ad41-43bf-a304-0f7ec82a392f").query(&params).send().expect("Error getting api").json().expect("Error parsing Json");
    for game in data.data{
        games_list.push(game);
    }

    eframe::run_native(
        "Basketball Scores",
        native_options,
        Box::new(|cc| Ok(Box::new(BasketballGamesToday::new(cc, games_list)))),
    )
}