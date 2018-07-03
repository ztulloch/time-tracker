#![feature(assoc_unix_epoch)]
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use std::fs::OpenOptions;
use std::error::Error;
use std::path::Path;
use std::env;
extern crate getopts;
extern crate csv;
#[macro_use]
extern crate serde_derive;

use getopts::Options;

// Struct for entries in the project logging file
#[derive(Debug, Deserialize, Serialize)]
struct Unit {
    project_code: String,
    start_time: u64,
    end_time: u64,
    duration: u64,
}

// Struct for a running timer
#[derive(Debug, Deserialize, Serialize)]
struct Timer {
    start_time: u64,
    project_code: String,
}

// function that checks if a file exists
fn file_exists(filename: &str) -> bool {
    if Path::new(filename).exists() {
        return true;
    } else {
        return false;
    }
}

// Quick check to see if the timer is running
fn print_status () {
    if file_exists("timer.csv") {
        println!("Timer is running");
    } else {
        println!("There is no timer running");
    }
}

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} start/stop/status/hours [options]", program);
    print!("{}", opts.usage(&brief));
}

// initial print hours function just to test reading in log file
fn print_hours() -> Result<(), Box<Error>> {
    if file_exists("logger.csv") {
        // read start timer struct from timer file
        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .append(false)
            .open("logger.csv")
            .unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);
        for result in rdr.deserialize() {
            let unit: Unit = result?;
            if unit.duration>120 { // ignore anything less than 2 minutes
                println!("Project: {}       date {} duration {} hrs {} mins", unit.project_code, unit.start_time, unit.duration/60/60, unit.duration/60%60);
            }
        };
    } else {
        println!("Unable to find logfile.");
    }


    Ok(())

}

fn start_timer (project_code: &str) {
    // don't start a new timer if one is running
    if file_exists("timer.csv") {
        println!("Timer is already running");
    } else {
        // create csv writer and write start time and project code
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("timer.csv")
            .unwrap();
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);
        let current_time = SystemTime::now();
        let start_time = current_time.duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        wtr.serialize(Timer {
            start_time: start_time.as_secs(),
            project_code: project_code.to_string(),
        }).expect ("Error creating timer file");
        
        wtr.flush().expect("Error creating timer");
    }
}

fn stop_timer() -> Result<(), Box<Error>> {
    if file_exists("timer.csv") {
        // read start timer struct from timer file
        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .append(false)
            .open("timer.csv")
            .unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(file);
        for result in rdr.deserialize() {
            let timer: Timer = result?;

            // setup current time
            let current_time = SystemTime::now();
            let stop_time = current_time.duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            // calculate time difference
            let time_difference=stop_time.as_secs()-timer.start_time;

            // take timer data and add it to the log file along with the current time and the delta
            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open("logger.csv")
                .unwrap();
            let mut wtr = csv::WriterBuilder::new()
                .has_headers(false)
                .from_writer(file);

            wtr.serialize(Unit {
                project_code: timer.project_code,
                start_time: timer.start_time,
                end_time: stop_time.as_secs(),
                duration: time_difference,
            }).expect("Error writing logfile");
            
            wtr.flush().expect("Error writing logfile");
            // remove timer
            fs::remove_file("timer.csv").expect("Error deleting timer.");
        }
    } else {
        println!("There is no running timer to stop");
    }


    Ok(())

}

fn main() {
    // parse program arguments using getopts crate
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut project_code="default".to_string();
    let mut working_directory="default".to_string();

    let mut opts = Options::new();
    // -p PROJECT - project flag
    opts.optopt("p", "", "set project code.", "CODE");
    // -d PROJECT - project directory
    opts.optopt("d", "", "set working directory. Overrides $TIMERDIR", "CODE");
    // -h help - print usage
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    // h print usage
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    // p process project code
    if matches.opt_present("p") {
        match matches.opt_str("p") {
            Some(x) => project_code=x,
            None => println!("No project code specified, using default"),
        }
    }
    // d process working directory
    if matches.opt_present("d") {
        match matches.opt_str("d") {
            Some(x) => working_directory=x,
            None => println!("No working directory specified"),
        }
    }

    let command = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };
    
    if working_directory=="default" {
        let timerdir = match env::var("TIMERDIR") {
            Ok(val) => val,
            Err(_error) => "default".to_string(),
        };
        working_directory=timerdir;
    };

    if working_directory!="default" {
        let workdir = Path::new(&working_directory);
        if workdir.exists() {
            env::set_current_dir(workdir).expect("Error changing current working directory.");
        } else {
            println!("Working directory doesn't exist using current directory.");
            println!("No such directory: {}.", working_directory);
        };
        
    };
    
    if command=="start" {
        println! ("Starting timer...");
        start_timer (&project_code);
    } else if command=="stop" {
        println! ("Stopping timer...");
        stop_timer ().expect("Error stopping timer");
    } else if command=="status" {
        print_status ();
    } else if command=="hours" {
        print_hours().expect("Unable to parse log file");
    };
}

