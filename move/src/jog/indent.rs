use std::{io, str};

const EOL: u8 = b'\n';
const INDENT: &[u8] = b"  ";

pub trait Indent<W: io::Write> {
    fn indent(&mut self) -> IndentWriter<W>;
}

impl<W: io::Write> Indent<W> for W {
    fn indent(&mut self) -> IndentWriter<W> {
        IndentWriter {
            inner: self,
            new_line: true,
        }
    }
}

pub struct IndentWriter<'a, W: io::Write> {
    inner: &'a mut W,
    new_line: bool,
}

impl<W: io::Write> IndentWriter<'_, W> {
    fn write_line(&mut self, line: &str) -> Result<usize, io::Error> {
        if self.new_line && line.as_bytes()[0] != EOL {
            self.inner.write_all(INDENT)?;
        }

        self.new_line = true;
        self.inner.write(line.as_bytes())
    }
}

impl<W: io::Write> io::Write for IndentWriter<'_, W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, io::Error> {
        let string = from_utf8(buf)?;
        let matches = string.match_indices(char::from(EOL));

        let mut count = 0;
        let mut position = 0;

        for (i, _) in matches {
            let line = unsafe { string.get_unchecked(position..=i) };
            count += self.write_line(line)?;
            position = i + 1;
        }

        if position < string.len() {
            let remainder = unsafe { string.get_unchecked(position..string.len()) };
            count += self.write_line(remainder)?;
            self.new_line = false;
        }

        Ok(count)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        self.inner.flush()
    }
}

fn from_utf8(buf: &[u8]) -> Result<&str, io::Error> {
    str::from_utf8(buf).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
}

#[cfg(test)]
mod tests {
    use super::*;
    use io::Write;

    fn indent(expected: &str, write: fn(&mut IndentWriter<io::Cursor<Vec<u8>>>)) {
        let mut buf = io::Cursor::new(Vec::new());
        let mut writer = buf.indent();

        write(&mut writer);
        let actual = String::from_utf8_lossy(buf.get_ref());

        assert_eq!(actual, expected);
    }

    #[test]
    fn indent_single() {
        indent("  Hello", |w| {
            write!(w, "Hello").unwrap();
        });

        indent("  Hello\n  World", |w| {
            write!(w, "Hello\nWorld").unwrap();
        });

        indent("  Hello\n\n  World", |w| {
            write!(w, "Hello\n\nWorld").unwrap();
        });

        indent("  Hello\n  World\n", |w| {
            writeln!(w, "Hello\nWorld").unwrap();
        });

        indent("  HelloWorld", |w| {
            write!(w, "Hello").unwrap();
            write!(w, "World").unwrap();
        });

        indent("  Hello\n  World", |w| {
            writeln!(w, "Hello").unwrap();
            write!(w, "World").unwrap();
        });

        indent("  Hello\n  World\n", |w| {
            writeln!(w, "Hello").unwrap();
            writeln!(w, "World").unwrap();
        });

        indent("\n  Hello\n  World\n", |w| {
            writeln!(w, "\nHello").unwrap();
            writeln!(w, "World").unwrap();
        });

        indent("  Hello\n\n  World\n\n", |w| {
            writeln!(w, "Hello\n").unwrap();
            writeln!(w, "World\n").unwrap();
        });
    }

    #[test]
    fn indent_double() {
        indent("  Hello\n    Bonjour\n  World\n", |w| {
            writeln!(w, "Hello").unwrap();
            writeln!(w.indent(), "Bonjour").unwrap();
            writeln!(w, "World").unwrap();
        });
    }

    #[test]
    fn indent_triple() {
        indent(
            "  Hello\n    Bonjour\n      Hola\n      Hallo\n  World\n",
            |w| {
                writeln!(w, "Hello").unwrap();

                let mut indented = w.indent();
                writeln!(indented, "Bonjour").unwrap();

                writeln!(indented.indent(), "Hola").unwrap();
                writeln!(w.indent().indent(), "Hallo").unwrap();

                writeln!(w, "World").unwrap();
            },
        );
    }
}
