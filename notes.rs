/* =========
 * ---------
 * Variables 
 * ---------
 * =========
 */

// immutable by default
let x = 1;
x = 2; // doesn't work
let mut y = 1;
y = 2; // works!

// can be re-declared ("shadowed")
let z = 1;
let z = z * 5; // works, original z has memory de-allocated
let z = "foo"; // also works, doesn't have to be the same type

// const:
// - can *never* be mutable (not allowed to use mut)
// - *must* have type annotated
// - can only be set to the result of a constant expression, not result of a fn call
// - can be declared in global scope, not just fn scope
const THAT_WELL_KNOWN_CONSTANT : f64 = 3.14159265359;

/* Data Types */

// integer types
// either iX or uX, for signed or unsigned respectively
let a : u8 = 2^8 - 1;
let b : u16 = 2^16 - 1;
let c : u32 = 2^32 - 1; // default is i32
let d : u64 = 2^64 - 1;
let e : u128 = 2^128 - 1;
let f : usize = 2^64 - 1; // or 2^32 - 1 on a 32-bit system
// overflows result in PANIC in debug compilation, or wrapping in production
// different bases
let dec = 12345;
let hex = 0xff;
let oct = 0o77;
let bin = 0b1101;
let byte = b'A'; // can only be u8
// can also be declared using a type suffix (except for byte, since forced u8)
let a = 123u8;
let b = 123u16;
// floating point types
let a : f32 = 1.2345; // 32-bit
let b : f64 = 1.2345; // 64-bit - DEFAULT

// boolean is one byte in size
let t = true;
let f : bool = false;

// character - is unicode!
let a = 'a';
let b : char = 'b';

// tuples - elements can be of different types, fixed length
let tup = ( 123, 1.23, 'a' );
let tup : ( i32, f64, char ) = ( 123, 1.23, 'a' );
// element access using '.'
let onetwothree = tup.0;
let achar = tup.2;

// array - stack allocated rather than heap, still fixed length, all same type
let arr = [ 123, 456, 789 ];
let arr : [ i32; 3 ] = [ 123, 456, 789 ];
// initialising an array of length 5 with every element being 3:
let arr = [ 3; 5 ];
// element accesss using []
let onetwothree = arr[0];
let fourfivesix = arr[1];
// index out of bounds access is a runtime error (rather than accessing invalid memory)

// &str is a string slice, can be thought of as pointer+length combination
fn substring(string : &String, start : &usize, end : &usize) -> &str {
    &s[start..end]
}
// array slices similarly refer to part of an array, by storing a pointer+length
let array : [i32; 5] = [ 1, 2, 3, 4, 5 ];
let slice : &[i32] = &array[1..3]; // type is &[ArrayElementType]

/* =========
 * ---------
 * Ownership 
 * ---------
 * =========
 */

// copied from book ch4.1, found this really useful:

fn main() {
    let s = String::from("hello");
    // s' value moves into the function, and so is no longer valid after
    takes_ownership(s);
    // anything using s here would cause a compile error
    let x = 5;
    // x would move into the function, but i32 is `Copy`, so it's ok to still use x afterwards
    makes_copy(x);
// here, x goes out of scope, then s. but because s' value was moved, nothing special happens
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
// here, some_string goes out of scope and `drop` is called. the backing memory is freed.
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
// here, some_integer goes out of scope. nothing special happens
}

// again copied:

fn main() {
    // gives_ownership moves its return value into s1
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    let s3 = takes_and_gives_back(s2);
// here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.
}
// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string is returned and moves out to the calling function
    some_string
}
// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { 
    // a_string is returned and moves out to the calling function
    a_string  
}

// if my function takes a reference to something as its parameter
// then instead of taking ownership, it just stores a pointer to
// the original value, meaning that the value is never `move`d and
// ownership remains in the calling scope
fn main() {
    let s1 = String::from("hello");
    // just pass a reference, keeping ownership
    let len = calculate_length(&s1); 
    println!("The length of '{}' is {}.", s1, len);
}
// explicitly ask for a reference to a string as argument
fn calculate_length(s: &String) -> usize {
    s.len() // don't need to dereference in any way?
}
// when passing a reference instead of transferring ownership, we say you're `borrowing`

// references aren't mutable by default, trying to mutate/modify a borrowed value (ie. a reference)
// would cause a compile error. to make it work we'd need our parameter type to be &mut, an
// explicitly mutable reference
fn hack_a_string(target : &mut String) {
    target.push_str("hehe, hacked");
}

