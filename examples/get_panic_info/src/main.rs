use std::{any::Any, ops::Deref, panic};

pub fn get_panic_message(cause: &Box<dyn Any + Send>) -> Option<&str> {
    cause
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| cause.downcast_ref::<&'static str>().map(Deref::deref))
}

fn main() {
    {
        let p = panic::catch_unwind(|| panic!("A wild panic appeared!"));
        let err = p.unwrap_err();

        println!("{:#?}", err.downcast_ref::<panic::PanicHookInfo>()); // None
        println!("{:#?}", err.downcast_ref::<&str>()); // Some("A wild panic appeared!")
    }
    {
        // get_panic_message utility
        let p = panic::catch_unwind(|| panic!("A wild panic appeared!"));
        let err = p.unwrap_err();
        let message = get_panic_message(&err).unwrap();
        println!("message: `{message}`");
    }
}

// Output:
// ```
// thread 'main' panicked at examples/get_panic_info/src/main.rs:12:40:
// A wild panic appeared!
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// None
// Some(
//     "A wild panic appeared!",
// )
// thread 'main' panicked at examples/get_panic_info/src/main.rs:20:40:
// A wild panic appeared!
// message: `A wild panic appeared!`
// ```
