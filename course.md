# Programming in Rust: a shortened Book
Links to the Book:
+ **The _Book_** (for learning Rust): <https://doc.rust-lang.org/book/>  
+ Command to get the Book offline: `rustup doc --book`
+ **The translations** of the Book (works in progress): <https://doc.rust-lang.org/book/appendix-06-translation.html>  
+ Direct link to the **French translation**: <https://jimskapt.github.io/rust-book-fr/>  

I wrote a Rust guide to which I can refer if I need expainations so don't take it thiking you can learn Rust with it, that's a bad idea.  
This guide is based on [the Book](https://doc.rust-lang.org/book/): a 20 chapter guide to learn Rust written by the Rust team. It is very well explained so do not hesitate to check it out if you  need another example.  
I divided this guide the same way the Book does for chapters (in the form `1.`, `2.`, ...) and sections (in the form `2.5.`, `5.3.`, ...) but I added sub-sections (`3.2.1.`, ...) and sub-sub-section in order to have more links to jump to (I assume your mardown editor has an automatic index feature).  
I also tried to make this guide more interactive than the Book by adding a few exercises here and there. The Book give a link to a [more interactive version to the Book](https://rust-book.cs.brown.edu/) if you think you aren't coding enough by yourself.  
The Book has two kind of chapters: concept chapters and project chapters, _concept chapters_ are a just explaination of a Rust concept whereas _project chapters_ are showing how to build a project, they are serving of base for concept chapters.
Sadly there are only 3 project chapters: [chapter 2.](#2-guessing-game-project-1), [chapter 12.](#12-) and [chapter 20.](#20-)

I can sometimes write `//-> ` or `//out-> ` behind a line of code this is to show what the output of the function will be, in the case of `//-> x` the function/macro returns `x` and in the case of `//out-> x` the function will print the value of `x`.  
If you are confused by what I just wrote ignore it, you can come back when you will.

## 1. Introduction
### 1.0. The terminal
This section is only for those who are not comfortable with the terminal. What's the terminal? You know the program with a black background and green text the hackers uses to get your IP address.  

I am using zsh on Linux Mint so some syntax may vary with your shell/operating system.

A definition for your culture: a **shell** is a programming language used in a terminal, there exist many shells like [_zsh_](https://www.zsh.org/) (Z shell), [_bash_](https://www.gnu.org/software/bash/) or Powershell on Windows.

The term _CLI_ means _Command Line Interface_ and is opposed to _GUI_ that means _Graphical User Interface_, remember when downloading tool like Github which have the two versions.

#### 1.0.1. Exploring the terminal
Let's begin simple and just open the terminal, on Mint and Windows it is as simple as pressing Windows then type "_cmd_".  
If you're on a Linux-based OS, you should bind a shortcut to the terminal since you just have to use it at least some day. Even if you have Windows or MacOS it can be useful since the compilation and instalation of Rust is through command line.

_command line input examples begin with a \$_  
To run a command, type its name then hit enter and it should run the command, go to a new line because the input is incomplete or straight up give you an error.  
Let's do a Hello World for fun, run this command:
```bash
$ echo Hello World!
```
you should obtain a similar result:
```bash
$ echo Hello World!
Hello World!
```
If you don't, try putting single or double quote before and after the message, like this:
```bash
$ echo "Hello World!"
$ echo 'Hello World!'
```

#### 1.0.2. Paths
When writing paths (to run a execuatble for examples (`.exe` on Windows)) in the command line (in the terminal), you have two choices:
- Type the full path to the file/folder (its _absolute path_): `C:/my/favorite/path` or `~/I/love/this/path`
- Type the path from where you are in your file hierarchy: `./file` or simply `file` on Linux

The second option is the one you'll see the most because it is shorter.  
There are two symbols to be aware of when writing **relative paths**:
- `.` : Your current location (see next paragraph).
- `..`: The parent folder of your current location (ie: `C:/I/Like/me/mes/..` is `C:/I/Like/me`)

Look at the left of your command prompt (where you write commands), you have a path to a folder, it is your current location.  
You can change it with the command `cd`. If you did understood what I just said, you know that:  
`cd . ` does nothing and  
`cd ..` leads you one level above

If you don't want go through all this (I can understand), you can go in the file explorer, right-click in the folder you want to be and click on the _Open in terminal_ option and it will open a fresh new terminal.

_note: If there's spaces in path (I discourage it because it can bug some commands), you have to put single or double quotes around it else it will be treated as two separate files._

#### 1.0.3. Files
To create a new folder run the command 
```bash
$ mkdir <folder name>
```
And to create a new text file run:
```rust
$ echo "" > <file name>
```
If you want to create it with a message just put it between the quotes:
```bash
$ echo "I am a new file" > <file name>
```

We can use the `cat` (Linux) or `type` (Windows) to get the content of it:
```bash
$ echo "Wryy" > Dio.txt
$ cat Dio.txt
Wryy

$ echo "Wryy" > Dio.txt
$ type Dio.txt
Wryy
```
Similarily, you can use the `ls` command to see the files and folder in it.


If you want to remove a file use:
```bash
$ rm <file>		# on Linux
$ rd <file>		# on Windows
```

To remove a folder, it's more complicated.  
You have to add the flag `-r` on Linux and `/s` on Windows's command prompt:
```bash
$ rm -r <folder>
$ rd /s <folder>
```

There's a joke in the Linux community that is to run the command `sudo rm -rf /` to fix an issue. In case you don't know it, **$\color {red} \text{DO NOT EVER RUN IT}$** unless you know what you're doing **and** have a copy of your files.  
If you don't understand the joke, the `-rf` flag is a contraction of `-r -f` the second flag being a shorthand for `--force` that tells the command to not ask for confirmation; the `/` at the end is the directory to delete, it is the _root directory_ which is parent to all other directories. The `sudo` at before the command is to run the command as the root user (which has all rights like an admin). In short, **it deletes all files present on drives where Linux has access** (which is generally all).  
If you're curious, here is the Windows equivalent: `rd /s /q C:`.

#### 1.0.4. Get help
Firstly, **do not ever run a command nor code that you don't understand**. It could be a hack, buggy or just damaging to your computer.  
So just google anything that you don't understand especially if there is the prefix `sudo` or anything that can have an impact on the computer.  
There is documentations out there that explain what's going on (even a question to ChatGPT is better than nothing).

With command line applications, it's simpler. You can have a documentation for any command using the arguments `-h` or `--help`, if it doesn't work you can try the `man` (Linux) or `help` (Windows) with this syntax:
```bash
$ man  <command name> <argument?>
$ help <command name> <argument?>
```
(ex: `help help`, `man git`).

#### 1.0.summary
Wow! Even I didn't think that this introduction was going to get this big (~100 lines in raw markdown). I hope it will help you to understand what's going on in the installation process. I also hope it demystified the myth of only hackers using the terminal, in fact everybody can use it on any OS.

Here is a bonus tip: if you are on Windows you can type the command `color` to change the text color, to mimic hackers I like to enter `color A`. To replicate this on bash and zsh, you have to change respectively the `.bashrc` and `.zshrc` files in your home folder.

With this in mind, let's see the main subject: How to code in Rust!

### 1.1. Installing Rust
**It's the 06/11/24 ($6^\text{th}$ of November)**

We use CURL to install rustup:
`$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh`

The Book indicates that _"A C linker can be useful (I personally use gcc)"_.


> We can update Rust with 
`$ rustup update`

And uninstall Rust with `$ rustup self uninstall`

`$ rustup doc` shows the local doc

Rust version: `rustc `**`1.82.0 `**`(f6e511eec 2024-10-15)`

### 1.2. Hello World
#### 1.2.1. Writing and running a program
The Rust extention is **`.rs`** (`example.rs`).  

The basic compilation command is `$ rustc [filename].rs`.  
To run it, just `$ ./[filename]`.

It means you can just write this line to build and immediatly run a rust project:
```bash
$ rustc file.rs; ./file
```

#### 1.2.2. Anatomy of a program
_(I say "Cs" as the plural of C, It means C ans C++, unlike CS which is C# or C-like which include C, C++ and C#)_

Let's analyze this Hello World program:
```rust
fn main() {
	println!("Hello, world!");
}
```

We can see all the code is inside the main function (like in Cs).
The function does not require any staticly inferred type and isn't required to return smth (as in the _`void`_ type).

The _Book_ says _"Rust style is to indent with **four spaces**, not a tab"_ but I'll just continue to use tabs because.  

`println!()` is not a function but a macro (a function would be called without the '!'), they say that it'll be in [chapter 19.](#195-Macros) (19.5. by looking at the titles) on 20 chapters so be patient!

We also see that **there's a semicolon** at the end of the Hello World, the Book says : _"Most lines of Rust code end with a semicolon."_ I'm guessing those are at the end of instructions like C++ does.


### 1.3. Hello, Cargo!
Cargo handles dependencies (a bit like pip or composer do).

To create a new Cargo project, use `$ cargo new [project name]`, 
it'll create a new folder containing the project and a new Git repository.
The structure is:  
- _[project name]_ 
	+ src
		- main.rs
	+ .git
		- _[git stuff...]_
	+ Cargo.toml
	+ .gitingnore

_note: Git files won’t be generated if you are in an existing Git repository;
 you can override this behavior by using `$ cargo new --vcs=git` instead_

Let's talk about the **Cargo.toml** file:  
```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
```
_TOML_ stands for _Tom's Obvious, Minimal Language_ ([TOML doc](https://toml.io/en/)).  
The section **_package_** contains all things necessary to the cargo compilation.  
The section **_dependencies_** contains all... dependencies, it is currently empty, we'll fill it in the chapter 2 for our first project.

When we look into the  **main.rs** file, we see the Hello World program from earlier.

#### 1.3.2. Building and running a Cargo project
To build a Cargo project, simply use: `$ cargo build`, the executable file is at `target/debug/[project name]`.
But no need to run the old `& [file path]` just use the `$ cargo run` command!  
With `$ cargo check` you can check if your code is correct without having to create an executable which is much faster.

Here is the [Cargo doc](https://doc.rust-lang.org/cargo/).

### 1.summary
In this chapter, we've seen a lot of new concepts (ie: cargo and `rustc` commands), and it may take a lot of time to get the hang of it; especially if you've never coded (terminals was a hacker thing)

## 2. Guessing Game (project 1)
**it's the 09/11**

###  2.1. First steps
I skip the creation of the Cargo project because It was last section.

The book introduce how **define a variable** and how to get the user's input.

Here's the code:
<span id="anchor21"></span>
```rust
use std::io;

fn main() {
	println!("Guess the number!");

	println!("Please input your guess.");

	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed: {}", guess);
}
```  

Let's analyze it line by line:


> `use std::io;`  

This import the `io` library in the scope. All library that don't need to be imported with `use` (as the String type) are in the [**Prelude**](https://doc.rust-lang.org/std/prelude/index.html).


> `let mut guess = String::new();`

To declare a variable in Rust we can do like if we were in JS `let var = 5;` but `var` will be immutable (cannot change), in order to change it we have to add the keyword `mut`: `let mut myVar = 5;`.  
This concept will be detailed in [chap 3.](#311-reminders).

`String::new()` is the method to create a string object.
So `let mut guess = String::new();` creates the mutable varible `guess` and assign it to an empty string.


> `io::stdin().read_line(&mut guess)`
```rust
io::stdin()
	.read_line(&mut guess)
	.expect("Failed to read line");
```
is exactly the same as 
```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
```
We write it like that to clarify and shorten the amount of text by line.

Even we didn't wrote the `use std::io` line, we could still use this module we'd just have to write its full name.

`io::stdin` is a type which let us handle the inputs in the terminal.  
The `read_line` method will read the user's input and *append* it at the end of a string.
Here we pass `&mut guess` as a argument, it means that we pass `guess` as a **mutable reference**; it will be explained in chapter 4 but for now a reference in a function/method will means that the function can modify the variable (without the `&mut` only a copy of `guess` would had been given to the function).


`.expect("Failed to read line");` will print string passed to it only if an error occurs in `.read_line()`.  
The `.read_line()` will return a value of type `Result`, this type is an _enum_ it means it can only be a defined set of values, for the `Result` type those values are `Ok` and `Err` so the `.read_line()` method will return `Ok` if it printed the string on the screen or `Err` if it didn't; the `expect` method will simply be ignored if `.read_line()` returns `Ok`. 
(_more on enums in chapter 6_)



> `println!("You guessed: {}", guess);`

The curly brackets will print the value of `guess`: this is a _placeholder_, the Book gives you this mnemonic technique: _"think of `{}` as little crab pincers that hold a value in place"_.  
Another way of doing it would be `println!("You guessed: {guess}");`.

### 2.2. Generating a (pseudo) random number
#### 2.2.1. Installing the `rand` crate
For now, the standard library (`std`) does not include a random number generator but the developers provide a _crate_ for this purpose.  
A _crate_ is a collection of Rust code.  
A _library crate_ is code that is not meant to run on it's own.  
Until now, we've been building only _binary crates_ (executables).

In order to include we first need to modify our _Cargo.toml_ file. Under the `[dependencies]` line, paste this line: `rand = "0.8.5"` (pay attention to the version number).  
Cargo uses [**Semantic Versioning**](https://semver.org/) (SemVer), the standard to write version numbers.  
The specifier `0.8.5` is a shorthand for `^0.8.5` which means we are searching for a version that verifies $0.8.5 \leqslant \text{version} \lt 0.9.0$ .

Now if we build without changing the code, you can see that Cargo downloads and compiles a lot of file that are either `rand`'s or its includes.  
Cargo finds its packages on <https://crates.io> where people post their open source Rust project for other Rustaceans (people using Rust) to use.

If you want, to update your crates you can run the command `$ cargo update` which will update all your dependencies **according to your _Cargo.toml_ file**. It means that if the Rust team posts two new versions: the _0.8.7_ and the _0.9.1_, Cargo will update to the _0.8.7_ because you specified you wanted the versions _^0.8.5_ .

There's a lot more to say about Cargo and its ecosystem but we'll talk in [chapter 14](#14).

#### 2.2.2. Implementing random
Include the `rand` crate with this line:
```rust
use rand::Rng;
```
and generate the number with that line:
```rust
let secret_number = rand::thread_rng().gen_range(1..=100);
```
The `rand::thread_rng()` function will provide us a random number generator and the `gen_range()` use it to generate a number between 1 and 100. The notation `start..=end` means that we select a number between `start` and `end` include ( a number in $[\text{start}; \text{end}]$)

_note: If you don't know what crate to include, you can run `$ cargo doc --open`, it will display the documentation of each crate included in your project as HTML (webpage)._

### 2.3. See if the user was right
Now modify your _main.rs_ to look like this: 
```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1..=100);
	
	println!("(psst, the number is: {secret_number})");
	println!("Please input your guess.");

	let mut guess = String::new();
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	println!("You guessed: {guess}");

	match guess.cmp(&secret_number) {
		Ordering::Less => println!("Too small!"),
		Ordering::Greater => println!("Too big!"),
		Ordering::Equal => println!("You win!"),
	}
}
```
_This code will **not** compile_

Let's see what changed:
> `use std::cmp::Ordering;`

We import a new enum type: `Ordering`. This type can only have 3 values:
+ `Less`
+ `Equal`
+ `Greater`


> `match guess.cmp(&secret_number) {`...`}`

The `cmp` method will compare `guess` and `secret_number` and return a value from the enum `Ordering`. We test this value in a `match` structure.  

The `match` structure will do the same job as it does in Python or the `switch` expression in other languages. The `match` structure is made of **arms** and each arm is made of a _pattern_ and the code to execute if this pattern matches the tested expression (here `guess.cmp(&secret_number)`).  
All this stuff is a bit complicated for now so we'll see it later.

The intended behavior of this `match` is to print "_Too small!_" if `guess < secret_number`, "_Too big!_" if `guess > secret_number` and "_You win!_" if `guess = secret_number`.

Let's check our code with `$ cargo check`, we get this error: 
> error\[E0308]: **mismatched types**

That's because "_Rust has a strong, static type system_" so Rust will not convert from a type to another if we didn't asked to (unlike Python or Javascript).  
So the error, comes from the fact that Rust cannot compare a string (`guess`) and a number `u32` (`secret_number`).

To fix this, we will insert this line after asking the number.
```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

We declare a new variable called _guess_ of type u32 (a positive number stored on 32 bits) and assign it to guess but in a number.  
Didn't we had a variable called _guess_ before?  
Yes, we did but Rust has a feature called _shadowing_ which let us redeclare a variables several times. After we do this the old value stored in the identifier `guess` is overwritten by its new value. We'll talk again of this in the next chapter (I wrote that a lot of times already but that's because we're only at chapter 2 so be patient!) so for now this feature is only for changing a variable type.

### 2.4. Allowing several attempts
In this aim, we will insert a `loop` loop:
```rust
loop {
	let mut guess = String::new();
	
	//asking a number and converting it

	match guess.cmp(&secret_number) {
		//choices
	}
}
```

The `loop` loop will loop indefinitly so after running your program you will have only two option to stop it: an error and the shortcut `Ctrl + C`.

To stop the program when the user wins, modify the last arm of the `match`:
```rust
Ordering::Equal => {
	println!("You win!");
	break;
}
```
The `break` statement tells to Rust to exit the loop, here it stops the program because there is nothingafter the loop.

### 2.5. Handling invalid input
As says the proverb "_never trust the user!_", until now we trusted the user for entering a valid number, if the user typed a negative number, a number above $2^{32}$ ($429,496,296$) or simply something else than a number the program crashed.

To prevent this, let's modify the redefinition of `guess`:
```rust
let guess: u32 = match guess.trim().parse() {
	Ok(num) => num,
	Err(_) => continue,
};
```

Now if the user type one of the things lissted above, the program will just continue. The underscore (`_`) catches all matches and cannot be used as a variable. The `continue` keyword will skip the whatever is coming next and will force the loop to go to next iteration.

### 2.summary Final code
The book's final result is:
```rust
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1..=100);

	loop {
		println!("Please input your guess.");

		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,
		};

		println!("You guessed: {guess}");

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}
}
```

mine is a bit different:
```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
	println!("Guess the number!");
	let secret_number = rand::thread_rng().gen_range(1..=100);
	
	//println!("(psst, the number is: {secret_number})"); //comented so the game isn't too easy

	println!("Please input your guess.");
	loop {
		let mut guess = String::new();

		io::stdin()
			.read_line(&mut guess)
			.expect("Failed to read line");

		println!("You guessed: {guess}");

	
		//I use an 8 bit positive integer because with 8 bits the maximum is 255.
		//Since the number can't go more than 100, 32-bits is definitly overkill (in fact 7 would be the minimum number of ).
		let guess: u8 = match guess.trim().parse() {	
			Ok(num) => num,
			Err(ans) => {
				println!("{ans}");	//prints the error
				continue;
			},
		};

		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			},
		}
	}
}
```
That was our first project, the next one is [chapter 12.](#12-) and a small one at [section 5.2.](#52-an-example-using-structures).


## 3. Common programming concepts
**10/11**

In this chapter we'll see concept common at many languages.

### 3.1. Variables
#### 3.1.1. Reminders
By default, varibles are immutables and cannot be changed:
```rust
fn main() {
	let x = 5;
	println!("{x}");

	x = 90;	//compile error
	println!("{x}");
}
```

But you can let them be mutable with the keyword `mut`:
```rust
fn main() {
	let mut x = 5;
	println!("{x}");

	x = 90;	//Ok
	println!("{x}");
}
```

#### 3.1.2. Constants
A constant is like an immutable variable: it cannot be changed at runtime. You can declare a constant by replacing `let` 

They have, howerver multiples differences:
+ The name of a constant should be in upper case : `IAmConst` throws a warning but `I_AM_CONST` doesn't.
+ Constants must be staticly typed: `const x = 4;` is invalid but `const x : u8 = 4` is.
+ Constant can only be defined with value computable at compile time:
```rust
fn just2() -> u8 {	// will always be 2
	return 2;
}
fn main() {
	println!("just2(): {}", just2());	//prints 2

	const X :u8 = 2 * 4 + 5;
	println!("X: {X}");		//prints 13

	let y :u8 = just2(); 
	println!("y: {y}");		//prints 2

	const Z :u8 = just2();	//= 2 but is not resolvable at compile-time so throws a compile error
	println!("Z: {Z}");
}
```

#### 3.1.3. Shadowing
Shadowing let you redeclare variables, when a variable is declared several times, it is _shadowed_ by itself.
You can use shadowing for changing the type of a variable, changing its mutability or changing its value for a scope.

```rust
fn main() {
	let x = 5;
	let x = x + 1;
	let mut y = 9;

	{
		//entering a scope
		let x = x * 2;
		y = y * 8;
		println!("The value of x in the inner scope is: {x}");	//out-> 12
		println!("The value of y in the inner scope is: {y}");	//out-> 72
	}

	println!("The value of x is: {x}"); //out-> 6 (the value x had before entering the scope)

	let mut x = 6;
	x = 79;
	println!("The final value of x is: {x}");	//out-> 79
	println!("The final value of y is: {y}");	//out-> 72 (it is the same "y" as the one in the inner scope)
}
```

Now you are comfortable with variables, let's see the data types.

### 3.2. Data types
Rust is a staticly typed language, it implies that the compiler must know each variables' type at any time. So situations like this will make your program crash:
```rust
let guess = "42".parse().expect("Not a number!");
```
That's because the method `.parse()` can return several types (including ints), to fix this just explicitly infer the type:
```rust
let guess :i32 = "42".parse().expect("Not a number!");
```

#### 3.2.1. Scalar types
A scalar type is type that represent one piece of information, in Rust there is 4 scalar types: integers, floating points, booleans and characters (in C++ they are ints, float/double, bool and char).

##### 3.2.1.1. Intergers
###### Introduction			<!--There's somany "#"!-->
An integer (or int) is a number with its decimal part equal to $0$. In programming, we generaly distinct two types of integers: _unsigned_ and _signed_. The first one can represent only positive numbers (including $0$), the _sign_ does not matter with un*sign*ed numbers, it'll always be positive. The signed numbers can represent _positive_ and _negative_ numbers.

There is in total 12 types of integers in Rust:

|     size     |    signed    |   unsigned   |
|:------------:|:------------:|:------------:|
|  **8-bits**  |     `i8`     |     `u8`     |
|  **16-bits** |    `i16`     |    `u16`     |
|  **32-bits** |    `i32`     |    `u32`     |
|  **64-bits** |    `i64`     |    `u64`     |
| **128-bits** |   `i128`     |   `u128`     |
|   **arch**   |   `isize`    |   `usize`    |

With the `arch` size depending on the OS; if the OS is 32-bits, it will be 32-bits, if the OS is 64-bits, it will be 64-bits.

###### Maths
If you don't want to learn this table and not wanting to go see the documentation, here's the mathematical relations:

Let $n$ be the number of bits used to store the number, the **unsigned** data type can store any number in this interval: $[0, 2^n - 1]$.  
Explanations: $2^n$ because the number is stored in binary and the $-1$ is because $2^k$ is written $1$ with $k$ 0s behind.

Let $n$ be the number of bits used to store the number, the **signed** data type can store any number in this interval: $[-2^{n-1}, 2^{n-1} - 1]$.  
Explanations: $n-1$ is because the first bit is used to store the sign.

You can also find the maximum and minimum of each type with the constants `std::[type]::MAX` or `std::[type]::MIN`.

###### Interger overflow
This error happens when the value is too big to be stored in one type.

ie:
```rust
let x : u8 = 256; // u8::MAX = 255
```

With this code, Rust will _panic_ (end with an error (see [chap 9.](#9-))) and you could fix it by a lot of way but we will see the `--release` way. This method is just to compile your code with the flag `--release`, this way your program won't panic and Rust will perform a "_two’s complement wrapping_", in short it consist of taking the modulo (remainder of an integer division) of the overflowing number.

<!-- don't work because of the types but the logic works for positive nums
You can also do this, just use this code:
```rust
fn two_complement_wrapping(x : isize) {	//let's say that [type] can be all int types
	return x % (usize::MIN *-2 -1);
}

fn main() {
	println!("{}", two_complement_wrapping(usize::MAX + 2));	//out-> 2
	println!("{}", two_complement_wrapping(usize::MAX + 80));	//out-> 80
	println!("{}", two_complement_wrapping(usize::MAX * 2));	//out-> 0
}
```-->


###### Misc
You can write number by separating some digit with an underscore `_` (ie: `3_09` and `4_294_967_295` are two valid integers).

You can also write numbers with prefixes that will dertermine their base (2, 8, 10 or 16).
|         type        |     prefix    |    example     |
|:--------------------|:-------------:|:--------------:|
|     **Binary**      |      `0b`     |     `0b1001`   |
|     **Decimal**     |  $\emptyset$  |   `112358_13`  |
|      **Octal**      |      `0o`     |     `0o137`    |
|   **Hexadecimal**   |      `0x`     |   `0xFOODBAD`  |
|      **Byte**       |      `b`      |      `b'B'`    |

The _byte_ notation returns the [ASCII number](https://www.ascii-code.com/) of the char:
```rust
fn main() {
	println!("{}", b'\n'); 	//out-> 10
	println!("{}", b'N'); 	//out-> 78	
	println!("{}", b'@'); 	//out-> 64
}
```

##### 3.2.1.2. Floating-point
If the ints types were $\Z$ and $\N$, the floating-point (float) types are $\R$.  
In Rust, there are only two types: `f32` and `f64` (equivalents of `float` and `double` in C++), by default a float has the `f64` type.

##### 3.2.1.3. Numeric operations
Rust handles the five basic operations: addition, subtraction, multiplication, division, and remainder (modulo): 
```rust
fn main() {
	// addition
	let sum = 5 + 10;				//->15

	// subtraction
	let difference = 95.5 - 4.3;	//->91.2

	// multiplication
	let product = 4 * 30;			//->120

	// division
	let quotient = 56.7 / 32.2;		//->1.7608695652173911
	let truncated = -5 / 3; 		//->-1 (we're doing whole division)

	// modulo
	let remainder = 43 % 5;			//->3
}
```


##### 3.2.1.4. Booleans
Those are 1-bit binary: either `true` (`1`) xor `false` (`0`). Their main usage is through conditions that we'll se in the _control flow_ section [3.5.](#35-control-flow) of this chapter.

##### 3.2.1.5 Characters
The type `char` is for storing characters like 'a' or '😻':
```rust
fn main() {
	let f : char = 'f';
	let cat = '😻';
	let N = 'ℤ';
	let null_char = '\0';
}
```

You can see that we are usign **single**-quotes instead of the double quotes we used to display text with `println!()`, that's because double-quotes means that we declare a `string` (collection of `char`s). Each char can store a [unicode character](https://en.wikipedia.org/wiki/List_of_Unicode_characters) from U+0000 to U+D7FF and from U+E000 to U+10FFFF.  
There is another way to write chars, you can write each char like this: `'\u{[unicode code]}'`.

namely:
```rust
fn main() {
	println!("{}", '\u{00D8}'); //out-> Ø
	println!("{}", '\u{0000}'); //out-> null char (you can do '\0' too)
	println!("{}", '\u{0010}'); //out-> line feed (you can do '\n' too)
	println!("{}", '\u{0D0B}'); //out-> ഋ
}
```

We'll see more use in c[hapter 8.](#8-Collections).

#### 3.2.2. Compound Types
Those are a mix of many values into one type. We'll see the two primitive compound types: arrays and tuples (we'll see objects in chapter 17).

##### 3.2.2.1. Tuples
Tuples are the main ways to group pieces of data together.

```rust
fn main() {
	let tuple : (i64, u8, char) = (-5, 250, 'c');
}
```

A tuple can hold several types and can vary in size.  
You can unpack a tuple like in Python and access it with a dot. Unlike Python a tuple can change over time (if mutable).

```rust
fn main() {
	let mut my_tuple = (1, 2, 3, 4, 5);

	let (one, two, three, four, five) = my_tuple; // one = 1, two = 2, ...

	let first = my_tuple.0;	//tuples are 0-indexed, the first element is at pos 0
	let fourth = my_tuple.3;

	my_tuple.4 = 3;

	println!("{} = {}, {}, {}, {} = {}, {}", one, first, two, three, four, fourth, five);	//out-> "1 = 1, 2, 3, 4 = 4, 5"
	println!("{}, {}, {}, {}, {}", my_tuple.0, my_tuple.1, my_tuple.2, my_tuple.3, my_tuple.4); //out->"1, 2, 3, 4, 3"
}
```

A tuple without any value is the _unit_ it represent a empty value as `undefined` is JS or `NULL` in C++. Expressions return it implicitly if they don't return nothing.  
By the way, there is no such thing as `null` in Rust so you have to find alternatives.

##### 3.2.2.2 Arrays
Arrays are similar to tuples but have differencies:
- All elements in an array are same type.
- An array is fixed in 	size.

The Book states: "_Arrays are useful when you want your data allocated on the stack rather than the heap (\[see] Chapter 4) or when you want to ensure you always have a fixed number of elements_", if you don't know is allocations _on the stack_ or _on the heap_, no worries, we will get to it later.  

In order to initialize an array you can do as with the tuples but with brackets `[]`, the second difference being in the type where you write this `[<type>; <size>]`.

Because the array is of fixed size, it can be allocated _on the stack_ (its elements are next to each other in memory) and you can access elements with brackets.

You can fill an array by initializing it like so: `let array = [<element>; <number of element>]`

example:
```rust
fn main() {
	let mut my_array : [char; 3] = ['a', 'b', 'c'];
	my_array[1] = 'e';

	let mut filled = [69; 100];	//the array contains 100 times the number 69
	filled[99] = 42;

	println!("my_array: {}, {}, {}", my_array[0], my_array[1], my_array[2]); //out-> "my_array: a, e, c"
	println!("filled: {}, {}, ..., {}, {}, {}", filled[0], filled[1], filled[97], filled[98], filled[99]); //out-> "filled: 69, 69, ..., 69, 69, 42"
}
```
#### 3.2.summary
In this section, we've seen many data type (and we didn't seen them all!) so I did this


### 3.3. Functions
**11/11**

#### 3.3.1. Generalities
Do you what a function is in maths? It is a list of operations to do in a certain order, for example $f: x \longmapsto x + 1$ is the successor function. In programming, it is **exactly** the same thing: a list of statements.

You already know some predefined functions like the `.cmp()` method (until we see the OOP in [chapter 5.](#5-introduction-to-objects-or-using-structs), remeber that's the same thing called with a dot before it) and how to define one (remeber how you define the `main()` function).

In Rust, a function name (variables names too) should be in _snake case_, it means that the name should be in lowercase and word separated by underscores `_`.

```rust
fn myFunction()  {} // this is camel case, you'll get a warning

fn myfunction()  {} // this is technically correct but is difficult to read

fn my_function() {} // this is the right way
```


#### 3.3.2. Parameters
Like in maths, you can pass _parameters_ or _arguments_ to functions. A parameter is a variable that is defined when calling the function, in maths those parameters are separated by commas, so are they in programming.
The only difference is that in Rust you have to define the types, so: `fn foo(x, y) {}` is wrong but `fn foo(x : char, y : u32) {}` is correct.

```rust
fn add(x : isize, y : isize) {
	println!("{x} + {y} = {}", x+y);
}

fn main() {
	add(1, 2); //-> "1 + 2 = 3"
	add(80, -30);//-> "80 + -30 = 50"
}
```

#### 3.3.3. Statement and expressions
In Rust, the concept of _statements_ and _expressions_ is a bit different compared to other languages:
- A **statement** is an instruction to perform an action and $\color{red}\text{does not return any value}$ (not even the unit `()`).
- An **expression** evaluates a resultant value.

Let's look at some examples:  
Creating a variable and defining a function are **statements** , so this code does not make any sense:
```rust
fn main() {
	let var1 = (let var2 = 5);

	let mut foo = (fn s() {});
}
```
This differ from languages like C or Ruby due to the syntax `x = y = 4` working because it return a value.

Expressions have to return a value and are pretty much the rest of Rust code. Let's take an example: in the statement `let x = 9;` the number `9` is an expression. Math operations, calling a function and scopes are all expressions.  
We can do things like those:
```rust
fn main() {
	let my_var = {
		let my_other_var = 69;
		my_other_var / 23	//there is no semicolon for expressions
	}; //note the semi-colon

	println!("my_var has a value of {}", my_var);	//out-> 3
}
```
The curly brackets `{}` indicate that we enter a _scope_, in this case the scope: 
```rust
{
	let my_other_var = 69;
	my_other_var / 23	//there is no semicolon for expressions
}
```
returns the value `69/3` or `3`.

#### 3.3.4. Functions with return value
In Rust, curly brackets means we are in a scope. In this case, defining a function is just naming a scope. And scopes returns the last expression it computed. There is one exeption, for functions only, we can use the keyword `return` to return the expression passed.

If your function return something else that the unit value `()`, you have to specify a return type by putting an arrow and the return type behind the parenthesis (`fn foo(param : type) -> return_type {}`).

Those 4 functions do exactly the same thing:
```rust
fn successor1(x : isize) -> isize {
	return x+1;
}
fn successor2(x : isize) -> isize {
x+1	//there is no semicolon
}
fn successor3(x : isize) -> isize {
	return {	//definig a scope which returns x + 1
		x + 1
	};
}
fn successor4(x : isize) -> isize {
	return x+1;

	//the function ends to the returns statement so all the code after it will be unreachable
	println!("A cosmic ray just hit my computer!");	//creates a warning for unreachable code
	x+1;	//creates a warning for unused statement
}

fn main() {
	println!("The first  successor to 142 is {}", successor1(142));		//out-> 143
	println!("The second successor to 142 is {}", successor2(142));		//out-> 143
	println!("The third  successor to 142 is {}", successor3(142));		//out-> 143
	println!("The fourth  successor to 142 is {}", successor4(142));	//out-> 143
}
```

### 3.4. Comments
A comment will not be compiled by the compiler, it is simple: `//` and the rest of the line is a comment:
```rust
fn main() {
	//this is a comment
	let x = 4; // the rest of the line is commented
	println!("{x}");

	//Multi
	//line
	//comment
}
```

There is another type of comment `/**/`, but we'll talk about it in [chap 14.](#14-).

### 3.5. Control flow
#### 3.5.1. Logical operators
To understand what we are doing next, we will see the list of logical operators in Rust. A logical operator is an operator which return a boolean value based on a test.

| operator |     name      |                              description                             |                  example                  |
|:--------:|:-------------:|:--------------------------------------------------------------------:|:-----------------------------------------:|
|    `!`   |     NOT       |                             invert the boolean                       |    `!true` = `false`, `!false` = `true`   |
|   `==`   |     XAND      |          Verify if the left side is equal to the right side          |                   `1 == 1`                |
|   `!=`   |     XOR       |        Verify if the left side is not equal to the right side        |                 `'u' != 'v'`              |
|   `>`    |   superior    |    Verify if the left side is strictly superior to the right side    |                   `1 == 1`                |
|   `>=`   |  superior eq  |    Verify if the left side is superior or equal to the right side    |                   `1 == 1`                |
|   `<`    |   inferior    |    Verify if the left side is strictly inferior to the right side    |                   `1 == 1`                |
|   `<=`   |  inferior eq  |    Verify if the left side is inferior or equal to the right side    |                   `1 == 1`                |
|   `&&`   |     AND       |                  Verify if both expressions are `true`               |               `5 < 8 && 8 > 5`            |
|  `\|\|`  |      OR       |              Verify at least one of its operand is `true`            |             `46 != 60 \|\| true`          |

#### 3.5.2. if and else
The `if` expression let you run a scope only if a condition is `true`.

An `if` statment is constructed with this syntax:
```rust
if [condition] {
	[code]
}
```

where condition returns a _boolean_.

For example:
```rust
fn main() {
	if 5 > 80 {	//condition always false, the scope will never be executed
		println!("You broke maths");
	}
	println!("This will be printed no matter what");
}
```

You can also follow an `if` by an `else`, the `else` has no condition but will be executed if the first condition is `false`:
```rust
fn main() {
	const FIVE : u8 = 5;
	if FIVE > 80 {	//condition always false, the scope will never be executed
		println!("You broke maths");
	} else {		//since the first condition is evaluated to false, this scope is executed
		println!("All's good");
	}
	println!("This will be printed no matter what");
}
```

You also write `else if`s before the `else` expression. An `if` expression can has a syntax:
```rust
if cond_1 {
	//if cond_1
} else if cond_2 {
	//if !cond_1 && cond_2
}else if cond_3 {
	//if !cond_1 && !cond_2 && cond_3
}
//...
else if cond_n {
	//if !cond_1 && !cond_2 && ... && !cond_(n-1) &&cond_n
}else {
	//if !(cond_1 || cond_2 || ... || cond_n)
}
```





<!--Wow! already the $1000^ {th}$ line! The 3rd chapter is 619 of those lines.-->
Since `if` is an expression you can write
```rust
fn main() {
	//...
	let has_a_life = if is_online {"no"} else {"yes"};
}
```
Where in other languages, you would write ternaries like `string hasALife = isOnline? "no" : "yes";`.

#### 3.5.3. Loops
In Rust there are 3 loops: the `loop` loop, the `while` loop and the `for` loop.

##### 3.5.3.1. The `loop` loop
This loop let us iterate until we explicitly tell it to stop, either with `Ctrl + C` if you are on the terminal, the task manager or the `break` statement.

##### 3.5.3.2. Returning from a loop
To do this we use the keyword `break`:
```rust
fn main() {
	let mut i = 0;
	let res = loop {
		if i >= 10 {
			break i;
		}

		i += 1; //i = i + 1;
	};

	println!("We have looped {res} times!"); //out-> 10
}
```
##### 3.5.3.3. Breaking from a particular loop
You can give names to loops witth labels:
```rust
fn main() {
	'my_loop: loop {		//the label must begin with a "'" else it will be mistaken for a variable
		// looping
	}
}
```
If you do this, you can use the keywords `break` and `continue` to apply to the loop with a label:
```rust
fn main() {
	let mut i = 0;
	const MAX_LOOPING : u8 = 10;
	'first: loop {
		let mut j = 0;
		'second: loop {
			if j >= MAX_LOOPING {
				i += 1;
				continue 'first;	//reset i to 0
			}
			if i >= MAX_LOOPING {
				break 'first;
			}
			j += 1;
			continue; 	//equivalent to "continue 'second;"
		}
	}
	println!("Successfully looped {} times", MAX_LOOPING * MAX_LOOPING);	//out-> "Successfully looped 100 times"
}
```

##### 3.5.3.4. The `while` loop
A `while` loop is a loop that ends when its condition is not fulfilled anymore (it runs _while_ the condition is fulfilled).

This behavior can be reproduced with a `loop` loop but is less readable:
```rust
loop {
	if !condition {
		break;
	}
	//code
}
```
VS
```rust
while condition {
	//code
}
```

Let's say we want to do a count down, we could do this:
```rust
fn main() {
	let mut it_left = 10;
	while it_left > 0 {
		println!("{it_left}!");	//definitly not factorials
		it_left -= 1;
	}
	println!("Happy new cake day!");
}
```

##### 3.5.3.5. The `for` loop
To be more precise, it would be a `forin` loop which is the equivalent of `foreach` loops in other languages (like Cs or PHP). It is the same as in Python.

Here's its syntax:
```rust
for var in arr {
	//code
}
```
VS 
```rust
{
	let (mut) arr : [type; ARRAY_LENGTH] = //an array
	let mut i = 0;
	while i < ARRAY_LENGTH {	//we assume that ARRAY_LENGTH has already been defined
		let var = arr[i];
		//code
		i += 1;
	}
}
```
VS
```rust
{
	let (mut) arr : [type; ARRAY_LENGTH] = //an array
	let mut i = 0;
	loop {
		if i >= ARRAY_LENGTH {	//we assume that ARRAY_LENGTH has already been defined
			break;
		}
		let var = arr[i];
		//code
		i += 1;
	}
}
```

We can refactor our countdown code with a for loop:
```rust
fn main() {
	for count in (1..=10).rev() {	//the method .rev() reverses the list
		println!("{count}!");
	}
	println!("Happy new Cake day!");
}
```
It is shorter but can be diffcult to read and understand at first glance (unlike the algorithm with `while`) so always remember that one day you'll go back to this part of the code and have forgotten what it does.

_____
**13/11**
Wow, that was a _long_ chapter, as I said when the chapter began pretty much all the concepts we saw (except the _expression_/_statement_ handling which make Rust stand apart) are not unique to Rust, so if you already know another language, it should had been a simple syntax cheatsheet but if you didn't you'll learn.

## 4. Ownership
The book says that _"ownership is Rust’s most unique feature"_. It would be used to write memory-safe code without a garbage collector (like almost all languages do) (all a garbage collector do is free not anymore referenced memory (ie: varible out of scope) (garbage)).

### 4.1. What is ownership?
_Ownership_ is a set of rules that Rust use to allocate and free memory
#### 4.1.1. Introduction to ownership
##### 4.1.1.1. The stack and the heap
The _stack_ and the _heap_ are obscure concept for [high-level](https://www.geeksforgeeks.org/what-is-high-level-language/) programmers due to the fact they just don't manage memory. But Rust is a [low-level language](https://www.geeksforgeeks.org/what-is-a-low-level-language/) (according to the Book) and let you manage memory with pointer, knowing how to manage memeory is thus a good thing.  
Rust handles differently data allocated on the **heap** and on the **stack**.

To understand the **stack** think of a literal stack, for example a pile of boxes. If we remove any other boxes that the box on top, it will collapse so we can't do this; same logic with adding a box, we add it on top of the stack.  
With a data stack, you can only _push_ (add elements) or _pop_ (remove elements) of it. The thing is, Rust can only stack element of the same size, so data that haven't a fixed sized at compile time or that can change size are allocated on the _heap_ instead. Hence the fact that arrays are stored on the stack and tuples aren't.

The **heap** is a less loved way of handle memory, you just request space and the compiler put where there is space available, just like you ask waiter a table with $n$ places and he gives you a table with _at least_ $n$ places free.  
To remember where's the data, we use a _pointer_ that is just a variable that contains the memory address of your data (but is on the stack).  
_Store something with the heap is called **allocating**._

_note: pushing onto the stack is faster than allocating because with the heap, the compiler has to search where to store a value. 
Accessing is also slower on the heap because modern processors are faster when they jump aroud less in memory_

##### 4.1.1.2. The rules
- Each value has **exactly** one owner.
- When the owner goes out of scope, the value will be dropped (deleted from memory).

##### 4.1.1.3. The scope of a variable
Do you remember the scopes from when we talked about them being expression? If no please go back to sub-section [3.1.3.](#313-shadowing). For those who actually remember: when a variable is created inside a scope, it cannot go out of this scope but can be called in inner scopes.

demonstration:
```rust
fn main() {
	{
		let my_var = 42;
		{
			println!("{my_var}");
		}
	}
	println!("{my_var}");	//cannot find my_var and panics
}
```

We just demonstrated rule number 2.

**16/11**
#### 4.1.2. The `String` type
To better illustrate ownership we need a type more complex than those in [section 3.1.](#31-variables) which can be stored on the stack (except tuples). In this sub-section we'll only discuss of the parts which relate to ownership, the rest will be seen section [8.2.](#82-).

We've already seen the string literals earlier (remeber it's the double-quoted strings `"hello!"`) but they can be not suitable for every projects because firstly they are immutable and secondly you cannot know all the text (ie: the name of a user).  

You can convert any string literal to a `String` object with `String::from`:
```rust
fn main() {
	let str = String::from("str litteral");
}
```
The double colon `::` is to indicate that the method `::from()` is in the string namespace/class (wait [section 5.3.](#53-methods)).

This kind of string can be mutated:
```rust
fn main() {
	let mut str = String::from("Hello");
	str.push_str(" world!");

	println!("{str}"); //out-> "Hello world!"
}
```

You may be asking yourself _"Why can't string literals change value but `String`s can ?"_, this is because of how the two types are allocated in memory.

#### 4.1.3. Memory and allocation
The string literals are fast, this is because the compiler knows them at compile time are they are directly in the final executable hence the fact those cannot change in the program.  
The `String`s aren't hard-coded and can change in the program, to do this they are allocated _on the heap_ hence the fact they can _grow_, _shrink_ or change in general.


I can see some people not understanding in the back and say this program compiles:
<span id="anchor431"></span>
```rust
fn main() {
	let mut my_str = "Hi!";
	println!("{my_str}");

	my_str = "Hello";
	println!("{my_str}");
}
```
And it does however the string literal `"Hi!"` does not change, only the value of `my_var` does and the literal `"Hi!"` is gone in the void of your RAM when we set `"Hello"` to `my_var`.

The role of the `String::from()` method is to allocate memory for the `String` object similary to all programing languages.  
Rust differentiate because where in C you'd have to call `free(var)` after allocate it with `malloc(sizeof(var))`, Rust automates it and free the memory when a variable is out-of-scope:

```rust
fn main() {
	{
		let var = String::from("str"); //allocates memory for var

		//do stuff
	}
	// the variable var is now out-of-scope so the space String::from allocated is freed
	// you can no longer call var
}
```
<!--That's literally a garbage collector-->
When var is out-of-scope Rust calls the function [`drop()`](https://doc.rust-lang.org/std/ops/trait.Drop.html#tymethod.drop) (which calls a destructor).

_Note from the Book: "In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns."_

#### 4.1.4. Data interacting with movement
Let's compare those two pieces of code:
<span id="anchor414"></span>
```rust
fn main() {
	let x = 5;
	let y = x;
}
```
```rust
fn main() {
	let s1 = String::from("I love strings!");
	let s2 = s1;
}
```

The first one is straightforward, we assign `x` to `5` (we push `5` onto the stack) then we asssign `y` to a copy of `x` (we once again push `5` onto the stack).  
As you can see on the image I stole from the Book the `s1` variable 
contains a _pointer_, a length and a capacity, the pointer _points_ on the string value. When we copy `s1`'s value we don't copy the array of chars, we copy thepointer, length, and capacity of `s1`.

<style>
	img {
		max-width: 40px; /*doesn't work, idk why*/
		max-heigth: 40px; /*doesn't work, idk why*/
	
		width: 50%;
		heigth: 50%;
	}
</style>
<img alt="Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1" title="Representation in memory of the variable s2 that has a copy of the pointer, length, and capacity of s1" src="https://doc.rust-lang.org/book/img/trpl04-02.svg"  />

A C++ equivalent would be this:
```C++
#include <string>

int main(void) {
	std::string  s1 = "hello";
	std::string& s2 = s1;
	
	return 0;
}
```

If you don't know C++ it means that if we change `s2`, `s1` will be also changed and the inverse is also true.

If Rust just copied the array, it would look like this:
<img alt="s1 and s2 points to 2 distinct arrays" title="Representation in memory if Rust copied heap-allocated values" src="https://doc.rust-lang.org/book/img/trpl04-03.svg"/>
And would be very memory-consuming, especially on heavy  and large pieces of data (for example a string with $2^5$ characters).

When you try to free a pointer already freed it produces an error. When assigning `s2` to `s1`, Rust invalidates `s1` so when freeing the variables it doesn't panics.  
That's what's happennig in the code below:
```rust
fn main() {
	let s1 = String::from("hello");
	let s2 = s1;	//invalidates s1

	println!("{s1}, world!");// trying to use an invalid var
}	
```

The book defines _moving a variable_: _"If you’ve heard the terms shallow copy and deep copy while working with other languages, the concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. In this example, we would say that s1 was moved into s2."_

#### 4.1.5. Cloning
##### 4.1.5.1. Copying data on the heap
If you _do_ to deep copy the heap data, you'll have to use the `.clone()` method to copy the data and have in memory the same pattern as in the previous image.

```rust
fn main() {
	let s1 = String::from("hello");
	let s2 = s1.clone();	//copy s1

	println!("{s1}, world!");// Success
}	
```

##### 4.1.5.2. Copying data on the stack
Now let's go back to the [first program of 4.1.4.](#anchor414):
```rust
fn main() {
	let x = 5;
	let y = x;
}
```

We can see that the data is copied without the use of `.clone()`, `x` wasn't moved to `y`. That's because the size of `x` and `y` are know at compile time so copying will take a short time and cannot be too big. There is no difference between shallow and deep copying, so calling `.clone()` is not necessary since it can't be different from the shallow copy.

Rust has a `Copy` trait (see you in chapter 10) that we can place on types allocated on the stack, if a variable with a type that implement `Copy`, it will not be moved and will be trivially copied.

What does implement the `Copy` trait? To get a complete answer go check [the documentation](https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors) it has a complete list for the standard library, but in general, "_any group of simple scalar values can implement `Copy`, and nothing that requires allocation or is some form of resource can implement `Copy`_". Tuples implement `Copy` if they contains only types that implement `Copy`.

#### 4.1.6 Ownership in functions from the perspective of...
##### 4.1.6.1. ...arguments
Passing a value to a function is similar to defining a variable, it can lead to unsettling results to the untrained programmer:
```rust
fn copying(x : usize) {	//copy argument to x
	println!("Here's my copied argument: {x}!");
}	//x is freed but nothing else happens

fn moving(s : String) {	//move the argument to `s`
	println!("Here's my moved argument: ' {s}'!");
}// free `s`, so the argument's address

fn main() {
	let my_string = String::from("XSS attack!'!   It may seem I can write where it is not 'intended");
	moving(my_string);  //out-> "Here's my moved argument: 'XSS attack!'!   It may seem I can write where it is not 'intended'!"
	//can't use `my_string` anymore

	let number = 0;
	copying(number); //out-> "Here's my copied argument: 0!"
	println!("The variable 'number' survived the copy, here's its value: {x}");	//out-> "The variable 'number' survived the copy, here's its value: 0"
}
```

##### 4.1.6.2. ...return values
Returns values can also be moved:
```rust
fn give_ownership() -> String {
	String::from("It's yours now")
}

fn takes_and_give_back(to_move : String) -> String {
	to_move
}

fn main() {
	let new_owner = give_ownership();		//the value "It's yours now" is moved to `new_owner`

	let present = String::from("Happy Christmas!");
	let child = takes_and_give_back(present);	//`child` recieves the ownership of "Happy Christmas!"

	println!("The former owner said: '{new_owner}'");	//out-> "The former owner said: 'It's yours now'"
	println!("Santa said           : '{child}'"); 		//out-> "Santa said           : 'Happy Christmas!'"
}
```

What if we wanted to return something else than parameters just to get back the ownership?  
One solution is to return a tuple:
```rust
fn get_length_and_ownership(s : String) -> (usize, String) {
	//tuple on mutiples lines
	(
		s.len(),	//gives the length of the string
		s
	)
}

fn main() {
	let former_owner = String::from("House");

	let (length, new_owner) = get_length_and_ownership(former_owner);	//unpacking
	println!("The string '{new_owner}' has a length of {length}.");	//out-> "The string 'House' has a length of 5."
}
```

That's tedious to do all this work to get the ownership back, fortunaly we make this more simpler with **references**.

### 4.2. Borrowing and references
#### 4.2.1. Is that a Rust reference ?
What's a reference you say? The Book define them like this: "_A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable._" but I personnaly find this definition more confusing than helpful, so here's mine: A _reference_ is a variable stored in the same address that another one. It means that if `var` is a variable and `ref` is a reference to `var`, if we change the value of `ref`, `var` will be affected and the reverse is also true.

In Rust, references are in the syntax `&var` with `var` a defined variable, we already saw them in our first project (chapter 2).  
We can rewrite our program:
```rust
fn get_length(s : &String) -> usize {
	s.len()
}	//s goes out of scope but since it didn't have ownership in the first place, nor is it dropped	

fn main() {
	let owner = String::from("House");
	let length = get_length(&owner);

	println!("The string '{owner}' has a length of {length}.");	//out-> "The string 'House' has a length of 5."
}
```

_note: If you can **reference** with an ampersand `&`, you can **dereference** with an asterix `*` the same way you'd reference. We'll dereference in [chapter 8.](#8-) and we will see the complete picture with [chapter 15.](#15-)._

**17/11**
The action of creating a reference is called _borrowing_ because the reference doesn't own the value (like you'd borrow a book).

#### 4.2.2. Mutable references
As the variables are, references are immutable by default (that's why we put a `&mut` in the `.read_line()` method in [chapter 2.](#anchor21)). 

Here's an example of how to change a variable with references:
```rust
fn put_a_new_line(txt : &mut String) {
	//modifying `txt`
	txt.push_str("\r\n");	//We're placing a CRLF for windows users
}

fn main() {
	let mut txt1 = String::from("I am text 1!");
	let txt2 = "I am text 2!";
	put_a_new_line(&mut txt1);	//`txt1` is modified

	println!("{txt1} and {txt2}");
	//out-> "I am text 1!
	// and I am text 2!"
}
```

Remember that Rust does not allow more that **1 mutable reference**, you can put as many immutable references as you want but there can only be one mutable reference at each given time:
```rust
fn main() {
	//setting as mutable else cannot asign mutable refs
	let mut some_var = String::from("This could be heaven, this could be hell");

	let hotel = &some_var;
	let such_a_lovely_place = &some_var;
	let such_a_lovely_face = &some_var;
	println!("{hotel},\n {such_a_lovely_place},\n {such_a_lovely_face}"); //prints 3 times some_var separated by new line (not on windows)

	let never_leaving = &mut some_var;
	let check_at_any_time = &mut some_var;	//compile error
	println!("{never_leaving}\n {check_at_any_time}");
}
```

This design choice is to prevent _data races_, the Book defines a data race as:
- "_Two or more pointers access the same data at the same time._"
- "_At least one of the pointers is being used to write to the data._"
- "_There’s no mechanism being used to synchronize access to the data._"

Data races can lead to weird behavior and are complex to diagnose so Rust just refuses to compile this kind of code.

Rust does not allow declaring a mutable reference we when immutable ones are used in scope. 

So this code does not compile:
```rust
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &mut s; 	//error here
println!("{}, {}, and {}", r1, r2, r3);	//references used after the muatble ref definition
```

but this one does:
```rust
fn main() {

	//setting as mutable else cannot asign mutable refs
	let mut some_var = String::from("This could be heaven, this could be hell");

	let hotel = &some_var;
	let such_a_lovely_place = &some_var;
	let such_a_lovely_face = &some_var;
	println!("{hotel},\n{such_a_lovely_place},\n{such_a_lovely_face}"); /
	// those references are not used anymore so Rust tolerates this code

	let never_leaving = &mut some_var;
	println!("{never_leaving}");
}
```

#### 4.2.3. Dangling references
A dangling reference is a reference that points to a freed adress in memory, 
for example:
```rust
fn main() {
	let dangling_ref : &String = {		//produces an error
		let var = String::from("hallo");

		&var
	};	//`var` is deleted so `&var` points to an invalid adress
}
```
The error message you'll get: "_\`var\` does not live long enough_" refers to the concept of lifetimes that we will se in [chapter 10.](#10).

To conclude this section, references are pointers that points to valid address, there cannot be any mutable references if another reference is is defined.

### 4.3. The Slice type
#### 4.3.1. A long introduction
A Slice is a continious reference to a part of a collection (we currently know only `String` as a collection but we'll discuss other like vectors in chapter 8 but you can always check [the doc](https://doc.rust-lang.org/std/collections/index.html)).

The Book gives this exercise: "_Write a function that takes a string of words separated by spaces and returns the first word it finds in that string. If the function doesn’t find a space in the string, the whole string must be one word, so the entire string should be returned._" (for example with the function `foo` the answer: `foo("I am words") == "I"`, `foo("word") == "word"`, `foo("single(s) words") == "single(s)"`).

I created a function `unit_test()` to help you test your script:
```rust
fn unit_test(s : &str) {	
	println!("'{}'", first_word_ref(&String::from(s)));
}
```
usage: `unit_test("I am the son of GOD! I was SENT DOWN by my father...")`


If you read what's behind this line, I assume you tried the exercise, if not: shame on you!
_____

Here's my try:
```rust
fn first_word(txt : String) -> String {	//takes ownership (I forgot about this)
	let mut buffer : String = String::from("");	//declaring a buffer to store the word
	for letter in txt.chars() {	//iterates `txt` letter by letter (found `.chars()` thanks to the complier's hints)
		if letter == ' ' {		//verify if the current character is a space
			break;	//at first I wanted to return with the loop but for loops can't return anything
		}
		buffer.push_str(&String::from(letter));	//adding `letter` to buffer
	}

	buffer	//returning result
}
```

Enough of bad scripts, the Book's proposing this code:
```rust
fn first_word(s: &String) -> usize {	//taking a reference to avoid ownership, returning the position of the end of the word
	let bytes = s.as_bytes();			//explode the string in a sequence of interger (byte notation)

	for (i, &item) in bytes.iter().enumerate() {	//see 1.
		if item == b' ' {			//verify is current char is a space
			return i;				//returns i
		}
	}

	s.len()		//in case the program didn't find any space, return the length of the string
}
```
> **1.** The function `.iter()` is using _iterators_ that we will see in [chapter 13.](#13-), for now it just returns each element in collection and `.enumerate()` put them in an exploitable tuple.  
In the first part of the first line of the `for` loop, we use patterns (That we'll see in [chap 6.](#6-enumarations-and-pattern-matching)), the first variable is `i` and represents the number of times we iterated -1 (like in `for(int i = 0; i < list.size(); i++) `) and the variable `item` the current letter of the text passed (`bytes[i]`), you can see that this is a reference.

This function is well-made but a position is only exploitable in the context of the string you passed, if it change the return value is useless.  
Take this program:
```rust
//we consider `first_word()` already defined

fn main() {
	let mut my_str = String::from("This text is ugly!");	//-> 4
	let my_first = first_word(&my_str);
	my_str.clear(); //empties the string like ```my_str = String::from("");``` would
	//`my_first` is now useless

	let first = first_word(&"My text is beautiful!");	//-> 2
	//`first` is now useless
}
```

Now imagine a function `second_word()` (you can write but there will not be any correctiuon), its definition would look like:
```rust
fn second_word(txt : String) -> (usize, usize) { //...
```
It would return two ints but not linked to data at all, we'd have to update them manually if `txt` would've changed.

Fortunaly, Rust has a built-in that we can use: Slices.

#### 4.3.2. Introduction to `String` Slices
A _string slice_ is a reference to a part of a `String`, it looks like this:
```rust
let alphabet = String::from("abcdefghijklmnopqrstuvwxyz");

let fast   = &alphabet[11..16]; //-> "lmnop"
let melody = &alphabet[16..22]; //-> "qrstuv	"
```

The sequences part of a string, they only need 2 variables, their start position (`11 + alphabet.begin()` for `lmnop`) and their length.

<img src="https://doc.rust-lang.org/book/img/trpl04-06.svg" alt="a representation of a string `s` and a sequence `world`" />

In this case the `s` is defined with `let s = "hello world"` and `let world = &s[6..11]`.

_note: if your slice begin at `0`, the first `0` is falcutative: `&s[..5]` is valid.  
By the same logic, if you want to take until the last character of the string, the second number is not required: `&s[6..]` and `&s[..]` are valid._

Now you know this go rewrite your `first_word()` function (you'll have to return a reference to a `str` due to string Slices being represented by the type `&str`).  
Hence the definition of your function should be:
`fn first_word(txt : &String) -> &str {`
____________________________________

My try:
```rust
fn first_word(txt : &String) -> &str {
	for (i, &item) in txt.as_bytes().iter().enumerate() {	//see 1. of previous exercise
		if item == b' ' {			//verify is current char is a space
			return &txt[..i];		//returns correct sequence (without the ' ' char)
		}
	}
	&txt[..]	//returns all of the string
}
```

**23/11**

This is roughly the same code as the Book gives us.


Now remember the program we made in [4.3.1.](#anchor431), now it does not compile:
```rust
//we consider `first_word()` already defined

fn main() {
	let mut my_str = String::from("This text is ugly!");	//-> 4
	let my_first = first_word(&my_str);

	my_str.clear(); // compile error
	println!("The first word is '{my_first}'.");
}
```

The error message says: ``cannot borrow `my_str` as mutable because it is also borrowed as immutable``.  
If you remember last section; when a mutable reference is created, there cannot have any other reference to prevent data races. And the `.clear()` method uses a mutable reference to clear the `String`, `my_first` being a Slice (so being a reference) the compiler refuses to compile this code.

A more experienced programmer would not have written the function `first_word()` as we did, he would have made the parameter a Slice too:
```rust
fn first_word(txt : &str) -> &str {
```

It lets us pass only a part of the string, for example:
```rust
// definiion of `first_word()`

fn main() {
	let original_string = String::from("All I want for christmas is Rust.");

	let word = first_word(&original_string[..]);
	println!("The first word of \"{}\" is \"{word}\".", &original_string[..]);	//out-> "The first word of "All I want for christmas is Rust." is "All""

	let word = first_word(&original_string[6..]);
	println!("The first word of \"{}\" is \"{word}\".", &original_string[6..]); //out-> "The first word of "want for christmas is Rust." is "want"."

	let word = first_word(&original_string[11..24]);
	println!("The first word of \"{}\" is \"{word}\".", &original_string[6..]); //out-> "The first word of "for christmas" is "for"."
}
```

#### 4.3.3. Other slices
There's other `Slices` that `&str`s, it works with arrays (and many other types):
```rust
fn main() {
	let arr = [1, 2, 3, 4, 5];

	let arr_slice = &arr[2..];

	// This code prints `arr_slice`
	print!("[");				// The macro `print!()` is the same as `println!()` but doesn't insert a new line '\n' at the end of the string
	for number in arr_slice {	//out-> "[3, 4, 5]"
		print!("{number}, ");
	}
	println!("\u{0008}\u{0008}]");	// The character '\u{0008}' is the equivalent of the '\b' character. 
									// It erase the last character printed (in this case they trim the ", " str).
}
```


### 4.summary
The chapter in short:  
The ownership concept is a concept that only Rust have, it lets you have control over the way your data is stored and do the cleanup for you. It has simple rules but it is crucial to well understand them and we will talk again of ownership all througout this file.

In the next chapter, you will be introduced to _Object Oriented Programming (OOP)_.

## 5. Introduction to objects or Using `struct`
Structures or structs are _like in Cs_ a way to agglomerate different pieces of data. With them you can create new custom types. In this chapter we'll compare structs with tuples to not get lost in the OOP's abyss.  
This chapter will show how to define structures-related functions, also called methods.

Note that even if structs are a way to construct new types, they are not the only one and enums (in [chapter 6.](#6-enumarations-and-pattern-matching)) are another way.

### 5.1. Defining structures
#### 5.1.1. The general syntax and some examples
Structures are like tuples as we talk in [sub-section 3.2.2.1.](#3221-tuples), they are a way to store data but _unlike_ tuples, each element of a struct is named which means that firstly it is more easy to know what is the purpose of each element and secondly there is a predefined number of elements.

To define a structure, we follow this syntax:
```rust
struct StructName {
	field1: type1,
	field	2: type2,
	//...
}
```

for example a chair would be represented by this:
```rust
struct Chair {
	name : String,
	number_of_legs : u8,
	dimensions : (u8, u8, u8),	//x: horizontal, y: depth, z: heigth in cm
	is_soft : bool,
	
	//etc...
}

//outside `main()`
```


After defining the structure, we can _instantiate_ an _object_. A struct is like the assembly instructions of the chair and the object or the instance is the assembled chair.  
To instantiante, we do like this:
```rust
//inside `main()`
let strandmon = Chair {			//we built a chair which has those properties
	name : String::from("STRANDMON"),
	number_of_legs : 4,
	dimensions : (82, 96, 101),
	is_soft : true,
};
```

To get an instance's field, we use the _dot notation_ like for tuples: `strandmon.name` is valid.

We can declare the instance `strandmon` as mutable but we cannot choose for certain fields to be mutable and other no.

#### 5.1.2. Building instances from a function
We can define a function returning a `Chair` object:
```rust
fn build_soft_chair(name : &String, dimensions : (u8, u8, u8)) -> Chair {
	Chair {
		name: name,
		number_of_legs: 4,
		dimensions: dimensions,
		is_soft: true,
	}
}
```
It only make sense to name the parameters the same name the struct does. But we developpers are lazy people and can't write two times the same name so Rust's devs added a shorthand: if the two names are the same, we just can write it once:
```rust
fn build_soft_chair(name : &String, dimensions : (u8, u8, u8)) -> Chair {
	Chair {
		name,
		number_of_legs: 4,
		dimensions,
		is_soft: true,
	}
}
```
If you already programmed orientated object you must be waiting for how we define constructors but Rust has no such thing, we can only build from regular function or methods.

#### 5.1.3. Partially copying another object
What if a chair has the same properties as `strandmon` but just another name?  
We would have to write this thing ?
```rust 
fn main() {
	let strandmon = build_soft_chair(&String::from("STRANDMON"), (82, 96, 101));
	let original_chair = Chair {			
		name : String::from("STALMON"),
		number_of_legs : strandmon.number_of_legs,
		dimensions : strandmon.dimensions,
		is_soft : strandmon.is_soft,
	}
}
```
or we'd have to write a function?

No we would not have to. I as I said, devs are lazy so the Rust team created a faster way:
```rust
fn main() {
	let strandmon = build_soft_chair(&String::from("STRANDMON"), (82, 96, 101));
	let original_chair = Chair {			
		name : String::from("STALMON"),
		..strandmon		//copy the all field left
	};
}
```
See that we used the `=`, it means that if at least one element of chair had implemented the `Copy` trait the whole instance of `strandmon` would've been moved in `original_chair` (as we said in [sub-section 4.1.4](#414-data-interacting-with-movement) and the [one after it](#4151-copying-data-on-the-heap)).

#### 5.1.4. Tuples structs
You can define a structure without any name, it is called a _tuple struct_.

Here's an example of tuples structs:
```rust
struct Point3D (f32, f32, f32);
struct Vector3D(f32, f32, f32);

fn main() {
	let origin   = Point3D (0.0, 0.0, 0.0);
	let zero_vec = Vector3D(0.0, 0.0, 0.0);
}
```
In this example `origin != zero_vec` because their types are different.

#### 5.1.5 Unit-like structs
You will sometimes want to define structures with no fields, it is possible with unit-like structs (remember the unit `()` value we discussed at the end of [3.2.2.1](#3221-tuples)). Those cases are when you need to store some traits on a type but requires no value (traits are in [chapter 10.](#10-)).

Here is how to define unit-like structs:
```rust
struct UnitLike;

fn main() {
	let some_var = unit_like;
}
```
It will be useful when will need, for example, a type which is equal to any other type <!--for some reasons-->.

#### 5.1.6. Ownership and structures
We can let some elements of a structure be references, it only have to follow the concept of lifetimes (that we'll see in [chap 10.](#10-)).

For example, let's try to compile this code:
```rust
struct DoNotOwn {
	heap1: &String,
	heap2: &String
}

fn main() {
	let if_buying = DoNotOwn {
		heap1: &String::from("Gaming"),
		heap2: &String::from("isn't stealing"),
	};
}
```

We will get two errors: `missing lifetime specifier`, we will really fix those error in [chapter 10](#10-); in the meantime, we'll just define string references as `&str` instead of `&String`.

### 5.2. An example using structures
**24/11/24**
To well understand when using structs, let's make a little program that calculates the area of a rectangle. That means a it's a new ~(mini)~ project!

To begin, let's make a program that takes two `const` values that represent the width and the heigth of the rectangle.

<sub>(this line indicates the code after it is the correction)</sub>
______

#### 5.2.1. Our first try: simple variables
```rust
fn area(width : f32, heigth : f32) -> f32 {
	width * heigth
}

fn main() {
	const RECT_WIDTH : f32 = 9.3;   //any arbitrary value does the trick
	const RECT_HEIGHT: f32 = 5.9;

	println!("The area of a rectangle with a width of {RECT_WIDTH}u and heigth {RECT_HEIGHT}u has an area of {}u².", area(RECT_WIDTH, RECT_HEIGHT));
}
```

This code does the job but _it’s not clear anywhere in our program that the parameters are related_ since they are just two floats.  
Fortunaly, we've already encountered this case: let's refactor with tuples.
______

#### 5.2.2. Our second try: tuples
```rust
fn area(rect : (f32, f32)) -> f32 {
	rect.0 * rect.1
}

fn main() {
	const RECT : (f32, f32) = (9.1, 1.112);

	println!("The area of a rectangle with a width of {}u and heigth {}u has an area of {}u².", RECT.0, RECT.1, area(RECT));
}	
```

Tuples are a good way to add structure <!--intentional--> to our code but we don't which value of `RECT` is its width and which is its heigth. In this case, the order doesn't really matter since we're just multiplying two `f32` but if we have to draw rectangles to the screen, it will matter.  
This implies the use of, you guessed it, structures. So let's refactor with structs:
_________
#### 5.2.3. Our third try: implementing structures

```rust
struct Rect {
	width  : f32,
	height : f32,
}

fn area(rect : &Rect) -> f32 {
	rect.width * rect.height
}

fn main() {
	const RECT : Rect = Rect {
		width : 9.1,
		height: 1.112,
	};

	println!("The area of a rectangle with a width of {}u and heigth {}u has an area of {}u².", RECT.width, RECT.height, area(&RECT));
}
```

This will be the general look of the code for the rest of the section.  
This code is straigthforward and, I think, needs explaination only on the use of references. Here our instance `RECT` is a constant and cannot be moved but if we changed `RECT` to be a simple variable `RECT` would be moved in the `rect` parameter so we use references to borrow `RECT` and keep the ownership.

#### 5.2.4. Adding functionalities with traits
Our program is done but it lacks features like printing with `print!()` and `println!()` macros.

Currently we cannot print our structure like this:
```rust
println!("{}", RECT);
```
We have to do this:
```rust
println!("(w: {}, h: {})", RECT.width, RECT.height);
```

When we try the first `println!()`, we get this error: ``error[E0277]: `Rectangle` doesn't implement `std::fmt::Display` ``. That's because `println!()` do formatting, and the default one for curly brackets is `fmt::Display` that is formatting for users to see.  
The error doesn't stop here, Rust gives us hints and one of them says thos: ``note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead``.  
Let's try those two: 
```rust
println!("{RECT:?}" );
println!("{RECT:#?}");
```

We now get this error: ``error[E0277]: `Rect` doesn't implement `Debug` `` and once again gives us an helpful note: ``note: add `#[derive(Debug)]` to `Rect` or manually `impl Debug for Rect` ``.

So we add this line `#[derive(Debug)]` before the `Rect`'s definition (`struct Rect {`) and run once again `$ cargo run`.  
For `println!("{RECT:?}");` we get this output: `Rect { width: 9.1, height: 1.112 }` and for this one `println!("{RECT:#?}");`: 
```
Rect {
    width: 9.1,
    height: 1.112,
}
```

We could've obtain the same result with the [`dbg!()`](https://doc.rust-lang.org/std/macro.dbg.html) macro which takes the ownership and returns it back unlike the `print!()` macro which only take a reference.  
Here's how to use it:
```rust
let data = impl_dbg {};		// The `impl_dbg` struct implements the `Debug` trait
let data = dbg!(data);		// We take the ownership back after `dbg!()` is done with it
```

_note: The `dbg!()` macro prints the instance in the `stderr` whereas `print!()` prints in the `stdout` stream. For now that's we don't care since the terminal prints the two (IDEs migth give a different style to the `stderr` stream) but we will in [section 12.6](#126-)._

In addition of the `Debug` trait Rust a variety of traits, they're all listed in [Appendix C of the Book](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html). There will be an explaination to how to create and use traits in [chapter 10.](#10-).

The `area()` function is very specific: it works only on instances of `Rect`. To be sure to only use it on `Rect` instances we'll implement it as a _method_ of the the `Rect` struct.

### 5.3. Methods
I will finally give you an accurate definition of _method_:  
A **method** is a function specific to an instance, it can modify it, read its fields and return a result according to those values. A method can be defined in a context of a struct, an enum or a trait object (that we will cover in chapter [6.](#6-enumarations-and-pattern-matching) and [17.](#17-) respectively).

#### 5.3.1. Defining and using methods
This is our code with the `area()` implementing as a method:
```rust
// Rect definition
#[derive(Debug)]
struct Rect {
	width  : f32,
	height : f32,
}

// Definitions of methods
impl Rect {
	fn area(&self) -> f32 {	// The `self` argument represents the instance calling the method
		self.width * self.height
	}
}

// Main script
fn main() {
	let rectangle = Rect {
		width : 9.1,
		height: 1.112,
	};
	let area = rectangle.area();

	println!("A rectangle of width {}u and heigth {}u has an area of {}u².", rectangle.width, rectangle.height, area);
}
```

To create a method we first create the `impl` block (for implementation block). All defifnition inside this block will be associated to the `Rect` structure.  
We then define the `.area()` method with the `self` argument, like in Python the `self` parameter represents the object calling this method, it is passed by reference to avoid loss of ownership. This argument is always the first one.  
In order to call the `.area()` method, we use the dot notation. Note that we do not need to manually pass `self` to the method.

In the definition of `.area()` we do not specify a type, that's because `&self` is a shorthand to `self: &Self`, in a `impl` block the `Self` type is an alias to the structure related to the block (here the `Rect` structure).

In a method definition, you can take the ownership of `self`, it is typically used when using `mut self` to transform the instance and stopping the method's user to use the old version of the instance.

It is legal to name a method and a field the same name, in most cases you will want to do this as when the method is a _getter_ to the field, that's to say the function only returns the value of the field and do nothing else. It will make more sense to define getter when we will get access to private fields in [chapter 7.](#7) ([7.2](#72-) by looking the titles).

#### 5.3.2. Is there a `->` operator ?
In Cs you have the `->` operator and the `.` operator which do essensially the same thing but `ptr->value` is a shorthand to `(*ptr).value` where `ptr` is a pointer to an object or a pointer to a method (the `*` operator has not the same role as in Rust).

In Rust, there's no such thing. Instead there is a feature called: _automatic referencing and dereferencing_ which handles the hard work for us.  
Those two lines are the same:


<!--The 2000^th^ line! at this rate this document will have above 10,000 at the end-->
```rust
p1.method(&p2);
(&p1).method(&p2);
```
where in C++ you would have to:
```C++
ptr1.method(ptr2);	//crashes because C++ pointers can't have attribute/method
ptr1->method(ptr2);
```

#### 5.3.3. Adding more parameters to methods
Try making a method `Rect.can_hold()` which takes another `Rect` instance in argument and determines if the caller can hold (is bigger) than the argument.

You can use this `main()` from the Book to test your method:
```rust
fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect {
        width: 10,
        height: 40,
    };
    let rect3 = Rect {
        width: 60,
        height: 45,
    };

    println!(
		"Can rect1 hold rect2? {}", 
		rect1.can_hold(&rect2)		//-> true
	);
    println!(
		"Can rect1 hold rect3? {}",
		rect1.can_hold(&rect3)		//-> false
	);
}
```
______

The solution is quite simple:
```rust
fn can_hold(&self, rect : &Rect) -> bool {
	self.width > rect.width && self.height > rect.height
}
```

#### 5.3.4. Associated function
Technically all function defined in an `impl` block are associated to a struct but we call associated functions those function but which don't have the `self` argument because they are not methods.  
Generally those are `constructors` (which return an instance) and are, by convention, called `new()`. You can access them by typing the class name a double colon `::` and the method name.  
Those are equivalent to static methods in other languages.

Now an example:
```rust
//[def of `Rect`]

impl Rect {
	//[def `can_hold()` and `area()`]

	fn new(width : f32, height : f32) -> Self {	//shorthand
		Self {
			width,
			height,
		}
	}
	fn square(length : f32) -> Self {
		Self::new(length, length)	//calls the ::new() constructor
	}
}

fn main() {
    let rect1 = Rect::new(30.0, 50.0);	//calling Rect::new()
    let rect2 = Rect::new(10.0, 40.0);
    let rect3 = Rect::square(60.0);		//calling Rect::square()

    println!(
		"Can rect1 hold rect2? {}", 
		rect1.can_hold(&rect2)		//-> true
	);
    println!(
		"Can rect1 hold rect3? {}",
		rect1.can_hold(&rect3)		//-> false
	);
}
```

#### 5.3.5. Several `impl` blocks
I don't know what to say, the title is pretty self-explanatory.  
Just know that each class can have multiple `impl` blocks.

### 5.summary
The chapter in a nutshell is: you can define new types with the keyword `struct` which are called structures. Each structure can have attributes or not (for unit-like structures) and have associated function. In the case of a function, it can either be associated to instances with their first parameter being `self` and is called _method_, or it can be associated just to the struct and have no particular name.

Struct aren't the only way to create new types: enums can also do this.

## 6. Enumarations and pattern matching
So this chapter treats of *enumaration*s (abbreviated to *enum*s), they let you defining a type by just a list of values, booleans could be seen as enums.  
We already used enums in [chapter 2.](#23-see-if-the-user-was-right) in the aim of checking the user's response.

### 6.1. Defining an enum
#### 6.1.1. Understanding the concept
Let's say we're working on a project and we have to differentiate IPs addresses (if you find this useless remember the [final project and chapter](#20-) will be building a web server). 

Currently, there is two standards in IP addresses: IPv4 and IPv6, it means that any IP addresses is either v4 or v6 but not two at the same time (not even quantum computer have this issue).  
In this case using booleans seems to be a good choices since they are:
+ Stored on the stack
+ 1-bit long (memory efficient)
+ Have only two values

But there's an issue, a big issue, how to choose which represents which without making any arbitrary choices (`true` is v4 or v6?)?  
An another way would to store the version as an `u8` but doing like this let you define an IP address v255.  
And there's the enum method, with it only you can control the possible values and their number if an IPv780 appear some day you just have to add one line to your enum definition.

The definition of an enum is similar to the one of a struct:
```rust
enum Name {
	value1,
	value2, 
	value3,
	//...
}
```

We know how to define our IP address enum:
```rust
enum IPAddrKind {
	v4,
	v6
}
```
We just created a new type, the `IPAddrKind`.

To access those values, we use the double colon `::` notation (like in C#):
```rust
let ip1 = IPAddrKind::v4;
let ip2 = IPAddrKind::v6;
```

We can now define a function that routes an incoming IP address:  
`fn routeKind(kind : IPAddrKind) { //...`

We can even combine enums with structs:
```rust 
struct IPAddr {
	address : String,
	kind : IPAddrKind,
}
```

But why defining a struct when you can directly put a value inside the enumaration?
```rust
enum IPAddr {
	v4(String),
	v6(String),
}

fn main() {
	let localhost = IPAddr::v4(String::from("127.0.0.1"));
	let loopback  = IPAddr::v6(String::from("::1"));
}
```

Because it's an IP address we can just store it parsed:
```rust
enum IPAddr {
	v4(u8,  u8,  u8,  u8),
	v6(u16, u16, u16, u16, u16, u16, u16, u16),
}
```


#### 6.1.2. The standard library's solution
Wanting to store and parse IP is more common than you think, so common that the standard library has an [enum](https://doc.rust-lang.org/std/net/enum.IpAddr.html) already defined.

As you can see in <a href="https://doc.rust-lang.org/src/core/net/ip_addr.rs.html#31">the source code</a> (ignore the `#[stable...` things, that's documentation things) we wrote roughly the same code. The main difference is that they went a level of abstraction above us by defining a general `IpAddr` then defining a struct for each version.

If we collect all the definitions, `IpAddr` is this:
```rust
struct Ipv4Addr {
    octets: [u8; 4],	//4 times a number in [0, 255]
}

struct Ipv6Addr {
    octets: [u8; 16],	//16 times a number in [0, 255]
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

Since Rust is open-source don't hesitate to go check the source code, it can help understand what does exactly a function or what are the member of a struct. Sometimes you can have 1000 lines of code and sometimes you can have have litterally 11 lines.

Even if the standard library define those structures you can rewrite them without conflict because you didn't imported them with `use` (we'll see this [next chapter](#7-packages-crates-and-modules-let-you-organise-your-code)).

#### 6.1.3. Another example

Let's dissect this code:
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This enumeration has four variants:
+ **`Quit`**: has no associed value
+ **`Move`**: has named fields (like a struct)
+ **`Write`**: includes a `String`
+ **`ChangeColor`**: is a tuple containing three intergers

You can also define methods with an `impl` block:
```rust
impl Message {
	fn get(&self) {
		self
	}
}

fn main() {
	let m1 = Message::Write(String::from("Hello!"));
	let m2 = m1.get();
}
```

Now that you are comfortable with enums, let's look at a final example.

#### 6.1.4. The `Option` enum
This enum encode the more than common situation where a function returns something or nothing.  
Let's take an example: a function that search the letter `'a'` in an array. If the letter is found we returns its index but if not what do we do? In certain language you could return an arbitrary value like `false`, `-1` or `NULL`. In Rust, the function will return an `Option` enum which will have variant `Some` if found else the variant `None`.

There is no `NULL` value in Rust, the Book quotes Tony Hoare, who is qualified by it as _the inventor of null_.  
He said:
> "I was designing the first comprehensive type system for references in an object-oriented language. My goal was to ensure that all use of references should be absolutely safe [...]. But I couldn’t resist the temptation to put in a null reference, simply because it was so easy to implement. This has [...] caused a billion dollars of pain and damage in the last forty years."

_"In languages with null, variables can always be in one of two states: null or not-null"_ which can lead to various errors.

However the concept of null value can still be useful, it represents a piece of data missing or absent for a reason or another. So do use `NULL` but be aware of what you are doing.

As I said in my dirst paragragh, an `Option` can have only two values: `None` and `Some`.  
The definition of the type is thie:
```rust
enum Option<T> {
    None,
    Some(T),
}
```

What is the `<T>`? It is _a generic type parameter_, in C++ we'd say a template. `T` represents an undefined type so you can pass any type as T.  
For example, we can replace `T` by `bool`:
```rust
Option<bool> {
    None,
    Some(bool),
}
```

You can go [chapter 10.](#10-) to have a better definition.

If you want to have a variable of type `Option`, you can do:
```rust
fn main() {
	let some_bool : Option<bool> = Option::Some(true);	//staticly typing
	let some_int = Option::Some(89);					//dynamicly typing

	let none : Option<String> = Option::None;			//the type is falcutative
}
```

Remember that `Option` is a type and always convert always your `Option<T>` to `T` (unless it's of variant `None`):
```rust
let num : u8 = 23;
let option : Option<u8> = Option::Some(3);

println!("{}", num * option);	//Compilation error: "cannot multiply `u8` by `Option<u8>`"
```

This is how Rust stops you assuming a value isn't `null` since if an `Option` is of variant `None`, it will not convert.  
When writing Rust, you want to have code that runs when it receives a `None` and another piece that runs when it receives a `Some` variant.

If you have question left, here is [the documentation](https://doc.rust-lang.org/std/option/enum.Option.html)

**25/11**
### 6.2. How to `match` with enums
#### 6.2.1. Re-discovering the expression
The `match` construct lets you run code for each variant encountered. "_The power of match comes from_ [...] _the fact that the compiler confirms that all possible cases are handled_"(a similar result may be achived with `if..else`s if you're enum has implementes the trait `PartialEq`).

The `match` expression is like a coin-sorting machine, each expression try to enter a hole and if they can't they go to the next.

Here is an example program that I borrowed from the Book:
```rust
//Defining the enum
enum Coin {
    Penny	,
    Nickel	,
    Dime	,
    Quarter	,
}

//Defining a function that returns the value of the coin
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny		=> 1,
        Coin::Nickel 	=> 5,
        Coin::Dime		=> 10,
        Coin::Quarter	=> 25,
    }
}
```
Now you saw this, it might recall you [chapter 2.](#2-guessing-game-project-1) because we used the match statement in [section 2.3.](#23-see-if-the-user-was-right) with the `Ordering` enum.

The match expression try to match each variant with the variant passed (here the `coin` variant from the `Coin` enum), might want to put most used cases first for micro-optimisations.

If you want to have more than 1 line of code, you have to create a scope:
```rust
//Defining the enum
enum Coin {
    Penny	,
    Nickel	,
    Dime	,
    Quarter	,
}

//Defining a function that returns the value of the coin
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny		=> {
			println!("I am worthless!");
			1	//returns 1 anyways
		},
        Coin::Nickel 	=> 5,
        Coin::Dime		=> 10,
        Coin::Quarter	=> 25,
    }
}
```

#### 6.2.2. Getting values bound to variants
Each arm can get the value bound to the variant matched.

For example, "_From 1999 through 2008, the United States minted quarters with different designs for each of the 50 states on one side. No other coins got state designs, so only quarters have this extra value_", the idea is to print the state the coin is from if it's a quarter.

To do this, let's create the enum `State` and modify `Coin`:
```rust
#[derive(Debug)]		//lets us print the state
enum UsState {
    Alabama,
    Alaska,
    // etc...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),	//contains its origin state
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {	//the state is in the variable `state`
            println!("State quarter from {state:?}!");
            25
        }
    }
}

//tests
fn main() {
	println!("The value of a penny  is {}.",   value_in_cents(Coin::Penny ));
	println!("The value of a nickel is {}.",   value_in_cents(Coin::Nickel));	
	println!("The value of a dime   is {}.\n", value_in_cents(Coin::Dime  ));//writing a new line to well see what is printed

	println!("The value of a quarter is {}.", value_in_cents(Coin::Quarter(UsState::Alaska)));
	//out-> "State quarter from Alaska!
	//		The value of a quarter is 25."
}
```

#### 6.2.3. `match`ing with the `Option<T>` type
You already know how to match with an `Option`, it's just the generic type that is confusing. If you're confident, you can write a program that multiplies by 2 the value of an `Option<isize>` and returns the result. Since you cannot multiply `None`, you have to find a workaround.  
We're assuming the result can be represented as an `Option<isize>` so do not worry about [interger overflow](#interger-overflow).
_______

```rust
fn multiply_by_two(n : Option<isize>) -> Option<isize> {
	match n {
		None => None,			//returns None if None is passed
		Some(x)	=> Some(x * 2),	//returns x*2 if Some is passed
	}
}
fn main() {
	let number			= Some(345);				//-> Some(345)
	let can_be_number	= multiply_by_two(number);	//-> Some(690)
	let none			= multiply_by_two(None);	//-> None
}
```

#### 6.2.4. `match` must be exhaustive
You may have learnt it by doing the exercise: **all variant of the checked enum must have a single corresponding arm**. If we choose to not handle the `None` variant in `multiply_by_two`, the compiler will return an error: ``error[E0004]: non-exhaustive patterns: `None` not covered``. It keeps us from doing the "billion-dollar mistake" the Book described earlier by assuming we have a non-null value wxhen we might have one.

#### 6.2.5. The catch-all pattern keyword and the `_` placeholder
This placeholder matches all absent variants.  
To demonstrate the concept, let's take an example. Imagine you're developping a board game in Rust and have three action to do dependant on the result of the 10 face dice roll:
1. If the player rolled a 3, he doesn't move but get a new hat.
2. If the player rolled a 7, he doesn't move and loose one of his hat.
3. Else the player rerolls the dice.

The code should look like this:
```rust
//[definition of `get_new_hat()`, `loose_hat()` and `reroll_dice()`]

fn main() {
	let dice_roll = 8;		//it's random the first time

	match dice_roll {
		3 => get_new_hat(),
		7 => loose_hat(),
		_ => reroll_dice(),
	}
}
```

Now let's hire a better game designer, he tells us to make our character move by the number on the dice. We now have to modify our code to have the value of the dice. In Rust, you can write a _catch-all pattern_ which is a variable that contains the value passed.

```rust
//[definition of `get_new_hat()`, `loose_hat()` and `move_player()`]

fn main() {
	let dice_roll = 2;		//see it changed!

	match dice_roll {
		3 => get_new_hat(),
		7 => loose_hat(),
		catch_all => move_player(catch_all),
	}
}
```
I chose the name ``catch_all`` but any valid variable name works.  
We could have put it higher but all arm below it would not have been reached and Rust throws a warning.

Now an angry tester claims there are not enough hat drops so you decide to give him a version where nothing happens if another number than `3` or `7` is given as a revenge.  
In order to do this you have to return the unit value `()` if another number than `3` or `7` is given.

```rust
//[definition of `get_new_hat()`, `loose_hat()` and `move_player()`]

fn main() {
	let dice_roll = 2;		//see it changed!

	match dice_roll {
		3 => get_new_hat(),
		7 => loose_hat(),
		_ => (),		//return void; does nothing
	}
}
```

We we'll learn more about patterns in [chapter 18.](#18-).

Let's learn about the `if let` syntax for the cases where `match` are too much writing.

**01/12**
### 6.3. How to use the `if let` expression
This will be a short section since there isn't much to say.

The `if let` expression combines the `if` expression and the `let` statement. It is another shorthand for checking patterns.  
Those two programs are equivalent:
```rust
//[definition of `correct_input`]
match correct_input {	//`correct_input` is an `Option<u8>`
	Some(input) => println!("You entered the number {input}."),
	_ 			=> ()
}
```

```rust
//[definition of `correct_input`]
if let Some(input) = correct_input {
	println!("You entered the number {input}.");
}
```

If `correct_input` is defined it prints it, else is doesn't.

The `if else`'s syntax is sure short and requires less indenting (you have to write at least 2 indents for a mutiline code in a `match` against at least 1 in an `if let`) but is not exhaustive: you don't have to explicitly detail all cases. So be careful when implementing it, you could forgot some cases.

We can add an `else if` and `else` to an `if let` block, it replaces the placeholer `_`:
```rust
//you alredy saw the definition of `UsState`
#[derive(Debug)]
enum EUCountry {
	France,
	Italy,
	Belgium,
	//etc...
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),	//contains its origin state
	Euro(EUCountry),		
}

fn main() {
	//`user_input` is a `Coin` variant
	if let Coin::Quarter(input) = user_input {
		println!("Your coin is from the american state \"{input:?}\".");
	}else if let Coin::Euro(input) = user_input {
		println!("Your coin is from \"{input:?}\"");
	}else {
		println!("We cannot determine from which american state this coin is.");
	}
}
```

It is however difficult to read and understand where a `match` bloc would have been more secured and more straightforward:
```rust
match user_input {
	Coin::Quarter(input) 	=> println!("Your coin is from the american state \"{input:?}\"."),
	Coin::Euro(input) 		=> println!("Your coin is from \"{input:?}\""),
	_						=> println!("We cannot determine from which american state this coin is."),
}
```
So be careful and think about readability of your code especially when working with other people.

### 6.summary
You now know how to use enumerations but do not forget structures each one has its use cases and when developping an API (a programming interface (not real meaning)), you want users to know what to do with a certain type of data.  
You want to have a good type safety so just working with `String`s will not work (think to languages like Javascript).

If you are developping an API, you will need to know how to use modules and lucky you it's the next chapter's subject!

## 7. Packages, crates and modules let you organise your code
Until now we've only been writing code inside one file which on larger projects, is very bad design. All project have this hierarchy: _workspaces->package->crate->module->file_ you can have several of them in a superior one. Think of it as boxes, you can put smaller boxes inside bigger ones hence the fact you can have multiples packages in a workspace.  
In this chapter we'll cover each one except _worspaces_ since it is only used on very large project, you have to wait until [chapter 14.](#14-).

We'll also cover encapsulation which is a OOP concept that let you hide or forbid the use of some field or methods. In a program you would say that a member is _public_ or _private_.

We will also talk about scopes and how to resolve conflicting names. Those scopes aren't those from your code which are defined by curly brackets `{}`, they are scopes inside project hierarchy.

Before starting let's see some definitions:


**The module system:**  
This system is the one containing all the features that lets organize your code, it contains all the feature listed below.

**Packages:**  
A way provided by Cargo to build, test and share crates.

**Crates:**  
A tree of modules that produces once compiled an executable or a library (binary and library crates).

**Modules (and `use`):**  
Rust feature that lets you control the organization of your code and use paths.

**Paths:**  
A way to name items like structures, function or modules.

### 7.1. Packages and crates
Crates are the smallest piece of code the compiler can handle, even our very first project is considered a crate. There are two types of crates:
- Binary crates: A program that can run, all the crates we've created are binary crates.
- Library crates: A crate that provides functions, types, .... They don't have any `main()` functions and cannot compile to an executable by themselves, they need to be included by an binary crate to be used. When informal, rustacean use the word "crate" to imply "library crate", they use this word as a synonym of the concept of libraries in other programming languages.

The _crate root_ is the binary crate that the compiler starts from.

A package is a set of crates but:
+ it must have a Cargo.toml that say how to build the crates.
+ it must contain at least one (binary or library) crate.
+ it must have at most one library crate.


### 7.2. Defining Modules to Control Scope and Privacy
#### 7.2.1. Modules Cheat Sheet
This sub-section is a quick reference to about the usage of modules, paths, `use` and `pub` and how most of developpers make tidy code. There will be more examples later in this chapter but if you ever forget how one of these work, jump here.

- **Declaring modules**: in the crate root, you can declare modules with the `mod` keyword. Let's say you want a module named "garden", you will write `mod garden;` and the compiler will look for the module's code in those places:
	+ Inline: inside the curly `{}` you put instead of the semi-colon `;`.
	+ In the file `src/garden.rs`
	+ In the file `src/garden/mod.rs`

- **Declaring sub-modules**: in any other file that the crate root, you can declare a sub-module the same way you would for regular modules. If you want add to `garden` a sub-module named `vegetable`, you'd write `mod vegetable;` in `src/garden.rs` and the compiler would search its code in those places:
	+ Inline
	+ In the file `src/garden/vegetable.rs`
	+ In the file `src/garden/vegetable/mod.rs`

- **Using the code inside those modules**: Once you have a module, you can access its code anywhere in the same crate as long you respect privacy rules. For example, you can declare the type `Carrot` inside `vegetable`, you would access it via the path `crate::garden::vegetable::Carrot`.

- **Private vs public**: By default, all code in a module is private, to make it public use `pub`.

- **Using `use`**: This keyword is like in C++, it shorten the length of the paths you have to write. For example, if you need to use a lot the `vegetable` module, you can write `use crate::garden::vegetable;` it lets you access the elements inside the module easily since you can now just write `Carrot` to access the type.

Now we've seen the global picture, let's see the details.

#### 7.2.2. Grouping code
Modules let us define the privacy of their elements. Private elements are only available to other modules elements, they are for internal use. Public elements are available everywhere the module is included.

To understand all of this, let's create a restaurant library crate. In the restaurant industry, there are 2 sides: the _front of the house_ and the _back of the house_. The front of the house is what is seen by the customers: waiter serving food, bartenders preparing cocktails or the hosts seating the customers. The back of the house is whet is not shown, chefs coocking, dishwasher cleaning the plates and manager doing administrative work.  
We will only write code for the front of the house.

We can create a new library with Cargo by passing it the `--lib` argument.
We can then compile this code in `src/lib.rs`:
```Rust
mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {}

		fn seat_at_table() {}
	}

	mod serving {
		fn take_order() {}

		fn serve_order() {}

		fn take_payment() {}
	}
}
```

Modules are like namespaces, they can contain definitions for types, constants and traits.  
Modules let us visually see which element is related to which.

### 7.3. Paths for Referring to an Item in the Module Tree
#### 7.3.1. What is a path
In Rust, paths for functions are the same thing as paths for files just with `::` istead of `/` (or `\` in Windows). For beginners, there are two different types of paths:
- Absolute paths: They go for the crate root to the desired function.
- Relative paths: They begin at the current module.

As an example, we can create the function `eat_at_restaurant()` that will use those two types of paths:
```Rust
mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {}
	}
}

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();	//`crate` represent the current crate

	// Relative path
	front_of_house::hosting::add_to_waitlist();
}
```
This code does not compile because we didn't make `add_to_waitlist()` public, how to make it public? Read the next section to learn this.

#### 7.3.2. Using the `pub` keyword
As we seen in the [cheatsheet](#721-modules-cheat-sheet), to make something public we need to use the `pub` keyword:
```Rust
mod front_of_house {
	pub mod hosting {
		fn add_to_waitlist() {}
	}
}

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();
}
```
But this code does not compile too, we get this error message:
```
function `add_to_waitlist` is private
```

Even if the _module_ `hosting` is public, all functions inside are by default private. This fix this error, we need to add another `pub`:
```Rust
mod front_of_house {
	pub mod hosting {
		pub fn add_to_waitlist() {}
	}
}

pub fn eat_at_restaurant() {
	// Absolute path
	crate::front_of_house::hosting::add_to_waitlist();

	// Relative path
	front_of_house::hosting::add_to_waitlist();
}
```

If you ever plan to write a library, there's a lot more to take into account than just public or private functions, or even pure code. Those consideration are not in the Book but there is the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) that explain convention and good practises.

#### 7.3.3. The usage of `super` in relative paths
Using the `super` keyword is equivalent of using `..` in a filepath, it elevate the scope to the parent scope. To get an example, let's take the old `front_of_the_house` module:
```Rust
mod front_of_house {
	mod hosting {
		fn add_to_waitlist() {}
		fn seat_at_table() {}

		fn new_client() {
			add_to_waitlist();
			super::do_nothing();
			//If we wanted to use an absolute path we'd have to write this:
			//crate::front_of_house::do_nothing()
			
			seat_at_table();
		}
	}

	fn do_nothing() {}
}
```

#### 7.3.4. Strutures and enumarations' privacy
Structures follow the same rules as modules: every member is private by default:
```Rust
struct Menu {
	pub mealAvailable   : [String; 3],
	culinaryExperiments : [String; 3]
}

//can only access `mealAvailable`
```

Enumerations follow the opposite rule: every variant is public by default, that's because an enum with only private field is useless:
```Rust
enum dishKind {
	Soup,
	Salad,
	Buger,
	Vegan,
	//...
}

//All kinds	of dishes are available
```

### 7.4. Bringing Paths into Scope with the `use` Keyword
#### 7.4.1. `use` and scopes
The `use` keyword is a bit like the `using` keyword in C++ but more restricted. `use` have only one purpose: shorten paths.  
ie:
```Rust
mod Languages {
	pub mod Rust {
		pub fn compile() {}
	}
}

use Languages::Rust::compile;
compile();	//valid
```

The shortcut is only valid **in the exact same scope** as the keyword is used.

So this will not compile:
```Rust
mod Languages {
	pub mod Rust {
		pub fn compile() {}
	}
	use Languages::Rust::compile;	//throw a warning because it is not used

	pub mod Javascript {
		pub fn execute() {
			compile();	//throw an error
		}
	}
}
```

#### 7.4.2. Idoms
In the previous sub-section, we imported `compile()` by writing its full path instead of the path to its parent module, because for one function it's okay but for 4:
```Rust
mod Languages {
	pub mod Rust {
		pub fn compile()	{}
		pub fn assemble()	{}
		pub fn link()		{}
		pub fn execute()	{}
	}

	use Languages::Rust::compile;
	use Languages::Rust::assemble;
	use Languages::Rust::link;
	use Languages::Rust::execute;
	pub fn rustc() {
		compile();
		assemble();
		//...
	}
}
```

That's difficult to read, why not just:
```Rust
mod Languages {
	pub mod Rust {
		pub fn compile()	{}
		pub fn assemble()	{}
		pub fn link()		{}
		pub fn execute()	{}
	}

	use Languages::Rust;
	pub fn rustc() {
		Rust::compile();
		Rust::assemble();
		//...
	}
}
```
Which is more compact. The answer is conventions, we should only create shortcuts for what we need else that's not _"idiomatic"_. There is however one exception, when importing two objects with the same name since it would create a name conflict: the reason ~~namespaces~~ modules were invented.

#### 7.4.3. Creating aliases
The `as` keyword is similar to Python's when importing.

```Rust
use std::fmt::Result as FmtResult;
use std::io::Result  as IoResult;

fn function1() -> FmtResult {}

fn function2() -> IoResult<()> {}
```

#### 7.4.4. Re-exporting
The imported names are by default private and cannot be accessed in other scopes, this can be avoided by mking them public with `pub`:
```Rust
mod Discord {
	pub mod Message {
		pub enum Type {
			text,
			command,
			audio,
			image,
			video
		}
	}
	
	pub use Message::Type as MsgType;
}

fn guessMessageType(message: String) -> Discord::MsgType {}
```
This method is called re-exporting, it's useful when restructuring for users (other programmers) since you may want to see the library you built in a different manner that you wrote it.

#### 7.4.5. Nesting paths
If you recall [7.4.2.](#742-idoms), we had to write many paths to get all functions into scopes. We can limit the number of `use` with nested paths:
```Rust
mod Languages {
	pub mod Rust {
		pub fn compile()	{}
		pub fn assemble()	{}
		pub fn link()		{}
		pub fn execute()	{}
	}

	//three lines saved
	use Languages::Rust::{compile, assemble, link, execute};
	pub fn rustc() {
		compile();
		assemble();
		//...
	}
}
```

If we want to only import `execute()` in the scope and the `Rust` module, we could
```Rust
use Languages::Rust;
use Languages::Rust::execute;
```
or without any redundancy
```Rust
use Languages::Rust::{self, execute};
```
where `self` represents the `Rust` module.

We're going a bit off tracks but if you wondered, you cannot catch two names and then get a shared children. So the following code won't compile
```Rust
mod Languages {
	pub mod Lua {
		pub mod Function {
			pub fn getName() {}
		}
	}
	pub mod Bash {
		pub mod Function {
			pub fn execute() {}
		}
	}
}

use Languages::{Lua, Bash}::Function::{getName, execute};	// after curly brackets a semicolon is expetcted
```

#### 7.4.6. The glob operator
You can import all elements of a module with the glob operator `*`:
```Rust
mod Languages {
	pub mod English	{}
	pub mod French	{}
	pub mod Spanish	{}
	pub mod German	{}
	pub mod Russian	{}
}

use Language::*;
```
Although you can do it, it doesn't mean you should since with operator you can't see the names you are importing which can lead to name confilcts and a less readable code.

### 7.5. Separating Modules into Different Files
All of this is great but what if we want to code on multiple files? We did mention this in our [cheatsheet](#721-modules-cheat-sheet), it is the notation with the semicolon `;`:
```Rust
mod File;		//The compiler will look in src/File.rs and in src/File/mod.rs
```

Remember that `use` isn't an `import` nor an `#include`, all it does is create shorthands and reducing paths. We could not write `use` and access the function with an [absolute path](#731-what-is-a-path).

### 7.summary
Rust allow splitting your code in multiples files and modules (namespaces). If an element is from anther file you don't have to import it, you can use an absolute path. In order to avoid always using long paths, you can use the `use` feature. All members of a module are private by default but you can write `pub` before their declaration to make them public.

## 8. Common Collections
Collections are, as their name suggest, collections of values. They are all stored on the heap so they can grow and shrink overtime unlike strings and tuples.  
This chapter is about the most common ones:
+ *vector*s that are the most similar to arrays.
+ *string*s that are collections of chars.
+ *hash map*s that let you bind a key to a value and their more general form _maps_.

If you have questions, you can see the [documentation](https://doc.rust-lang.org/std/collections/index.html).

### 8.1. Storing Lists of Values with Vectors
The type of vectors is _`Vec<T>`_, as a remainder the `<T>` indicate that `Vec` uses generics (templates) which can be any type. You can see vectors as arrays that can grow and shrink.

#### 8.1.1. Vector basics
In order to create a vector, there is two syntaxes:
```Rust
let empty : Vec<char> = Vec::new();				//mandatory type annotation
let full  : Vec<char> = vec!['a', 'b', 'c'];
```

The first one uses the `Vec::new()` method to create an empty vector, you have to annotate the type of `empty` because of static type checking: Rust doesn't know what is the type of `empty` since it could be `Vec<u32>`, `Vec<f64>` or even `Vec<[char; 4]>`.  
The second one uses the `vec!` macro, it doesn't require type annotation since `'a'`, `'b'` and `'c'` can only be chars.


You can push new values at the end of vectors with the `Vector.push()` method:
```Rust
let mut squares : Vec<u32> = Vec::empty();

squares.push(1);
squares.push(4);
squares.push(9);
squares.push(16);
squares.push(25);
```

To access the values of a vector, you can use brackets like for arrays or the `Vec.get()` method:
```Rust
let pseudo_str : Vec<char> = vec!['H', 'e', 'l', 'l', 'W'];

let win		: char  =  pseudo_str[4];		//->'W'
let win_ref	: &char = &pseudo_str[4];		//->'W'

let loss	: Option<&char> = pseudo_str.get(3);
match loss {
	Some(ch) => println!("The element to index 3 is ' {}'.", ch),
	None	 => println!("There is no element at index 3."),
}	//out->The element to index 3 is 'l'.

let lost	: Option<&char> = pseudo_str.get(42);	//Out of Bounds
match lost {
	Some(ch) => println!("The element to index 42 is ' {}'.", ch),
	None	 => println!("There is no element at index 42."),
}	//out->There is no element at index 42.
```

The difference between brackets and `get()` is that `get()` returns `None` if the index is out of bound whereas brackets will cause the program to panic:
```Rust
let pseudo_str : Vec<char> = vec!['H', 'e', 'l', 'l', 'W'];

pseudo_str.get(24);		//->None
pseudo_str[24];			//error
```


<!--Already?-->
Remember the borrow checker? I hope so because [references](#42-borrowing-and-references) rules still hold:
```Rust
let mut v = vec![9, 8, 7, 6, 5];
let no = &v[0];

v.push(4);		//compile error, v is borrowed as a mutable reference

println!("{} is my favorite number!", no);
```

If you want how the `Vec` type works under the hood, you can see [this guide](https://doc.rust-lang.org/nomicon/vec/vec.html) that shows how to build the type from scratch.

#### 8.1.2. Iterating over a vector
If we want to iterate over a vector , we have to use a [`for`](#3535-the-for-loop) loop.
```Rust
let warudo = vec!['T', 'H', 'E', ' ', 'W', 'O', 'R', 'D', '!'];
for character in &warudo {
	print!("{character}");
}		//out->"THE WORD!"
```

We can also change vector's elements with mutable references:
```Rust
let mut song = vec!["Stayin", "' ", "Alive"];
for word in &mut song {
	*word = "Ha! ";
	print!("{word}");
}	//out-> "Ha! Ha! Ha! "

//song == vec!["Ha! ", "Ha! ", "Ha! "]
```

#### 8.1.3. Multiple types
Vectors can only contain one type but you can enums to bypass this limit:
```Rust
enum Number {
	Int(i32),
	Float(f32),
	NaN,
}

let heterogen = vec![
	Number::Int(6),
	Number::Float(9.6),
	Number::NaN
];
```

#### 8.1.3. Deleting a vector
Deleting a vector will free all of its elements, any reference will produce an error by the borrow checker:
```Rust
{
	let boring_vec = vec![1, 2, 3];
	//...
}  // boring_vec is out of scope here
```

### 8.2. Storing UTF-8 Encoded Text with Strings