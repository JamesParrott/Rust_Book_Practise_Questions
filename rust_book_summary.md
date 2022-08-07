# Study Notes from the (Rust book)[https://github.com/rust-lang/book]  

Study notes summarising the rust book.  A concise personal aide-memoire, not an accessible readable work.  Intended only for an audience of one (myself).  
By its nature, it heavily quotes and summarises content from the actual The Rust Book, so copyright remains with the The Rust Project Developers.
Feel free to use this however you wish subject to the MIT license (below), but no particular effort towards 
readability has been made.  In any case you will gain much 
more benefit from reading The Rust Book directly yourself and making your own notes.... https://doc.rust-lang.org/book/title-page.html

## MIT License  
Copyright (c) 2010 The Rust Project Developers  
  
Permission is hereby granted, free of charge, to any  
person obtaining a copy of this software and associated  
documentation files (the "Software"), to deal in the  
Software without restriction, including without  
limitation the rights to use, copy, modify, merge,  
publish, distribute, sublicense, and/or sell copies of  
the Software, and to permit persons to whom the Software  
is furnished to do so, subject to the following  
conditions:  
  
The above copyright notice and this permission notice  
shall be included in all copies or substantial portions  
of the Software.  
  
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF  
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED  
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A  
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT  
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY  
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION  
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR  
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER  
DEALINGS IN THE SOFTWARE.  
  
  
## Chapter 1 - Installation  
Use rustup.    
linux - need a linker (e.g. that from gcc or clang).    
windows - needs Visual Studio C++ build tools (e.g. 2019)  
rustup doc opens local documentation copy  
### Hello World  
rust source files end in .rs  
rust code runs in a `main` function:  
```  
fn main() {  
  
}  
```  
'Function' calls with a ! between the function name and bracketed args are Rust macros.   
Statements in rust end in semi-colons.  Otherwise they are expressions, e.g. for implicit return of the final argument from a function or code block.  
You can compile stand alone files with `rustc`, but it's normal to use cargo.  
### Hello Cargo  
`cargo new project_name` creates a new Hello World project, including dependencies (`Cargo.toml`), dependency versions, `Cargo.lock` and .gitstuff (other VCSs are supported too).  
Once the version are figured out they are stored in `Cargo.lock` for future reference, giving automatically reproducible builds.  Hence:  
`Cargo.lock` is often checked into source control.  
`cargo update` will recalculate the versions and update `Cargo.lock`.  
`cargo build` to compile.   
`cargo build --release` to compile optimised version.  
`cargo run` to build and run.   
`cargo check` checks compilation works without producing an executable.  
  
