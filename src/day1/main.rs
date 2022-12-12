use std::fs::read_to_string;

#[derive(Debug)]
struct Podium<T> {
    inner: [T; 3],
}

impl<T> Default for Podium<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            inner: [T::default(), T::default(), T::default()],
        }
    }
}

impl<T> Podium<T>
where
    T: PartialOrd + Copy + std::fmt::Debug,
{
    fn propose(&mut self, candidate: T) {
        if candidate > self.inner[2] {
            if candidate > self.inner[0] {
                self.inner = [candidate, self.inner[0], self.inner[1]];
            } else if candidate > self.inner[1] {
                self.inner = [self.inner[0], candidate, self.inner[1]];
            } else {
                self.inner = [self.inner[0], self.inner[1], candidate];
            }
        }
    }
}

impl Podium<Elf> {
    fn sum_calories(&self) -> u32 {
        self.inner.iter().map(|e| e.1).sum()
    }
}

#[derive(Default, Debug, Clone, Copy)]
struct Elf(usize, u32);

impl PartialEq for Elf {
    fn eq(&self, other: &Self) -> bool {
        self.1 == other.1
    }
}

impl PartialOrd for Elf {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.1.partial_cmp(&other.1)
    }
}

fn main() -> std::io::Result<()> {
    let parse_u32 = |s: &str| {
        s.parse::<u32>()
            .expect(format!("Impossible to parse integer {}", s).as_str())
    };

    let podium = read_to_string("./data/day1.dat")?
        .trim()
        .split("\n\n")
        .map(|grpd_lines| grpd_lines.split('\n').map(parse_u32).sum())
        .enumerate()
        .map(|(idx, cal)| Elf(idx, cal))
        .fold(Podium::default(), |mut podium, cur| {
            podium.propose(cur);
            podium
        });

    println!("Sum of the calories of the top 3 elves tht picked the most calories {:?}", podium.sum_calories());

    Ok(())
}
