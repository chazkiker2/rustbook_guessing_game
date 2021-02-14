/*
this statement brings the `io` (input/output) library into scope
the `io` library comes from the standard library (which is known as `std`)

By default, Rust brings only a few types into
the scope of every program in the prelude (https://doc.rust-lang.org/std/prelude/index.html).

If a type you want isn't in the prelude, you have to bring that type into scope explicitly with
a `use` statement.
*/
use std::io;




fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    /*
     `let` creates a new variable. variables are immutable by default.
     `mut` makes a variable mutable.
     let foo = 5; // immutable variable
     String::new() constructs a new instance of String.
    */
    let mut guess = String::new();

    /*
     without the import statement, we could've called stdin() like so: `std::io::stdin()`
     the `stdin()` function returns an instance of `std::io::Stdin`, which is a type that
     represents a handle to the standard input for your terminal
    */
    io::stdin()
        /*
         the string arg must be mutable so the method can
         change the string's content by adding the user input
         the & indicates that this argument is a reference, which gives you a way to let
         multiple parts of your code access one piece of data without needing to copy that data
         into memory multiple times.
         &mut guess to make reference mutable
         &guess for immutable ref
        */
        .read_line(&mut guess)
        /*
        read_line puts what the user types into the string we're passing it, but it also returns a
        value — in this case, an `io::Result`. Rust has many types named Result in std.
        A generic Result as well as specific versions for submodules, such as io::Result.

        The Result types are enumerations (enums). An enum is a type that can have a fixed set of
        values, and those values are called the enum's variants. Chapter 6 will cover enums in more
        detail.
        For Result, the variants are `Ok` or `Err`. The `Ok` variant indicates the operation was
        successful, and inside `Ok` is the successfully generated value.
        The `Err` invariant means the operation failed, and `Err` contains information
        about how or why the operation failed.

        The purpose of Result is to encode error-handling information. Values of `Result` have
        methods defined on them. An instance of `io::Result` has an `expect` method that you can
        call. If the returned instance of Result is an `Err` value, expect will cause the program to crash
        and display the message you passed as an argument to `expect`.

        If the `read_line` method returns an `Err`, it would likely be the result of an
        error coming from the underlying operating system. If we didn't call expect, the program
        would compile but we'd also get this warning:
        ``warning: unused `std::result::Result` that must be used.

        */
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