## Chapter 2 Guessing Game and Chapter 3 and a bit of 4  
`Cargo.toml` contains a  `[dependencies]` table.  
Lines are: `name = x.y.z`, crate named name, semantic version: major x, minor y, patch z.  E.g. `rand = "0.8.3"`  
`use std::thing` imports thing from the standard library (prelude).  
`use crate_name::TraitName` refers to a trait on the crate, a definition of methods that must be implemented.  
`\\ comments are after \\ `.  There are documentation comments as well.  
`fn` declares a new function:   
```  
fn function_name() -> ReturnType {  
    statement;  // statements don't return a value.  
    statement; // but don't necessarily end in ;, e.g. function definitions are statements.  
    ...  
    statement; // statements can contain expressions.  
    expression // never end in semi-colon. implicitly returned by functions.  Optional.  
}  
```  
Function calls and macro calls and scope blocks (`{ \\code } `) are expressions.    
Naming convention is snake_case.  Code body of functions must be in `{` and `}`.  Functions are 'hoisted' like JS (can be declared below where they are called).  Parameter types MUST be declared in function signatures.  Args not declared with `&` will cause shadowing.  kwargs aren't required.  
`=` can bind a value to a variable.  Naming convention is snake_case.    
`&` indicates an argument is a _reference_.  
`let var_name: Type = val ` creates an `Type` immutable variable. This cannot be assigned to again after its definition.  
`Type` annotations can be added (and e.g. for `const` must be), but Rust also has type inference.  
`let mut var_name ` creates a mutable variable.  
`const var_name` creates a constant that can be evaluated at compile time, that must be followed by a type hint.  A basic operation set is supported: https://doc.rust-lang.org/reference/const_eval.html.  The naming convention for them is CAPITALS plus underscores.  
`::` following a type refers to an 'associated function' of that type,   
e.g. `let s = String::new()` defines an immutable empty String (UTF-8 encoded).  
`io::stdin().read_line(& mut input)` reads a line from stdin, appending to a string input (including `\n`.  Rust doesn't include `\r` in new lines).  
`.expect(msg)` handles errors indicated by the `Result` retval of a lot of methods, e.g. `read_line`, by returning a string literal `msg` and crashing the program.    
`Result`, the retval of `.read_line` above, is an "enumeration" (enum).  Enumerations can be in multiple possible states called variants, e.g. `OK` and `Err` for `Result`.  
`let var: type = old_var.trim().parse().expect()` likely follows `.read_line`,   
especially with shadowing.    
`.parse` tries to parse old_var to a `type`   
`.trim` removes all leading and trailing whitespace, including "\n".    
`"..."` enclose a string literal.  Their values are known at compilation, so can be hardcoded into the binary, making them fast and efficient.  
`'...'` enclose a single `char` character. `char`s have 4 bytes and can represent a Unicode Scalar Value.  
`"{}"` is an empty format string.  The `{}` can contain the name of a value to interpolate, and if inside `println!` or `format!` can be followed by those value(s).     
`String` manages data on the heap (described below), and can grow. e.g. `let s = String::from("hello");`.  If they are `mut` they support the `push_str` method and others.  Requests memory at run-time.  
  
`x..y` is a "range expression"  
`x..=y` is an inclusive "range expression" that includes its upper bound.  
`(x..y).rev()` reversed range expression.  
`Ordering` is an enum from `use std::cmp::Ordering;` with variants `Less`, `Greater` and `Equal`.  
```  
match var_1.cmp(&var_2) {  
    Ordering::Less => {} //arm 1  
    Ordering::Greater => expression //arm_2  
    Ordering::Equal => expression //arm_2  
}  
```    
Example syntax for match statements.  
`let y = x` will make y "shadow" x, making x leave scope afterwards.   
`i8, i16, i32, i64, i128, iDDD` DDD-bit signed integer types (on stack). Twos complement representation..  
`u8, u16, u32, u64, u128, uDDD` DDD-bit unsigned integer types (on stack).  
`isize, usize` are 32 bit or 64 bit depending on the architecture of the machine running the program.  
Number literals can use _ as thousands separators.  
`0x` Prefix of a hex literal.  
`0o` Prefix of an octal literal.  
`0b` Prefix of a binary literal.  
`Integer overflow` by default integer wrapping will occur!  
To handle  these you can use `wrapping_`, `checked_`, `overflowing_` and `saturating_` methods.  
`f32, f64` 32 bit and 64 bit floating-point number types (signed).  IEEE-754.  Single, Double.  
`+` addition.  
`-` subtraction.  
`*` multiplication.  
`/` division (int or float depends on the type of the operands).  
`%` remainder.  
`bool` Boolean type either `true` or `false`.  
`let variable: ( , , ) = ( , , )`.  Tuple. Can mix types.  Fixed length.  May be `mut`able.  Can access elements with `.0`, `.1` etc. Can be unpacked using a pattern.  
`()` "unit".  Empty tuple.  
`let variable: [type; 9] = [ , , ... , ]`.  Array.  All elements must have same type.  Fixed length. Allocate contents to the stack. `[ ; ]` syntax can be used to initialise to repeated values.  Access elements using [0], [1], etc.  Accessing out of bounds indices will cause a panic.  
`Vec`.  Vectors.  Like arrays but can grow.  Implemented using generics.  
`loop {  }` creates an infinite loop (most terminals allow Ctrl-C to quit).  
`break` breaks out of a loop.  It can also return expressions from them: `let result = loop {... break retval;}` or break specific labelled loops:  
`'loop_name: loop{ break 'loop_name}`.  Otherwise inner most loop only is `break`ed out of.  
`continue` skips to the next iteration of a loop.  
`while condition { ... }`  
`for element in collection `  
Rust keywords are not valid variable or function names: https://doc.rust-lang.org/book/appendix-01-keywords.html  as, async, await, break, const, continue, crate, dyn, else, enum, extern, false, fn, for, if, impl, in, let, loop, match, mod, move, mut, pub, ref, return, Self, self, static, struct, super, trait, true, type, union, unsafe, use, where, while, and for future use: abstract, become, box, do, final, macro, override, priv, try, typeof, unsized, virtual and yield.  
Shadowing - reassign a variable name or make a new reference to it with `let` (e.g. that can be initialised to an expression involving the old shadowed variable).  Mutable variables can change type.  Immutables can't.  
Shadowed variables on the heap leave scope.  This is like shallow copying but as it also invalidates the first variable, it is called _moving_.  
Cannot do `x = y = 6` in Rust.  
`if condition_1 {} else if condition_2 {} ... else {}` if/else if/ else statements.  
condition must be a `bool`.  It does not automatically evaluate the truthiness of a variable.    
`if...` is an expression.  It can be used in a `let` statement as a Ternary, e.g. `let var_name = if condition {1} else {-1}`.  However the resulting expressions of all arms must be the same type.  
  
## Chapter 4 Ownership  
Rust has no automatic garbage collector (it has `drop`).  Nor explicit allocation and deallocation of memory.  
Stack : LIFO.  Fast.  Data on the stack must have a known size.  
Heap : Not LIFO.  A bit slower, but can grow - data that changes size can be stored.  Uses a pointer.  
Each value in Rust has an _owner_.  Only one at a a time.   
A value is dropped when its owner goes out of scope.  
Rust uses Resource Acquisition is Initialisation (RAII).  
Rust never automatically creates deep copies (this cna be done by   
`.clone`), so automatic copying is inexpensive, performance wise.  
Variables can either have the `Copy` trait or the `Drop` trait.  
Return values of functions can transfer ownership if their arg doesn't use a `&` to reference the arg.  So using `&` is common.  
`*` is the dereferencing operator.  
`&mut` must be used to alter a `mut` variable.  
But a `mut`able variable can have only one `&mut`able reference at a time to it.  
As many immutable references as desired are allowed, but having one precludes a mutable reference.     
`&variable_name[a..b]` is a "slice".  It is a reference to a portion of a String or an Array.  They avoid problems with byte counting in strings.  
  
## Chapter 5 Structs  
```  
struct StructName {  
    field_name_1 : FieldType1,  
    field_name_2 : FieldType2,  
    ...,  
}  
```  
Structs can be defined before the main function.  
```  
let instance = StructName {  
    field_name_1 : value_1,  
    field_name_2 : value_2,  
    ....,  
};  
```  
`instance.field_name_1` accesses fields.  
`instance.field_name_2 = new_value` assigns to fields of `mut`able instances only.    
```  
fn build_user(email: String, username: String) -> User {  
    User {  
        email,  
        username,  
        active: true,  
        sign_in_count: 1,  
    }  
}  
```   
"field init shorthand" syntax (omit repeating identically named and typed args)  
```  
    let user2 = User {  
        email: String::from("another@example.com"),  
        ..user1  
    };  
```   
struct update syntax, for creating one instance from another. But this can Move variables.  
```  
struct Color(i32, i32, i32);  
  
fn main() {  
    let black = Color(0, 0, 0);  
}  
```   
are tuple structs.  Their fields are unnamed.  
```  
struct AlwaysEqual;  
  
fn main() {  
    let subject = AlwaysEqual;  
}  
```   
unit like structs.  
  
```  
#[derive(Debug)]  
struct Rectangle {  
    width: u32,  
    height: u32,  
}  
  
fn main() {  
    let rect1 = Rectangle {  
        width: 30,  
        height: 50,  
    };  
    println!("rect1 is {:?}", rect1);  
}  
```   
Structs can opt into providing debugging information. The output from a format string field `{:#?}` can be easier to read.  
  
### Method syntax.  
Methods are like functions obviously.  They are defined within structs, enums or traits.  Their first param is always self.  For example:  
```  
#[derive(Debug)]  
struct Rectangle {  
    width: u32,  
    height: u32,  
}  
  
impl Rectangle {  
    fn area(&self) -> u32 {  
        self.width * self.height  
    }  
}  
```  
"impl" stands for implementation.  
`&self` is short for `self: &Self`.  Methods can borrow `self` mutably.  It's rare for methods to take ownership, but for methods that transform instances it's possible.  
`rect1.area()` method call.  
Methods can share the names of fields - Rust differentiates by looking for brackets afterwards.  Rust does not implement getters automatically for struct fields.  
Rust doesn't have separate `->` and `.` like C and C++ (different to return type hints), it has _automatic referencing and dereferencing_.  
All functions in the impl block are _"associated functions"_.  but they need not all be a method with self as their first arg - e.g. factories can be included too.  
Structs can have multiple impl blocks.  
  
## Chapter 6 Enums  
Enums define a type by enumerating its possible variants.  
```  
enum EnumNameKind {  
    VariantName1(Type1, ...),  
    VariantName2(Type2, ...),  
}  
```   
definition syntax.    
Initialised e.g. as   
`EnumNameKind::VariantName1(Type1::from(some_variable))`  
There is no Null value in Rust.  The Option enum should be used instead:  
```  
enum Option<T> {  
    None,  
    Some(T),  
}  
```   
`<T>` is a generic type parameter.  
  
### match  
The arms of a match statement need to be exhaustive.  But catch alls can just use a simple variable, e.g. _other_ in:  
```  
    match dice_roll {  
        3 => add_fancy_hat(),  
        7 => remove_fancy_hat(),  
        other => move_player(other),  
    }  
```  
or to define 'nothing happens':  
```  
    match dice_roll {  
        3 => add_fancy_hat(),  
        7 => remove_fancy_hat(),  
        _ => (),  
    }  
```  
```  
    let config_max = Some(3u8);  
    match config_max {  
        Some(max) => println!("The maximum is configured to be {}", max),  
        _ => (),  
    }  
```   
accessing the content of Some via pattern matching.  
### if let  
```  
   let config_max = Some(3u8);  
    if let Some(max) = config_max {  
        println!("The maximum is configured to be {}", max);  
    }  
```   
Does same thing as above.  
`if let` takes a pattern and an expression separated by an equal sign.  It works like a match with one arm (and ignores all other values).  The exhaustive checking that match enforces is lost.  
`if let` can be followed by an `else`, which runs if there is no match.  
  
## Chapter 7 - Packages, Crates and Modules.  
 -   Packages: A Cargo feature that lets you build, test, and share crates.  A package contains a Cargo.toml file that describes how to build those crates.  A package can contain at most one library crate.  
 -   Crates: A tree of modules that produces a library or executable.  A crate can be a binary crate or a library crate. Library crates don't have a main function, and they don't compile to an executable. They define functionality intended to be shared with multiple projects.   
 The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate   
 -   Modules and use: Let you control the organization, scope, and privacy of paths  
 -   Paths: A way of naming an item, such as a struct, function, or module  
   
Cargo follows a convention that src/main.rs is the crate root of a binary crate with the same name as the package. Likewise, Cargo knows that if the package directory contains src/lib.rs, the package contains a library crate with the same name as the package, and src/lib.rs is its crate root.      
### `use`, `pub`, modules and paths.  
 - Start from the crate root file:  `src/main.rs`, or `src/lib.rs` for  a library crate.  
 - `mod module_name;` declares a module inside the crate root file.  The compiler then looks for it in: `{`curly brackets after the declaration, on the same line`}`, `src/module_name.rs`, and in `src/module_name/mod.rs`.    
 - `mod sub_module_name;` within `src/module_name.rs` declared a submodule.  The compiler looks for it on the same line as above,   
  and in: `src/module_name/sub_module_name.rs` and `src/module_name/sub_module_name/mod.rs`  
 - You can then refer to code in submodules via: `crate::module_name::sub_module_name::TypeToImport`.  
 - `pub` before a `mod` (or any declaration) makes a module (and other items within it, including `struct` fields) public.  i.e. accessible by its parent modules.  
 - `use` creates shortcuts to reduce repetitions of long paths.  
  
### Paths in the Module Tree.  
A path can take two forms:    
 - An absolute path starts from a crate root by using a crate name (for code from an external crate) or a literal `crate` (for code from the current crate).  
 - A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.  
If `struct`s are public but have private fields, they need to provide a public factory to set private fields.  
A public `enum` automatically has all its variants public.  
To handle name clashes and names, `use crate_name::thing` supports `as NewName`.  
Both referring to items via their parent modules, and using aliases via `as` are considered idiomatic.  
`pub use` re-exports names brought into the current scope.  
#### Nested paths.  
`use std::{cmp::Ordering, io};` is the same as:  
```  
use std::cmp::Ordering;  
use std::io;  
```  
`use std::io::{self, Write};` is the same as:  
```  
use std::io;  
use std::io::Write;  
```  
#### Glob Operator  
use std::collections::*;  
### Modules in different files.  
`mod module_name` declares a module.  The compiler looks for `module_name.rs` or `src/module_name/mod.rs`  
`mod` is not an include - it only needs to declare an option once, and then other files in the project can refer to it via a path to that declaration (path as above).  
  
## Chapter 8. Collections.  
Data structures in _collections_ store data on the heap.  The main 3 are:  
 - vector,  
 - string,  
 - hash map.  
 Sets and a binary Heap are available too, as are variations of vector and hash map.  
### Vectors  
`let v: Vec<i32> = Vec::new();` example declaration, with no initial values.  
All elements contained by a Vector must have the same data type, but known different types can be stored together as variants of an Enum.  unknown different types require a trait.  
`let v = vec![1, 2, 3];` creating a vector using the `vec!` macro.  
`Vec`s support `.push(val_to_append)`, `.pop`, `.insert`, `.len`, `.clear`, `.drain`, `.is_empty`, `.truncate` and `.append` methods.  
`Vec`s can be sliced with ranges.  
Vectors drop all their elements when leaving a scope.  
`&v[i]` is a reference to the (i+1)th element of v. IT will cause a panic if i is out of bounds.  
`v.get(i)` returns an Option<&T>, to allow handling of out of bounds indices.  
An immutable reference of an element of a mutable vector cannot be created.  
```  
for el in &v {  
    println!("{}", el);  
}  
```   
iterates over the values in v.  
```  
let mut v = vec![100, 32, 57];  
for i in &mut v {  
    *i += 50;  
}  
```   
Mutating the vector.  Here, `*` is the dereferencing operator.  
  
### Strings  
`String`s can grow - they support `.push_str` to append string slices without taking ownership of the arg.  `.push` takes a single `char`.  
`String` is the main type. `str` is a string slice, usually seen as `&str`, e.g. string literals (hardcoded in the binary).  
```  
let data = "initial contents";  
let s = data.to_string();  
```   
and   
`let s = "initial contents".to_string();` both create a String from a string literal.  `.to_string` is available on anything that implements the `Display` trait.  
`+` concatenates strings.  It calls the add method, which takes ownership of the LHS, first argument, self.    
`.add` only adds an `&str` to a `String`, but the compiler can coerce an `&String` arg into an `&str` (deref coercion).  
  
#### The format! macro  
```  
let s1 = String::from("tic");  
let s2 = String::from("tac");  
let s3 = String::from("toe");  
let s = s1 + "-" + &s2 + "-" + &s3;  
```  
returns `tic-tac-toe`, but takes ownership of s1.  
`let s = format!("{}-{}-{}", s1, s2, s3);` doesn't take ownership of   
any args.  
  
##### Indexing into Strings  
Strings cannot be indexed with integers (different unicode code points need different numbers of Bytes under `utf-8`).   
`String` is a wrapper over `Vec<u8>`.  
Using ranges to create string slices can crash your program.  
  
##### Iteration over Strings  
The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.   
```  
for c in "नमस्ते".chars() {  
    println!("{}", c);  
}  
```  
```  
for b in "नमस्ते".bytes() {  
    println!("{}", b);  
}  
```  
Crates are available on crates.io for grapheme clusters (letters), e.g. [https://crates.io/crates/unicode-segmentation].  
  
### Hash Maps  
`HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a _hashing function_.  The key can be of any type.  They are growable.   
`use std::collections::HashMap;  
  
let mut scores = HashMap::new();  
  
scores.insert(String::from("Blue"), 10);  
scores.insert(String::from("Yellow"), 50);` Example use.  
HashMaps are not automatically in the prelude - they need to be brought in.  
```use std::collections::HashMap;  
  
let teams = vec![String::from("Blue"), String::from("Yellow")];  
let initial_scores = vec![10, 50];  
  
let mut scores: HashMap<_, _> =  
    teams.into_iter().zip(initial_scores.into_iter()).collect();  
```  
Creating a HashMap from a vector of keys and a vector of values.  
`<_,_ >` means Rust will infer the types the HashMap contains based on the data.  
HashMaps will own owned values like String, both as keys and values.  
References can be inserted into a HashMap but their values must be valid while the HashMap is.  
HashMaps support the `.get` method.  An Option is returned.  
```  
for (key, value) in &scores {  
    println!("{}: {}", key, value);  
}  
```   
Iteration over a HashMap scores.  
HashMaps are not ordered.  
HashMaps have a special API, `entry`, for checking if a key is in a HashMap (has a value) and if not, giving it one:  
```  
    use std::collections::HashMap;  
  
    let mut scores = HashMap::new();  
    scores.insert(String::from("Blue"), 10);  
  
    scores.entry(String::from("Yellow")).or_insert(50);  
    scores.entry(String::from("Blue")).or_insert(50);  
  
    println!("{:?}", scores);  
```  
`.or_insert` returns a mutable reference, which can be used to change the val without taking ownership.  For example, a word counter:  
```  
    use std::collections::HashMap;  
  
    let text = "hello world wonderful world";  
  
    let mut map = HashMap::new();  
  
    for word in text.split_whitespace() {  
        let count = map.entry(word).or_insert(0);  
        *count += 1;  
    }  
  
    println!("{:?}", map);  
```   
prints: `{"world": 2, "hello": 1, "wonderful": 1}`  
The default hashing function _SipHash_ of a HashMap is resistant to DDOS attacks but is a little slower.  It can be swapped to a different _hasher_, e.g. from crates.io.

## Chapter 9 Error handling

panic!(exit_message) prints exit_message, unwinds, cleans up the stack, and quits.  

You can make smaller binaries by adding:  
```
[profile.release]
panic = 'abort'
```
to Cargo.toml.

Backtraces can be printed on panic by running with `RUST_BACKTRACE=1 cargo run`, to set the env variable.

Result and Option enums are brought into scope by the Prelude.  

`use std::io::ErrorKind;` can bring in lots of useful error types for sub matching, e.g.:  
```    
let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
        ErrorKind::NotFound => match File::create("hello.txt") {
```  
etc. 

`.unwrap()` is a shortcut method that `unwrap`s the value inside and `OK`, else panics on an `Err`.  Unlike `.expect` its error message cannot be customised.   It's understood that a call to `unwrap` or `expect` is meant as a placeholder for the way you'd want your application to handle errors.  However, `panic!` is how a test is marked as a failure; calling `unwrap` or `expect` is exactly what should happen.  It is also appropriate to call `unwrap` when you have some other logic that ensures the `Result` will have an `Ok` value, but the logic isn't something the compiler understands. 

### The ? Operator 
`?` is a shortcut for propagating errors, e.g.: 
```
fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
If the value of the `Result` in a `Result?` is an `Ok`, the value inside the `Ok` will get returned from this expression, and execution continues within the function.  If the value is an `Err`, the `Err` will immediately be `return`ed from the whole function.
Error values that have the `?` operator called on them go through the `from` function, to coerce them into the return type of the parent function.  
`?` can be chained on to multiple method calls.
`?` can only be used on expressions whose values are compatible with the parent function's return type.  
You can use `?` on a `Result` in a function that returns `Result`, and you can use the `?` operator on an `Option` in a function that returns `Option`, but you can't mix and match.  

There are restrictions on what `main`'s return type can be other than `()`, but it can return a Result<(), E>, e.g.:

```
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```
As in C, the executable will exit with a value of 0 if `main` returns `Ok(())` and will exit with a non-zero value if main returns an `Err` value.  
`main` can return any types that implement the std::process::Termination trait.

## Chapter 10 Generic Types, Traits, and Lifetimes. 

### Generic types.  
Declaration of a function using generic type T: `fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { ...`.  
And of a struct: `struct Point<T> {` and an `enum Option<T> {`.
They can be used in method definitions: 
```
impl<T> Point<T> {
    fn x(&self) -> &T {
        
```
If the `<T>` after `impl` is omitted and a concrete type given, the same form can be used to only implement methods on structs for specific types.  

Monomorphization of code using generics at compile time means performance is not affected (post-compile multiple dispatch?).

### Traits.
Traits are like interfaces with some differences.  Custom traits can be used as the parameter type in a function's signature using `impl`. A type's behaviour consists of the methods we can call on that type. 
`Copy`, `Clone` and `Drop` are important built in traits that control ownership.
 
Trait definitions are a way to group method signatures together to define a set of behaviours.  
Example trait definition:
```
pub trait Summary {
    fn summarize(&self) -> String;
}
```
Each type implementing `Summary` must implement `summarize`, e.g.:
```
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        ...
    }
}
```
Users must bring a Trait into scope as well as its implementation: `use aggregator::{Summary, NewsArticle};`.   

Traits can be implemented on a type only if the trait or the type is local to our crate.  External traits can't be implemented on external types - _Coherence_ / the "_orphan rule_".  This ensures other people's code can't break your code and vice versa (by avoiding ambiguity from conflicting Trait implementations).  

Trait definitions can have a default implementation.  These can be used e.g. as `impl Summary for NewsArticle {}`, and the default method can then be called on an instance of it.

Using a Trait in a function signature:
```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```
#### Trait bounds:
Allow different types both implementing Summary:
```
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
```
Force both types to be the same type (implementing Summary):
```
pub fn notify<T: Summary>(item1: &T, item2: &T) {
```
Requiring more than one trait with `+`:
```
pub fn notify(item: &(impl Summary + Display)) {
pub fn notify<T: Summary + Display>(item: &T) {
```
##### where clauses
enable making long lines such as:
```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```
into shorter ones such as:
```
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
```
##### return types
```
fn returns_summarizable() -> impl Summary {
```    
This can only be used if a function returns a single type.  Returning multiple types implementing the same trait is not supported by `impl` (Trait Objects are needed, e.g. a Vec of Boxes of implementations, and monomorphisation cannot be used).

#### Conditional Implementation of Methods and other Traits
```
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
```
An example "Blanket implementation":
```
impl<T: Display> ToString for T {
    // --snip--
}
```

### Lifetimes.
Lifetimes ensure that references (/ borrows denoted by `&`) are valid as long as we need them to be.  
A lifetime is the scope for which a reference (e.g. to a value) is valid. 
We must annotate lifetimes when the lifetimes of references could be related in a few different ways, ensuring the actual references used at runtime will definitely be valid.  
Memory is deallocated when variables go out of scope.  
#### The Borrow Checker...
... compared scopes to determine validity of borrows.
##### Lifetime annotation syntax:
Lifetime annotations in functions go in the signature not the body. 
```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```  
`a` relates to some other lifetime annotation involving `a` (a single annotation is meaningless in isolation).  
```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```  
The function signature tells Rust that for some lifetime `'a`, the function takes two parameters, both string slices that live at least as long as lifetime `'a` (the same as the smaller of the two in practice).  

Function arguments that do not have a relationship to the returned value do not require lifetime annotations.

Lifetime annotations can be used in Structs too:
```
struct ImportantExcerpt<'a> {
    part: &'a str,
}
```  
##### Lifetime Elision.
Default elicited function signatures such as this below no longer require lifetime annotations ("lifetime elision rules").
```
fn first_word<'a>(s: &'a str) -> &'a str {
```
Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.  
##### Lifetime elision rules:
 1) The compiler assigns a unique lifetime parameter to each parameter that's a reference, e.g. a func with one parameter: `fn foo<'a>(x: &'a i32)`, or a func with two parameters: ` fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`. 
 2) If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32) -> &'a i32`
 3) If a method's input lifetime parameter is &self or &mut self, the lifetime of self is assigned to all output lifetime parameters.
