> [!WARNING]
> The project was created solely for educational purposes.

# Wut Cursor

A lightweight iterator over a sequence of characters.

## Usage

Add this to your Cargo.toml:

```toml
wur_cursor = { version = "0.1.0", git = "https://github.com/Refr900/wut_cursor"}
```

Then, you can use it:

```rust
use wut_cursor::Cursor;

fn main() {
    let source = r#"
    fn main() {
        println!("Hello world!");
    }
    "#
    .trim_start();

    let mut cursor = Cursor::new(source);
    let mut index = 0;
    let mut count = 1;
    loop {
        let token = cursor.next();
        let end = index + token.len as usize;
        let lexeme = &source[index..end];
        println!("{:>3}: {:?}", count, token.kind);
        println!("   |  lexeme: {:?}", lexeme);
        if token.is_eof() {
            break;
        }
        index = end;
        count += 1;
    }
}


```

## Subtleties

By default, the cursor does not parse strings or characters, but has a default implementation that supports escaped symbols.

```rust
use wur_cursor::{Cursor, Kind};

fn main() {
    let source = r#"
    fn main() {
        print!("Hello world!\n");
    }
    "#
    .trim_start();

    let mut cursor = Cursor::new(source);
    let mut index = 0;
    let mut count = 1;
    loop {
        let token = cursor.next();
        let mut end = index + token.len as usize;

        match token.kind {
            Kind!['"'] => {
                // ready-made string parsing
                let str = cursor.parse_str_continue();
                end = index + str.len as usize;
            }
            Kind!['\''] => {
                // ready-made character parsing
                let char = cursor.parse_char_continue();
                end = index + char.len as usize;
            }
            _ => (),
        };

        let lexeme = &source[index..end];
        println!("{:>3}: {:?}", count, token.kind);
        println!("   |  lexeme: {:?}", lexeme);

        if token.is_eof() {
            break;
        }
        index = end;
        count += 1;
    }
}
```

And yes, there is a macro for the token kind.

## License

See the [LICENSE](LICENSE) file for license rights and limitations (MIT).