// however, you *can't* have more than one mutable reference to a value at a time, to avoid a "data
// race", ie. a race condition as multiple pointers are accessing and mutating data at the same
// time without proper synchronisation

// similarly, you can't borrow something as mutable when it's already borrows as immutable, as
// otherwise the immutable reference could be mutated even though it's expected (by definition) to
// be immutable
let ref_me = String::from("hello");
let ref_1 = &ref_me; // no problem
let ref_2 = &ref_me; // no problem, both immutable so no danger
let ref_3 = &mut ref_me; // compile error! (assuming ref_1 and ref_2 are actually still used after)
// if ref_1 and ref_2 are unused by the time ref_3 is initialized, it would be fine.

// compiler will avoid dangling references (ie. pointers to data that has already been freed),
// the following would not compile
fn dangle() -> &String {
    let s = String::from("hello");
    &s // return a reference to the string
} // string gets cleaned up, but reference would still exist. compile error!

// string slices act as immutable borrows, meaning this would compile error
let mut string = String::from("hello world");
let world = &string[6..]; // immutable borrow for the slice
string.clear(); // requires a mutable borrow
println!("hello {}", world); // the immutable borrow was still in scope!

/* =======
 * -------
 * STRUCTS
 * -------
 * =======
 */

// defined by giving it a name, and defining the fields it contains and their type
struct User {
    username : String,
    email : String,
    sign_in_count : u64,
    active : bool,
}
// initialize an instance simply by declaring a variable with the fields filled
let me = User {
    // doesn't have to be provided in the order it was defined
    email: String::from("bossman@website.cool"),
    username: String::from("big boss"),
    active: true,
    sign_in_count: 1e9,
}
// all fields are immutable, or mutable if we initialized with `let mut me = User { ... }`

// can define tuple structs, where fields aren't named, just allows for implementing traits
struct Color(i32, i32, i32);
struct Vector3(i32, i32, i32);
// or structs with no fields!
struct ThingWithTraits();

// all fields have to be an owned type, as we need to ensure that all the data within the struct
// will live for as long as the struct itself lives.
// this would be a compile error:
struct User {
    username : &str, // slice has no owner
    email: &str, // we'd need to provide a `lifetime` to fix
}
// `String` works above since it is indeed owned)

// some traits can be derived automatically
#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}
let rect = Rectangle { width: 20, height: 30 };
println!("{:?}", rect); // prints "Rectangle { width: 20, height: 30 }"

// define a method by impl(ementing) it
impl Rectangle {
    // type of self is not needed, since it's known to just be Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
println!("{}", rect.area); // 600

// same as any function, a struct method can claim ownership of self
fn area(self) { ... }
// or borrow it immutably
fn area(&self) { ... }
// or borrow it mutably
fn area(&mut self) { ... }

// define an associated method within an impl block simply by not having `self` as a parameter
impl Rectangle {
    fn square(size : u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
let square = Rectangle::square(20);
println!("{:?}", rect); // prints "Rectangle { width: 20, height: 20 }"

/* Enums */

// defined by listing the values
enum Animal {
    Dog,
    Cat,
    Tiger,
    Dolphin,
    Shark,
}

// functions/structs can take any enum value as their parameter
struct Pet {
    animal : Animal,
    name : String,
}
fn befriend(animal : Animal) { ... }

// used by double-colon
let doggo = Pet {
    animal: Animal::Dog,
    name: "Doggo",
}

// can also have an enum which takes data directly into each enum variant
enum IP {
    V4(String),
    V6(String),
}
let home = IP::V4(String::from("127.0.0.1"));
// this has the advantage over structs that each variant can have different types/amounts of data
enum IP {
    V4(u8, u8, u8, u8),
    V6(String),
}
let home = IP::V4(127, 0, 0, 1);
// the enum data can anything, even a struct or enum itself
enum Person {
    WithPet(Pet), // named struct
    WithFriend { name : String }, // anonymous struct
    Lonely, // no associated data
}

// we can define methods on enums, same as methods on structs (methods apply to all types)
impl IP {
    fn ping(&self) { ... }
}
let home = IP::V4(127, 0, 0, 1);
home.ping();

/* Control Flow */

// instead of
match option_for_i32 {
    Some(69) => println!("nice"),
    _ => (),
}
// we can do
if let Some(69) = option_for_i32 {
    println!("nice");
}
// more concise, but less exhaustive (match guarantees every case is handled at compile time)
