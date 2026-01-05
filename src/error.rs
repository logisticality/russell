pub enum Error {
    Lex(usize),
}

pub fn report_error(error: Error, file: &String) -> String {
    unimplemented!()
}

///
fn error_position(place: usize, file: &String) -> (usize, usize) {
    let mut line = 1;
    let mut column = 1;

    for (index, character) in file.chars().enumerate() {
        if index >= place {
            break;
        }

        if character == '\n' {
            line = line + 1;
            column = 1;
        }
    }

    return (line, column);
}
