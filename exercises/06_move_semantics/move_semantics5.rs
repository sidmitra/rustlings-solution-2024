// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// I AM DONE

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // mutable ref last used here, hence it's scope ends here?
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references.
