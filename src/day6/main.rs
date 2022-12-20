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

const TEST5: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
const MINI: &str = "ab";

use std::fs::read_to_string;

fn main() -> std::io::Result<()> {
    let _data_string = read_to_string("./data/day6.dat")?;

    // let s = &data_string;
    let s = TEST5;
    let b = TEST5.as_bytes();

    for n in 0..7 {
        cross_myself(&TEST5[0..n].chars().collect::<Vec<char>>());
        println!("\n");
    }

    println!("{:?}", b);
    let r = s
        .as_bytes()
        .windows(4)
        .enumerate()
        // .find(|(i, w)| (w[0] != w[1] && w[1] != w[2] && w[2] != w[3] && w[3] != w[0] && w[0] != w[2] && w[1] != w[3]);
        .find(|(i, w)| {
            let a = w
                .iter()
                .map(|i| w.iter().filter(move |j| &i != j).map(move |j| (i, j)))
                .flatten()
                .find(|(x, y)| {
                    println!("x({}) != y({}) => {}", x, y, x != y);

                    x != y
                })
                .is_none();
            let b = w[0] != w[1]
                && w[1] != w[2]
                && w[2] != w[3]
                && w[3] != w[0]
                && w[0] != w[2]
                && w[1] != w[3];
            a
        });

    dbg!(r);
    println!("Index of seq start : {:?}", r.expect("err").0 + 4);

    Ok(())
}

/* fn cross_items<'t, I, T>(list: I) -> () where I: Iterator<Item=&'t T>, T: Debug + 't {
fn cross_items<'t, I, T>(list: I) -> () where I: IntoIterator<Item=&'t T>, T: Debug + 't {
    let i = list.into_iter();
    i.for_each(|v| println!("{:?}", &v));
    // (&list).iter().enumerate().flat_map(|(i, x)| list.enumerate()).map(|p| dbg!(p));

        //.skip(1).filter(|(i, _), (j, _)| && j > i);
    ()
} */

fn cross_myself<T>(list: &[T]) -> Vec<(&T, &T)>
where
    T: std::fmt::Debug,
{
    let mut acc = 0;
    for l in 1..=list.len() {
        acc += l - 1;
    }
    let mut vec = Vec::with_capacity(acc);
    println!("sizeis {}", acc);
    for (i, x) in list.iter().enumerate() {
        for (j, y) in list[..i].iter().enumerate() {
            vec.push((x, y));
            println!("({},{}): <{:?},{:?}>", i, j, x, y);
        }
    }
    vec
}