If after applying these three rules, the lifetime of  a parameter cannot be determined, Rust will not compile the code. 

`'static` means a reference lives for the entire duration of the program.

## Chapter 11.  Automated testing.
Tests are Rust functions that verify that the non-test code is functioning in the expected manner. 
Tests:  
 1) Setup  
 2) Run  
 3) Assert  
Tests are functions that have the `test` attribute - `#[test]` on the line before `fn`.  
`cargo test` builds a test runner binary that runs the annotated functions and reports pass/fail.
Default test template from `cargo new adder --lib`:
```
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
```  
`assert_eq!` and `assert!` are macros.  The latter takes a Boolean (negate a test that passes when `false` using `!`).  
For structs and enums implement `PartialEq` and `Debug` (e.g. `#[derive(PartialEq, Debug)]` as both are derivable traits) to use them in `assert_eq!` and `assert_ne!`.  

A custom failure message is formed from  optional arguments to the `assert!`, `assert_eq!`, and `assert_ne!` macros. Any arguments specified after the required arguments are passed along to the `format!` macro.  

Check for panics using `#[should_panic(expected = substring_of_expected_value")]` between `#[test]` and `fn`.  

Tests can also use `Result<T, E>`, e.g.:
```
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
```
The `#[should_panic]` annotation cannot be used on tests that return `Result<T, E>`.
To assert that an operation returns an `Err` variant, use `assert!(value.is_err())`, not the question mark operator.

