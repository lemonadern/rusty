///
/// The Rust Programming Language 15.3 Drop
/// https://doc.rust-jp.rs/book-ja/ch15-03-drop.html
///

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!(
            "Dropping `CustomSmartPointer` with data `{}` !!!",
            self.data
        );
    }
}

#[allow(unused)]
fn main() {
    let a = CustomSmartPointer {
        data: String::from("My stuff"),
    };

    let b = CustomSmartPointer {
        data: String::from("Other stuff"),
    };

    println!("CustomSmartpointers created.");
}

// Output:
// CustomSmartpointers created.
// Dropping `CustomSmartPointer` with data `Other stuff` !!!
// Dropping `CustomSmartPointer` with data `My stuff` !!!
