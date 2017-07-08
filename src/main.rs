
macro_rules! myprintln {
    () => (print!("\n"));
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

    use std::env;
macro_rules! requiree {
    ($($arg:tt)*) => (if cfg!(debug_assertions) {
        println!("panic: require:\n\tFile {}:{}",
            file!(), line!());
        env::set_var("RUST_BACKTRACE", "1");
        assert!($($arg)*);
    })
}

macro_rules! dbc_panic {
    ($type:expr, $a:expr) => (if cfg!(debug_assertions) {
        println!("panic: {}: \nfile: {}:{}",
            $type, file!(), line!());
        env::set_var("RUST_BACKTRACE", "1");
        assert!($a);
    });
    ($type:expr, $a:expr, $($args:tt)*) => (if cfg!(debug_assertions) {
        println!("panic: {}: \nfile: {}:{}",
            $type, file!(), line!());
        println!("vars:\n{}", sv!($($args)*));
        env::set_var("RUST_BACKTRACE", "1");
        assert!($a);
    })
}

#[macro_export]
macro_rules! require {
    ($a:expr) => (if cfg!(debug_assertions) {
        dbc_panic!("REQUIRE", $a)
    });
    ($a:expr, $($args:tt)*) => (if cfg!(debug_assertions) {
        dbc_panic!("REQUIRE", $a, $($args)*)
    })
}

#[macro_export]
macro_rules! ensure {
    ($a:expr) => (if cfg!(debug_assertions) {
        dbc_panic!("ENSURE", $a)
    });
    ($a:expr, $($args:tt)*) => (if cfg!(debug_assertions) {
        dbc_panic!("ENSURE", $a, $($args)*)
    })
}

macro_rules! requirev {
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
    ($var:ident) => (format!("   {}={:?}\n", stringify!($var), $var));
    ($var:ident, $($arg:tt)*) => (format!("{}{}", sv!($var), sv!($($arg)*)));
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
    let c = "ehloo";
    let f = 'f';
    //println!("---> a = {}", a);
    //pv!(a);
    //pv!(s);
    //println!("{}", sv!(a, s));
    //println!(vv!(a));
    //require!(false, "here is ia", "lot of information");
    ensure!(a==43,a);
    require!(a == 32, a, s, c, f);
    myprintln!("Hello, world!");
}
