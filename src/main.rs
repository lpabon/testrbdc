
macro_rules! myprintln {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

    use std::env;
#[macro_export]
macro_rules! requiree {
    ($($arg:tt)*) => (if cfg!(debug_assertions) {
        println!("panic: require:\n\tFile {}:{}",
            file!(), line!());
        env::set_var("RUST_BACKTRACE", "1");
        assert!($($arg)*);
    })
}

macro_rules! require {
    ($a:expr, $($args:tt)*) => (if cfg!(debug_assertions) {
        println!("panic: REQUIRE: File {}:{}",
            file!(), line!());
        println!("vars: {}", sv!($($args)*));
        env::set_var("RUST_BACKTRACE", "1");
        assert!($a);
    })
}

#[derive(Debug)]
struct AA(i32);

#[derive(Debug)]
struct BB(AA);

macro_rules! sv {
    ($var:ident) => (format!("{}={:?} ", stringify!($var), $var));

    ($var:ident, $($arg:tt)*) => (format!("{} {}", sv!($var), sv!($($arg)*)));
}

macro_rules! sv_save {
    ($var:tt) => (format!("{}={:?} ", stringify!($var), $var))
}

macro_rules! pv {
    ($var:tt) => (print!("{} ", format!("{}={:?}", stringify!($var), $var)))
}


fn main() {
    let a = 34;
    let s = BB(AA(234));
    println!("---> a = {}", a);
    //pv!(a);
    //pv!(s);
    println!("{}", sv!(a, s));
    //println!(vv!(a));
    //require!(false, "here is ia", "lot of information");
    require!(a == 32, a, s);
    myprintln!("Hello, world!");
}
