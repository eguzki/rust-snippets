use chrono::prelude::*;
use filetime::FileTime;

fn main() {
    let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
    println!(
        "Number of seconds between 1970-01-01 00:00:00 and {} is {}.",
        dt,
        dt.timestamp()
    );

    let current_mtime = FileTime::from_unix_time(dt.timestamp(), 0);
    println!("new mtime is {:?}", current_mtime);
}
