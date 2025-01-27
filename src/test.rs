#[test]
fn test() {
    use crate::{Cursor, Kind};
    let source = r#"
#export main
#extern std::print

#def str "hello world!"
#def char '\n'

main:
    mov  r0, 1
    mov  r1, 0b11
    mov  r2, %len[str]
    mov  r3, str
    call std::print
    halt
    "#
    .trim_start();
    let mut cursor = Cursor::new(source);
    let mut index = 0;
    let mut count = 1;
    loop {
        let token = cursor.next();
        let mut end = index + token.len as usize;

        match token.kind {
            // ready-made string parsing
            Kind!['"'] => {
                let str = cursor.parse_str_continue();
                end = index + str.len as usize;
            }
            // ready-made character parsing
            Kind!['\''] => {
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
