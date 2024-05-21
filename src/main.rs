pub use csv::Reader;
pub use libc::{alarm, pause, signal};
pub use libtimeleft::{get_time_left, set_csv, CSVTime, DrawFn, DrawType};
pub use std::env::args;

#[allow(non_snake_case)]
mod sequence {
    pub fn CUU(n: u32) {
        print!("\x1b[{}A", n);
    }
    pub fn CUD(n: u32) {
        print!("\x1b[{}B", n);
    }
    pub fn CUF(n: u32) {
        print!("\x1b[{}C", n);
    }
    pub fn CUB(n: u32) {
        print!("\x1b[{}D", n);
    }
    #[inline]
    pub fn move_to(x: u32) {
        print!("\x1b[{}G", x);
    }
    #[inline]
    pub fn save_pos() {
        //save cursor pos
        print!("\x1b7"); //7 is save
    }
    #[inline]
    pub fn restore_pos() {
        //restore the cursor positon to the one that was saved
        print!("\x1b8"); //8 is restore
    }
}

///positive is down and right
fn move_relative(x: i32, y: i32) {
    //up down right left ABCD
    if x.is_positive() {
        sequence::CUF(x as u32);
    } else if x.is_negative() {
        sequence::CUB(x.abs() as u32);
    }
    if y.is_positive() {
        sequence::CUD(y as u32);
    } else if y.is_negative() {
        sequence::CUU(y.abs() as u32);
    }
}
fn update(time_left: &String, current_time: &String) {
    move_relative(17, -2);
    sequence::move_to(17);
    println!("{}", time_left);
    //move_relative(0, 1);
    sequence::move_to(17);
    println!("{}", current_time);
    //println!("current");
}
fn draw_all_with(
    class: &str,
    time_left: &str,
    current_time: &str,
    class_text: &'static str,
    time_left_text: &'static str,
) {
    use sequence::move_to;
    //move to column 17 to make them all lined up
    print!("{}", class_text);
    move_to(17);
    println!("{}", class);

    print!("{}", time_left_text);
    move_to(17);
    println!("{}", time_left);

    print!("Current Time:");
    move_to(17);
    println!("{}", current_time);
}
pub fn default_draw(
    draw_type: DrawType,
    redraw_all: bool,
    class: &String,
    time_left: &String,
    current_time: &String,
) {
    match draw_type {
        DrawType::In => {
            if redraw_all {
                draw_all_with(class, time_left, current_time, "Class:", "TimeLeft:");
            } else {
                //else update
                //TODO calal this function elsewhere
                update(time_left, current_time);
            }
        }
        DrawType::Before => {
            if redraw_all {
                draw_all_with(
                    class,
                    time_left,
                    current_time,
                    "Next Class:",
                    "Time Till Start:",
                );
            } else {
                //else update
                update(time_left, current_time);
            }
        }
        DrawType::Out => {
            println!("Outta school B)");
        }
    }
}
pub fn sig_handler(_: i32) {
    get_time_left(&(default_draw as DrawFn));
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
        sig_handler(0);
    } else {
        unsafe {
            signal(libc::SIGALRM, sig_handler as libc::sighandler_t);
            loop {
                alarm(1);
                pause();
            }
        }
    }
}
