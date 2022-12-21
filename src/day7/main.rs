#[cfg(test)]
mod test;

fn main() {

    // TODO: 
    // - parsing with nom
    // - handmade tree structure fun fun and practise
}

struct File {
    name: String,
    size: usize,
}

struct Dir {
    name: String,
}

enum FsNode {
    File(File),
    Dir(Dir),
}


