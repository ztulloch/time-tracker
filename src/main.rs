use std::env;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
extern crate csv;
extern crate getopts;
#[macro_use]
extern crate serde_derive;

use getopts::Options;

extern crate chrono;
use chrono::prelude::*;

// Struct for entries in the project logging file
#[derive(Debug, Deserialize, Serialize)]
struct Unit {
    project_code: String,
    start_time: u64,
    end_time: u64,
    duration: u64,
    task: String,
}

// Struct for a running timer
#[derive(Debug, Deserialize, Serialize)]
struct Timer {
    start_time: u64,
    project_code: String,
    task: String,
}

// function that checks if a file exists
fn file_exists(filename: &str) -> bool {
    if Path::new(filename).exists() {
        return true;
    } else {
        return false;
    }
}

// Check to see if the timer is running
fn print_status() -> Result<(), Box<Error>> {
    if file_exists("timer.csv") {
        println!("Timer is running");
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
            let stop_time = current_time
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            // calculate time difference
            let time_difference = stop_time.as_secs() - timer.start_time;
            println!(
                "Timer: {} running {} hrs {} mins",
                timer.project_code,
                time_difference / 60 / 60,
                time_difference / 60 % 60
            );
        }
    } else {
        println!("There is no timer running");
    }
    // Also might be useful just to print what week we're on
    let naive_date_time = Utc::now().naive_utc();
    println!(
        "Week {} Month {} Day {} ",
        naive_date_time.iso_week().week(),
        naive_date_time.month(),
        naive_date_time.day()
    );

    Ok(())
}

// Print program usage
fn print_usage(program: &str, opts: Options) {
    let brief = format!(
        "Usage: {} start/stop/cancel/status/hours/weeks [options]",
        program
    );
    print!("{}", opts.usage(&brief));
}

// initial print hours function just to test reading in log file
// Just reads the log csv file and prints it out
fn print_hours(quash: bool) -> Result<(), Box<Error>> {
    if file_exists("logger.csv") {
        // read start timer struct from timer file
        let mut counter = 0;
        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .append(false)
            .open("logger.csv")
            .unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        for result in rdr.deserialize() {
            let unit: Unit = result?;
            if unit.duration > 120 || quash {
                // ignore anything less than 2 minutes
                let naive_datetime = NaiveDateTime::from_timestamp(unit.start_time as i64, 0);
                let datetime_again: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
                println!(
                    "Project: {} Task: {}      date {:?} duration {} hrs {} mins",
                    unit.project_code,
                    unit.task,
                    datetime_again,
                    unit.duration / 60 / 60,
                    unit.duration / 60 % 60
                );
                counter += unit.duration;
            }
        }
        println!(
            "Total is {} hours {} minutes.",
            counter / 60 / 60,
            counter / 60 % 60
        );
    } else {
        println!("Unable to Total. No logfile.");
    }

    Ok(())
}

// print hours tracked in terms of weeks
// need to extend to print on project basis
fn print_weeks(quash: bool) -> Result<(), Box<Error>> {
    if file_exists("logger.csv") {
        // read start timer struct from timer file
        let mut counter = 0;
        let mut week_pointer = 0;
        let file = OpenOptions::new()
            .read(true)
            .create(false)
            .append(false)
            .open("logger.csv")
            .unwrap();
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);
        for result in rdr.deserialize() {
            let unit: Unit = result?;
            if unit.duration > 120 || quash {
                // ignore anything less than 2 minutes
                let naive_datetime = NaiveDateTime::from_timestamp(unit.start_time as i64, 0);
                if week_pointer == naive_datetime.iso_week().week() {
                    counter += unit.duration;
                } else {
                    if counter != 0 {
                        println!(
                            "Week {} Hours {} hours {} minutes",
                            week_pointer,
                            counter / 60 / 60,
                            counter / 60 % 60
                        );
                    }
                    counter = unit.duration;
                    week_pointer = naive_datetime.iso_week().week();
                }
            }
        }
        println!(
            "Week {} Hours {} hours {} minutes",
            week_pointer,
            counter / 60 / 60,
            counter / 60 % 60
        );
    } else {
        println!("Unable to Total. No logfile.");
    }

    Ok(())
}

// start timer - just writes the current time in seconds to a csv file
fn start_timer(project_code: &str, task: &str) {
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
        let start_time = current_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");

        wtr.serialize(Timer {
            start_time: start_time.as_secs(),
            project_code: project_code.to_string(),
            task: task.to_string(),
        }).expect("Error creating timer file");

        wtr.flush().expect("Error creating timer");

        println!("Starting timer for project {}...", project_code);
    }
}