`--` separates command line options from those that go to `cargo test` and those that go to the test binary. 
### Parallel Testing.
By default tests run in parallel.  Make sure tests do not depend on each other or on any shared state (including files or env variables).
`cargo test -- --test-threads=1` tests with no parallelism, but is slower.
`cargo test -- --show-output` shows the output from successful tests as well as failed ones.
`cargo test test_name` only runs all test functions whose names contain  `test_name` as a substring.
To ignore a test unless requested:
```
#[test]
#[ignore]
fn expensive_test() {
```
`cargo test -- --ignored` runs only the ignored tests.
`cargo test -- --include-ignored` runs ignored and not ignored ones.

To test private functions (not marked `pub`):
```
#[cfg(test)]
mod tests {
    use super::*;
```
Integration tests use your library the same way any other code would, via its public API.
Put integration tests in a `\tests` directory (next to `src`).
To make a common shared module in `\tests` use `tests/common/mod.rs` so it isn't treated as a test file.

Integration tests cannot test code in `main.rs` in binary crates.  Code to be tested should be in `lib.rs`.  

## Chapter 12.  Project.

## Chapter 13. Iterators and Closures.
### Closures. 
Closures are anonymous functions that can capture values from the scope in which they're defined.  They can be saved in a variable or passed as arguments to other functions.

