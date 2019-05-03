fn main() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into this function, which then
                                        // moves its return value into s3

} // s2 goes out of scope and is dropped. s2 goes out of scope but was moved, so
  // nothing happens. s1 goes out of scope and is dropped.

// this function will move its return value into the function
// from which it is called
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // this is the return value
}

// this function takes a string and returns a string
fn takes_and_gives_back(a_string: String) -> String {
    a_string // this is the return value
}
