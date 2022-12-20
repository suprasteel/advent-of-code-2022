/*
Vmjqjpqmgbljsphdztnvjfqwrcgsmlb

After the first three characters (mjq) have been received, there haven't been enough characters received yet to find the marker. The first time a marker could occur is after the fourth character is received, making the most recent four characters mjqj. Because j is repeated, this isn't a marker.

The first time a marker appears is after the seventh character arrives. Once it does, the last four characters received are jpqm, which are all different. In this case, your subroutine should report the value 7, because the first start-of-packet marker is complete after 7 characters have been processed.

Here are a few more examples:

    bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
    nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
    nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
    zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

 */

const TEST: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";

use std::fs::read_to_string;

fn main() -> std::io::Result<()> {

    let data_string = read_to_string("./data/day6.dat")?;

    let s = &data_string;
    // let s = TEST;
    let r = s.as_bytes()
        .windows(4)
        .enumerate()
        .find(|(i, w)| (w[0] != w[1]) && (w[1] != w[2]) && (w[2] != w[3]) && w[3] != w[0] && w[0] != w[2] && w[1] != w[3]);

    println!("Index of seq start : {:?}", r.expect("err").0 + 4);

    Ok(())
}

