extern crate msgbox;
use std::env;
use std::thread;
use std::time::Duration;
use std::process::Command;

// CLI tool to schedule message reminders

/*
 * If invoked as:
 * heybud "message" num
 * scehdule a task num minutes from now that invokes heybud --alert "message"
 */
fn main() {
    let args: Vec<String> = env::args().collect();

    match parse_args(&args) {
        Ok(command) => match command {
            Cmd::EnqueueReminder(msg, mins_from_now) => enqueue_reminder(&msg, mins_from_now),
            Cmd::ShowMessage(msg, sleep_for) => show_message(&msg, sleep_for),
        },
        Err(_) => {
            eprintln!("Hey bud, can you please follow the friggin usage pattern: `heybud 'my reminder mesage' <number>`");
        },
    }
}

enum Cmd {
    EnqueueReminder(String, u64),
    ShowMessage(String, u64),
}

fn parse_args(args: &Vec<String>) -> Result<Cmd, String> {
    if args.len() != 3 && args.len() != 4 {
        return Err(String::from("incorrect use"));
    }
    
    if args.len() == 4 && args[1] == "--alert" {
        Ok(Cmd::ShowMessage(args[2].clone(), args[3].parse::<u64>().expect("invalid usage")))
    } else {
        let remind_in_mins = args[2].parse::<u64>().expect("invalid usage");
        Ok(Cmd::EnqueueReminder(args[1].clone(), remind_in_mins))
    }
}

fn show_message(message: &String, minutes: u64) {
    thread::sleep(Duration::from_secs(minutes * 60));
    match msgbox::create("Hey bud!", message.as_str(), msgbox::IconType::Info) {
        Err(_) => {},
        Ok(_) => {},
    };
}

fn enqueue_reminder(message: &String, minutes: u64) {
    // fork a child proc that uses our binary to sleep for n minutes and then show_message
    let mut cmd = Command::new("heybud");
    cmd.arg("--alert");
    cmd.arg(message);
    cmd.arg(minutes.to_string());

    match cmd.spawn() {
        Ok(_) => println!("Hey bud, I'll remind you in {} minutes", minutes),
        Err(e) => println!("Hey bud, I'm sorry I ran into an error: {:?}", e),
    };
}
