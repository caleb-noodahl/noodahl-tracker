use std::{thread, time, collections::HashMap};
use active_win_pos_rs::get_active_window;
use log::{info};
use itertools::Itertools;

extern crate env_logger;


fn main() {
    env_logger::init();
    let delay = time::Duration::from_secs(3);
    let mut fm = HashMap::new();
    fm.insert(String::from("unknown"), Focus { process:String::from("unknown"), duration: 0, titles: Vec::new() });

    loop {
        match get_active_window() {
            Ok(active_window) => {
                if fm.contains_key(&active_window.process_name) {
                    let val: &mut Focus = fm.get_mut(&active_window.process_name).unwrap();
                    val.add_title(active_window.title);
                    val.add_duration(delay.as_secs());
                }else {
                    let mut titles = Vec::new();
                    titles.insert(0, active_window.title);
                    
                    let focus = Focus{
                        process: active_window.process_name,
                        duration: delay.as_secs(),
                        titles: titles
                    };
                    fm.insert(focus.process.clone(), focus);
                }
                info!("{:#?}", fm);
            },
            Err(()) => {
                let val: &mut Focus = fm.get_mut(&String::from("unknown")).unwrap();
                val.add_duration(delay.as_secs());
            }
        }
        
        thread::sleep(delay);
    }
}


#[derive(Debug)]
struct Focus {
    process: String,
    duration: u64,
    titles: Vec<String>
}

impl Focus {
    pub fn add_title(&mut self, title: String) {
        self.titles.insert(self.titles.len(), title);
        self.titles = self.titles.clone().into_iter().unique().collect();
    }
    
    pub fn add_duration(&mut self, duration: u64) {
        self.duration += duration;
    }
}