// reads timer file and writes results to log file
fn stop_timer(quash: bool) -> Result<(), Box<Error>> {
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
            let stop_time = current_time
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards");

            // calculate time difference
            let time_difference = stop_time.as_secs() - timer.start_time;
            if time_difference > 120 || quash {
                println!(
                    "Timer has been running {} hrs {} mins",
                    time_difference / 60 / 60,
                    time_difference / 60 % 60
                );
                // use headers if it's the first time writing the logger file
                let existence = file_exists("logger.csv");
                // take timer data and add it to the log file along with the current time and the delta
                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open("logger.csv")
                    .unwrap();
                if existence {
                    println!("Building write without any headers");
                    let mut wtr = csv::WriterBuilder::new()
                        .has_headers(false)
                        .from_writer(file);

                    wtr.serialize(Unit {
                        project_code: timer.project_code,
                        start_time: timer.start_time,
                        end_time: stop_time.as_secs(),
                        duration: time_difference,
                        task: timer.task,
                    }).expect("Error writing logfile");

                    wtr.flush().expect("Error writing logfile");
                } else {
                    println!("Building writer with headers");
                    let mut wtr = csv::WriterBuilder::new()
                        .has_headers(true)
                        .from_writer(file);

                    wtr.serialize(Unit {
                        project_code: timer.project_code,
                        start_time: timer.start_time,
                        end_time: stop_time.as_secs(),
                        duration: time_difference,
                        task: timer.task,
                    }).expect("Error writing logfile");

                    wtr.flush().expect("Error writing logfile");
                }
            } else {
                println!("Timer has been running for less than 2 minutes, discarding...");
            }
            // remove timer
            fs::remove_file("timer.csv").expect("Error deleting timer.");
        }
    } else {
        println!("There is no running timer to stop");
    }

    Ok(())
}
// reads timer file and writes results to log file
fn cancel_timer() -> Result<(), Box<Error>> {
    if file_exists("timer.csv") {
        // remove timer
        fs::remove_file("timer.csv").expect("Error deleting timer.");
        println!("Timer cancelled.");
    } else {
        println!("There is no running timer to cancel.");
    }
    Ok(())
}

fn main() {
    // parse program arguments using getopts crate
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut project_code = "default".to_string();
    let mut task = String::new(); //"default".to_string();
    let mut working_directory = "default".to_string();
    let mut quash = false; // override 120 second limit on timers
    let mut opts = Options::new();
    // -p PROJECT - project flag
    opts.optopt("p", "", "set user definable project code.", "CODE");
    // -d PROJECT - project directory
    opts.optopt(
        "d",
        "",
        "set working directory. Overrides $TIMERDIR environment variable.",
        "CODE",
    );
    // -t TASK - task
    opts.optopt(
        "t",
        "",
        "set project task. Short summary of task undertaken.",
        "CODE",
    );
    // -q  - quash
    opts.optflag(
        "q",
        "quash",
        "Overrides 2 minute limit. Useful for testing.",
    );
    // -h help - print usage
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    // h print usage
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    // p process project code
    if matches.opt_present("p") {
        match matches.opt_str("p") {
            Some(x) => project_code = x,
            None => println!("No project code specified, using default"),
        }
    }
    // d process working directory
    if matches.opt_present("d") {
        match matches.opt_str("d") {
            Some(x) => working_directory = x,
            None => println!("No working directory specified"),
        }
    }
    // t process task
    if matches.opt_present("t") {
        match matches.opt_str("t") {
            Some(x) => task = x,
            None => println!("No task specififed, using default"),
        }
    }
    // o override
    if matches.opt_present("q") {
        quash = true;
    }

    let command = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    if working_directory == "default" {
        let timerdir = match env::var("TIMERDIR") {
            Ok(val) => val,
            Err(_error) => "default".to_string(),
        };
        working_directory = timerdir;
    };

    if working_directory != "default" {
        let workdir = Path::new(&working_directory);
        if workdir.exists() {
            env::set_current_dir(workdir).expect("Error changing current working directory.");
        } else {
            println!("Working directory doesn't exist using current directory.");
            println!("No such directory: {}.", working_directory);
        };
    };

    if command == "start" {
        // check task status and request if not available
        let mut input = String::new();
        let mut done = false;
        while !done {
            if task != "" {
                break;
            }
            println!("No task specified. What will you be working on?");
            println!("Please keep it brief and don't punctuate.");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            task = input.trim().to_string();
            if task.contains(",") {
                println!("Input string contains illegal character {}", task);
                task.drain(..);
                input.drain(..);
            } else {
                done = true;
            }
        }
        // check project code. If unset request from user
        done = false;
        input.drain(..);
        while !done {
            if project_code != "default" {
                break;
            }
            println!("No project specified. Please specify a one word project.");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            project_code = input.trim().to_string();
            if project_code.contains(",") {
                println!("Input string contains illegal character {}", project_code);
                project_code.drain(..);
                input.drain(..);
            } else {
                done = true;
            }
        }

        start_timer(&project_code, &task);
    } else if command == "stop" {
        println!("Stopping timer...");
        stop_timer(quash).expect("Error stopping timer");
    } else if command == "cancel" {
        cancel_timer().expect("Error cancelling timer");
    } else if command == "status" {
        print_status().expect("Unable to parse log file");
    } else if command == "hours" {
        print_hours(quash).expect("Unable to parse log file");
    } else if command == "weeks" {
        print_weeks(quash).expect("Unable to parse log file");
    } else {
        println!("Unknown option `{}`.", command);
        println!("");
        print_usage(&program, opts);
    };
}