Example:
```
#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
```
The `unwrap_or_else` method on `Option<T>` is defined by the standard library. It takes one argument: a closure without any arguments that returns a value `T` (the same type stored in the `Some` variant of the `Option<T>`, in this case, a `ShirtColor`). If the `Option<T>` is the `Some` variant, `unwrap_or_else` returns the value from within the `Some`. If the `Option<T>` is the `None` variant, `unwrap_or_else` calls the closure and returns the value returned by the closure.

Closures don't usually require you to annotate the types of the parameters or the return value like `fn` functions do. Closures aren't used in an exposed interface like this: they're stored in variables and used without naming them.  

Nonetheless, if required, type annotations can still be added to closures like so:
```let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

Compare and contrast:
```
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

The same unannotated closure cannot be called twice with args of different types.  The following results in a type mismatch error:
```
let s = example_closure(String::from("hello"));
let n = example_closure(5);
```

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. The closure will decide which of these to use based on what the body of the function does with the captured values.

To force a closure to take ownership of the values it uses in the environment, use the move keyword before the parameter list. 

Once a closure has captured a reference or moved a value into the closure, the code in the body of the function also affects what happens to the references or values as a result of calling the function. A closure body can move a captured value out of the closure, can mutate the captured value, can neither move nor mutate the captured value, or can capture nothing from the environment. The way a closure captures and handles values from the environment affects which traits the closure implements. The traits are how functions and structs can specify what kinds of closures they can use.

Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion:

 1) `FnOnce` applies to closures that can be called at least once. All closures implement this trait, because all closures can be called. If a closure moves captured values out of its body, then that closure only implements `FnOnce` and not any of the other Fn traits, because it can only be called once.
 2) `FnMut` applies to closures that don't move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
 3) `Fn` applies to closures that don't move captured values out of their body and that don't mutate captured values. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently. Closures that don't capture anything from their environment implement `Fn`.

