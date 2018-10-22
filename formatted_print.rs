fn main() {
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob"); // Alice, this is Bob. Bob, this is Alice

    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over"); // the quick brown fox jumps over the lazy dog

    // right align
    println!("{number:>width$}", number=1, width=6); // _____1

    // prefix
    println!("{number:>0width$}", number=1, width=6); // 000001
}
