fn main() {
    let possessive_numerals = [
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

    let presents = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens",
        "four calling birds",
        "FIVE GOLDEN RINGS",
        "six geese a laying",
        "seven swans a swimming",
        "eight maids a milking",
        "nine ladies dancing",
        "ten lords a leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];

    for d in 1..=12 {
        let nth_day = possessive_numerals[d - 1];
        let mut day_presents = vec![""; d];
        day_presents.as_mut_slice().copy_from_slice(&presents[..d]);
        day_presents.reverse();

        println!("On the {} day of Christmas, my true love gave to me:", nth_day);
        for p in day_presents.iter() {
            println!("{}", p);
        }
        println!()
    }
}