For example, see the definition of `unwrap_or_else`:

```
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
```

Functions can implement all three of the `Fn` traits too. If what we want to do doesn't require capturing a value from the environment, we can use the name of a function rather than a closure where we need something that implements one of the `Fn` traits. For example, on an `Option<Vec<T>>` value, we could call `unwrap_or_else(Vec::new)` to get a new, empty vector if the value is None.

`list.sort_by_key` on the other hand, requires a `FnMut` trait bound on its arg, as the closure passed to it will be called for each element of the list being sorted.  `FnMut` implementers e.g. cannot contain `sort_operations.push(value);` as if `value` is immutable, they move value out of their body.  Such closures can only be called once if `value` is not mutable.  

### Iterators. 
Rust iterators are lazy. 
E.g. Vectors have an `.iter` method that creates an iterator.
`for` loops implicitly create and consume iterators under the hood:
```    
for val in v1_iter {
    println!("Got: {}", val);
}
```  
Iterators avoid the need for indexing vectors.  

#### The Iterator Trait
All iterators implement the `Iterator` trait.  Ths requires them to have a method called `.next`.  They also define a type (by `type Item`).  The objects returned from the iterator will have type `Item`.  The `Iterator` trait has an "associated type".

To call `.next` on it, an iterator needs to be mutable (`for` loops do this behind the scenes). 
`.next` on `.iter` returns immutable references to values in a vector. To take ownership of values, create the iterator with `.into_iter` instead.
Similarly, use `.iter_mut` for mutable references.

The Iterator Trait has a number of default methods that consume the iterator - called "consuming adaptors", e.g.:
`.collect` can gather items returned from iterators into a container, e.g. a Vector.

Other methods are called "iterator adaptors" as they return other iterators, e.g.:
`.map` on an iterator, takes a closure and returns another iterator (of the closure called on each item of the original).

`.filter` returns an iterator for which each returned element evaluated to true under the provided closure that maps to a Boolean.  

Iterators are about the same speed as for loops. 
Iterators are one of Rust's "zero-cost abstractions". 

## Chapter 14.  Cargo and crates.io 

### Documentation Comments.
Documentation comments use three slashes `///` instead of two.  Markdown notation is supported.  It will generate html documentation.

Commonly used sections in doc comments:
 - Examples
 - Panics.  Scenarios that could result in a panic.
 - Errors.  If the function returns a Result. 
 - Safety.  If the function is unsafe. 

### Documentation Comments as Tests.
`cargo test` will run the code in examples in doc comments as a test.

The style of doc comment `//!` adds documentation to the item that contains the comments rather than to the items following the comments, typically inside the crate root file (e.g. `src/lib.rs`). 

`pub use` statements can re-export deeply nested items to provide a nicer API for end users. 

## Chapter 15.  Smart Pointers.
The most common kind of pointer in Rust is a reference, indicated by `&`, which borrow the value they point to.  They have no other special capabilities and no overhead.  

*Smart pointers* such as `String` and `Vec<T>`, have additional metadata and capabilities. In many cases, smart pointers own the data they point to. 

Smart pointers are usually implemented using structs that implement the `Deref` and `Drop` traits (unlike ordinary structs).  The `Deref` trait allows an instance of the smart pointer struct to behave like a reference. The `Drop` trait allows customisation of the code run when an instance of the smart pointer goes out of scope. 

