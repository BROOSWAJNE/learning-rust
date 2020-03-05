const ORDINALS : [&'static str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];

const ITEMS : [&'static str; 12] = [
    "a partridge in a pear tree",
    "two turtle doves",
    "three French hens",
    "four calling birds",
    "five gold rings",
    "six geese a laying",
    "seven swans a swimming",
    "eight maids a milking",
    "nine ladies dancing",
    "ten lords a leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

fn sing(line : &str) {
    let mut letters = line.chars();
    let string : String = match letters.next() {
        None => String::new(),
        Some(letter) => letter.to_uppercase()
            // append all remaining letters in the iterator
            .chain(letters)
            .collect(),
    };
    println!("{}", string);
}

fn sing_verse(num : i32) {
    let idx = (num - 1) as usize;
    let opening = format!("on the {} day of Christmas", ORDINALS[idx]);

    sing(&opening);
    if num == 9 {
        sing("me me me me me me");
    } else {
        sing("my true love gave to me");
    }

    for item in (0..=idx).rev() {
        if idx > 0 && item == 0 {
            let string = format!("And {}", ITEMS[item]);
            sing(&string);
        } else {
            sing(ITEMS[item]);
        }
    }

    println!("");
}

fn main() {
    for v in 1..=12 { sing_verse(v) }
}
