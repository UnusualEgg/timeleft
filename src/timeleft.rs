#[cfg(not(feature = "for_web"))]
pub use timeleft::*;
#[cfg(not(feature = "for_web"))]
mod timeleft {
    pub use csv::Reader;
    pub use libc::{alarm, pause, signal};
    pub use libtimeleft::{get_time_left, set_csv, CSVTime, DrawFn, DrawType};
    pub use std::env::args;

    ///positive is up and right
    fn move_relative(x: i32, y: i32) {
        //up down right left ABCD
        if x.is_positive() {
            print!("\x1b[{}C", x);
        } else if x.is_negative() {
            print!("\x1b[{}D", x.abs());
        }
        if y.is_positive() {
            print!("\x1b[{}A", y);
        } else if y.is_negative() {
            print!("\x1b[{}B", y.abs());
        }
    }
    #[inline]
    fn move_to(x: u32) {
        print!("\x1b[{}G", x);
    }
    #[inline]
    fn save_pos() {
        //save cursor pos
        print!("\x1b7"); //7 is save
    }
    #[inline]
    fn restore_pos() {
        //restore the cursor positon to the one that was saved
        print!("\x1b8"); //8 is restore
    }
    fn default_draw(
        draw_type: DrawType,
        redraw_all: bool,
        class: &String,
        time_left: &String,
        current_time: &String,
    ) {
        match draw_type {
            DrawType::In => {
                if redraw_all {
                    //move to column 17 to make them all lined up
                    print!("Class:");
                    move_to(17);
                    println!("{}", class);

                    print!("TimeLeft:");
                    move_to(17);
                    save_pos();
                    println!("{}", time_left);
                    // stdout.flush().unwrap();
                    print!("Current Time:");
                    move_to(17);
                    println!("{}", current_time);
                } else {
                    //else update
                    restore_pos();
                    move_relative(0, 2);
                    print!("{}", time_left);
                    restore_pos();
                    move_relative(0, 1);
                    println!("{}", current_time);
                }
            }
            DrawType::Before => {
                if redraw_all {
                    //move to column 17 to make them all lined up
                    print!("Next Class:");
                    move_to(17);
                    println!("{}", class);

                    print!("Time Till Start:");
                    move_to(17);
                    save_pos();
                    println!("{}", time_left);
                    // stdout.flush().unwrap();
                    print!("Current Time:");
                    move_to(17);
                    println!("{}", current_time);
                } else {
                    //else update
                    restore_pos();
                    move_relative(0, 2);
                    print!("{}", time_left);
                    restore_pos();
                    move_relative(0, 1);
                    println!("{}", current_time);
                }
            }
            DrawType::Out => {
                println!("Outta school B)");
            }
        }
    }
    fn sig_handler(_: i32) {
        get_time_left(&(default_draw as DrawFn));
    }
}
fn main() {
    const TIME_FN: &str = "times.csv";
    let home = std::env::var("HOME").expect("Couldn't get $HOME var");
    let csvpath: String = format!("{home}/{fn}",home=home,fn=TIME_FN);
    // CSVPATH.push_str();//   "~/times.csv"

    let f_result = Reader::from_path(csvpath.clone());
    let mut f;
    match f_result {
        Err(e) => {
            println!("HOME FOLDER:{}", home);
            panic!("\nPlease put times.csv in your home folder (/Users/[your username]/times.csv)\nTo get to there:\nPress ⌘+Shift+G in finder. Type ~ and press enter.\n{}",e);
        }
        Ok(v) => {
            f = v;
        }
    }

    // let csvtimes_local = Vec::new();
    //make csvtimes with all strings
    let mut csvs = Vec::new();
    for i in f.deserialize::<CSVTime>() {
        csvs.push(i.unwrap().into());
    }
    let len = csvs.len();
    set_csv(csvs);
    if len == 0 {
        panic!(
            "{} is empty.\nexample:\n\nname,begin,end\n1st Math,12:05,13:00\n",
            csvpath
        );
    }
    let args = args();

    if args.count() > 1 {
        get_time_left(&(default_draw as DrawFn));
    } else {
        unsafe {
            signal(libc::SIGALRM, sig_handler as libc::sighandler_t);
            loop {
                alarm(1);
                pause();
                // std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    }
}
