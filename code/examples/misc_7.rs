macro_rules! capture {
    ($e:expr) => {
        transform!($e)
    };
}

macro_rules! transform {
    ($a:tt + $b:tt) => {
        $a * $b
    };
    ($other:expr) => {
        $other
    };
}

fn main() {
    let x = capture!(2 + 3);
    let y = transform!(2 + 3);
    println!("{x}, {y}"); // what this will print?
}