Other examples include:
 - `Box<T>` for allocating values on the heap
 - `Rc<T>`, a reference counting type that enables multiple ownership
 - `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces the borrowing rules at runtime instead of compile time

### Boxes. 
`Box<T>` stores data on the heap rather than the stack (the pointer is on the stack). They have no performance overhead. They can be used for:
 - Types whose sizes are unknown at compile time, whose values need to be used in contexts with exact sizes
 - Avoiding copying data when transferring ownership
 - Owning a value but only caring about the Traits it implements, rather than it being of a specific type. 

`let b = Box::new(5);` definition of a Box that points to the value `5` on the heap. 

#### Recursive Types with Boxes.

##### Cons List.
A cons list is a data structure made up of nested pairs, e.g.: `(1, (2, (3, Nil)))`.  Define them as:
```
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};
```

### The Deref Trait.
The `Deref` trait customises the behaviour of the dereference operator `*`.  
```
use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```
(`.0` refers to the first element of a tuple).  
When running `*y`, behind the scenes Rust actually runs: `*(y.deref())`.
`.deref` returns a reference to a value, but a plain dereference outside the parentheses in *(y.deref()) is still necessary as otherwise the value would be moved out of self. We don't want to take ownership of the inner value in most cases where we use the dereference operator `*`. 

### Deref Coercion. 
Deref coercion converts a reference to a type that implements the `Deref` trait into a reference to another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait such that it returns `&str`. Deref conversion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the `Deref` trait. Deref coercion was added to Rust so that programmers writing function and method calls don't need to add as many explicit references and dereferences with `&` and `*`. 

There is no runtime penalty for taking advantage of deref coercion.

#### DerefMut.
The `DerefMut` trait overrides the `*` operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:
 - From `&T` to `&U` when `T: Deref<Target=U>`
 - From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
 - From `&mut T` to `&U` when `T: Deref<Target=U>`

### The Drop Trait. 
`Drop` (in the prelude) customises what happens when a value is about to go out of scope. You can implement the `Drop` trait on any type, e.g. to release resources like files or network connections. 

This avoids the need to call code to free memory or resources, e.g. file handles, sockets and locks.

Example:
```
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
```
Rust automatically calls `.drop` when instances of `CustomSmartPointer` go out of scope.  
The Drop trait's drop method cannot be called manually; instead call `std::mem::drop` to drop a value early. 

### Rc<T>, the Reference Counted Smart Pointer.
Multiple ownership must be explicitly enabled in Rust, e.g. using `Rc<T>` (only for use in single-threaded scenarios).  
`Rc::clone` doesn't make a deep copy, it only increments the reference count (viewable via `Rc::strong_count`), so does not cause performance problems. 
`Rc<T>` allows you to share data between multiple parts of your program for reading only.  To mutate shared data, use the interior mutability pattern and the `RefCell<T>`. 

### RefCell<T> and the Interior Mutability Pattern
`RefCell<T>` is only for use in single-threaded scenarios. 
*Interior mutability* is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data. It uses `unsafe` code inside a data structure to bend Rust's usual rules that govern mutation and borrowing.
Recall the borrowing rules:
 - At any given time, you can have either (but not both) one mutable reference or any number of immutable references.
 - References must always be valid.

With references and `Box<T>`, the borrowing rules' invariants are enforced at compile time. With `RefCell<T>`, these invariants are enforced at runtime. With references, if you break these rules, you’ll get a compiler error. With `RefCell<T>`, if you break these rules, your program will `panic` and exit.


 - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
 - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
 - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

Mutating the value inside an immutable value is the *interior mutability* pattern. 
`RefCell<T>` doesn't get around the borrowing rules completely: the borrowing rules are checked at runtime instead. If you violate the rules, you'll get a `panic!` instead of a compiler error.

#### Keeping Track of Borrows at Runtime with RefCell<T>. 
When creating immutable and mutable references, we use the `&` and `&mut` syntax, respectively. With `RefCell<T>`, we use `.borrow` and `.borrow_mut`.  `.borrow` returns the smart pointer type `Ref<T>`, and `.borrow_mut` returns the smart pointer type `RefMut<T>`. Both implement `Deref`, so we can treat them like regular references. `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart pointers are currently active. 

#### Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>. 
An `Rc<T>` that holds a `RefCell<T>`, implements a value that can have multiple owners, and that you can mutate.

#### Other Smart pointer types providing interior mutability. 
 - `Cell<T>` is similar to `RefCell<T>` but instead of giving references to the inner value, it is copied in and out.  
 - `Mutex<T>` offers interior mutability that's safe to use across threads. 

### Reference Cycles Can Leak Memory. 
Rust's memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak). Preventing memory leaks entirely is not one of Rust's guarantees, meaning memory leaks are memory safe in Rust. 
Creating reference cycles is not easily done, but it's not impossible either. 
#### Weak references.
Weak references to a value within an `Rc<T>` instance can be created by calling `Rc::downgrade` and passing a reference to the `Rc<T>`
Weak references don’t express an ownership relationship, and their count doesn't affect when an `Rc<T>` instance is cleaned up. They won't cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

`Rc::downgrade` returns a smart pointer of type `Weak<T>`. Instead of increasing the strong_count in the `Rc<T>` instance by 1, calling `Rc::downgrade` increases the weak_count by 1. The `Rc<T>` type uses weak_count to keep track of how many `Weak<T>` references exist, similar to strong_count. The difference is the weak_count doesn’t need to be 0 for the `Rc<T>` instance to be cleaned up.

Because the value that `Weak<T>` references might have been dropped, to do anything with the value that a `Weak<T>` is pointing to, you must make sure the value still exists. Do this by calling `.upgrade` on a `Weak<T>` instance, which will return an `Option<Rc<T>>`.  This will be `Some` if the `Rc<T>` value has not been dropped yet and a result of `None` if the `Rc<T>` value has been dropped. Hence there won’t be an invalid pointer. 

## Chapter 16. Concurrency.
 - Concurrent programming: different parts of a program execute independently.  
  - Parallel programming: different parts of a program execute at the same time.
Concurrency is called fearless in Rust because its ownership and type checking system helps manage many concurrency errors, making them compile-time errors, rather than runtime errors.   

### Threads. 
To create a new thread, call  `thread::spawn` with a closure. 
New threads will be interweaved with the main thread.
To guarantee a spawned thread will get to run at all, save the return value of `thread::spawn` in a variable of type `JoinHandle`, and call .join().unwrap() on it where it should be blocking. 
Adding the `move` keyword before a closure, forces it to take ownership of the values it's using rather than allowing Rust to infer that it should borrow the values.

