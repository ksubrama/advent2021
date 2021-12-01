// As the submarine drops below the surface of the ocean, it automatically performs a sonar sweep
// of the nearby sea floor. On a small screen, the sonar sweep report (your puzzle input) appears
// each line is a measurement of the sea floor depth as the sweep looks further and further away
// from the submarine.
//
// For example, suppose you had the following report:
//
// 199
// 200
// 208
// 210
// 200
// 207
// 240
// 269
// 260
// 263
// This report indicates that, scanning outward from the submarine, the sonar sweep found depths of
// 199, 200, 208, 210, and so on.
//
// The first order of business is to figure out how quickly the depth increases, just so you know
// what you're dealing with - you never know if the keys will get carried into deeper water by an
// ocean current or a fish or something.
//
// To do this, count the number of times a depth measurement increases from the previous
// measurement. (There is no measurement before the first measurement.) In the example above,
// the changes are as follows:
//
// 199 (N/A - no previous measurement)
// 200 (increased)
// 208 (increased)
// 210 (increased)
// 200 (decreased)
// 207 (increased)
// 240 (increased)
// 269 (increased)
// 260 (decreased)
// 263 (increased)
// In this example, there are 7 measurements that are larger than the previous measurement.
//
// How many measurements are larger than the previous measurement?
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::fs::File;

fn main() -> io::Result<()> {
    let f = File::open("input.txt")?;
    let r = BufReader::new(f);
    let mut lines = r.lines();
    let mut count = 0;

    let prev_depth_str = lines.next().expect("Hold onto your pants and expect").expect("Some bullshit");
    // what in the actual fucking fuck?  Two unwraps?
    let mut prev_depth: i32 = prev_depth_str.parse().expect("Are you not a number?");
    for line in lines {
        let cur_depth: i32 = line.expect("FUCK YOU").parse().expect("Are you not a number?");
        if cur_depth > prev_depth {
            count = count + 1;
        }
        prev_depth = cur_depth;
        // println!("{}", cur_depth.to_string());
    }
    println!("{}", count);

    Ok(())
}