### Message Passing to Transfer Data Between Threads.
To accomplish message-sending concurrency, Rust's standard library provides an implementation of a channel, a general programming concepts by which data is sent from one thread to another.  
A channel is directional and has two halves: a transmitter and a receiver: 
```
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
}
```

mpsc stands for multiple producer, single consumer - multiple sending ends that produce values but only one receiving end that consumes those values. 
tx and rx are traditionally used in many fields for transmitter and receiver. 
tx has a `send` method that takes the value to be sent, returning a `Result<T, E>` in case the receiver has already been dropped. 
rx has `recv` and `try_recv` methods.
`recv` will block the main thread's execution until a value is sent down the channel, returning a `Result<T, E>`. 
`try_recv` doesn't block, but will instead return a `Result<T, E>` immediately: an `Ok` value holding a message if one is available and an `Err` value if there aren't any messages this time.
A receiver, e.g. rx can be treated as an iterator.  
Multiple producers can be created by cloning the transmitter (e.g. tx). 

### Shared-State Concurrency. 
Mutex is an abbreviation for *mutual exclusion*, i.e. a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex's lock. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. 
`Mutex<T>` is a smart pointer.  Moreover, `.lock` returns a smart pointer called `MutexGuard`, wrapped in a `LockResult`.  `MutexGuard`  implements both `Deref`, and `Drop` which releases the lock automatically when a `MutexGuard` goes out of scope. 
Wrap a `Mutex<T>` in an `Arc<T>` to give it multiple owners (Atomic Reference Counter).  Atomics have a performance penalty. 
`Mutex<T>` comes with the risk of creating deadlocks.

### The Sync and Send Traits
Rust actually has very few concurrency features, but two concurrency concepts are embedded in the language: the `std::marker` traits `Sync` and `Send`. 

The `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. Almost every Rust type is Send, but there are some exceptions, including `Rc<T>`  (`Arc<T>` is Send).
Any type composed entirely of Send types is automatically marked as Send as well.

The `Sync` marker trait indicates that it is safe for the type implementing `Sync` to be referenced from multiple threads. In other words, any type `T` is `Sync` if `&T` (an immutable reference to `T`) is `Send`.  Primitives and `Mutex<T>` is `Sync`.  Types composed entirely of types that are `Sync` are also `Sync`. 

Manually implementing Sync and Send involves unsafe Rust code. 

## Chapter 17 OOP.
Rust has similarities to OOP languages, but does not support inheritance.  
There is no way to define a struct that inherits the parent struct's fields and method implementations. 
Code can still be shared using default Trait method implementations. 
### Polymorphism. 
Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types must provide. This is sometimes called bounded parametric polymorphism. 
Inheritance has recently fallen out of favour because it shares more code than necessary. 
Rust uses trait objects instead of inheritance to enable polymorphism.
### Trait objects. 
A trait object points to both an instance of a type implementing our specified trait as well as a table used to look up trait methods on that type at runtime. We create a trait object by specifying some sort of pointer, such as a `&` reference or a `Box<T>` smart pointer, then the `dyn` keyword, and then specifying the relevant trait. 
```
pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```
We can't add data to a trait object; their specific purpose is to allow abstraction across common behaviour. 
A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime. If you'll only ever have homogeneous collections, using generics and trait bounds is preferable because the definitions will be monomorphised at compile time to use the concrete types. 

#### Dynamic Dispatch
When we use trait objects, Rust must use dynamic dispatch. 
There is a runtime cost when this lookup happens that doesn’t occur with static dispatch. 

## Chapter 18 Pattern Matching.
Patterns are a special syntax in Rust for matching against the structure of types.  A pattern consists of some combination of the following:
 - Literals
 - Destructured arrays, enums, structs, or tuples
 - Variables
 - Wildcards
 - Placeholders

It's possible to mix and match `if let`, `else if`, and `else if let` 
expressions.  This allows more flexibility than a match expression in which we can express only one value to compare with the patterns. 

`if let Ok(age) = age` introduces a new shadowed age variable - we need to place an `if age > 30` condition within that block: we can't combine the two conditions into `if let Ok(age) = age && age > 30` as the shadowed `age` isn't valid until the new scope starts with the curly bracket.

When using `if let` expressions, the compiler doesn't check exhaustiveness. 

### Loops.
the `while let` conditional loop allows a while loop to run for as long as a pattern continues to match. 

In a for loop,  `for x in y`, `x` is the pattern.

### Let statements.
Formally, a let statement looks like `let PATTERN = EXPRESSION;`.

### Function Parameters
Function parameters can also be patterns, e.g.:
```
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
```
We can also use patterns in closure parameter lists.  

### Refutability.
Function parameters, `let` statements, and `for` loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match. 
`if let` and `while let` expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they're intended to handle possible failure. 
`match` arms must use refutable patterns, except for the last arm.

In `match` expressions, you can match multiple patterns using the `|` syntax, which means or.
The `..=` syntax allows us to match to an inclusive range of values. 
Ranges are only allowed with numeric values or char values.

We can also use patterns to destructure structs, enums, and tuples to use different parts of these values. 
```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```
We can also destructure with literal values as part of the struct pattern rather than creating variables for all the fields. 
```
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

The pattern to destructure an enum should correspond to the way the data stored within the enum is defined.  
Matching can work on nested items too!

With values that have many parts, we can use the `..` syntax to use only a few parts and ignore the rest, avoiding the need to list underscores for each ignored value. The `..` pattern ignores any parts of a value that we haven't explicitly matched in the rest of the pattern.   Using `..` must be unambiguous. 

### Extra Conditionals with Match Guards
A match guard is an additional if condition specified after the pattern in a match arm that must also match, along with the pattern matching, for that arm to be chosen.  The condition can use variables created in the pattern. 
```
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
```
The compiler doesn't try to check for exhaustiveness when match guard expressions are involved.  
Match guards solve some pattern-shadowing problems:
```
fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);
}
```
If multiple patterns are specified with `|`, the match guard condition will apply to all the patterns.  

### @ Bindings.  
The at operator (@) lets us create a variable that holds a value at the same time we’re testing that value to see whether it matches a pattern. 

## Chapter 19, Advanced Features. 

### Macros. 
Macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type. A function can't, because it gets called at runtime and a trait needs to be implemented at compile time.

You must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere. 

