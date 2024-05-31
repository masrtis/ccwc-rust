use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::path;

#[derive(Debug)]
enum WcError {
    Io(io::Error),
    CommandLine,
}

impl fmt::Display for WcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err) => {
                return err.fmt(f);
            }
            Self::CommandLine => {
                write!(f, "Usage: ccwc -<c,l,w> <filename>")
            }
        }
    }
}

impl Error for WcError {}

enum Command {
    CountBytes,
    CountLines,
    CountWords,
    CountChars,
}

trait Action {
    fn process(&self, file_path: &path::Path) -> io::Result<String>;
}

impl Action for Command {
    fn process(&self, file_path: &path::Path) -> io::Result<String> {
        match self {
            Self::CountBytes => {
                let metadata = file_path.metadata()?;

                return Ok(metadata.len().to_string());
            }
            Self::CountLines => {
                let file = fs::File::open(file_path)?;
                let read_buffer = io::BufReader::new(file);

                return Ok(read_buffer.lines().count().to_string());
            }
            Self::CountWords => {
                let mut file = fs::File::open(file_path)?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;
                let mut in_word = false;
                let mut word_count = 0;

                for c in buffer.chars() {
                    if in_word {
                        if c.is_whitespace() { 
                            in_word = false;
                            word_count += 1;
                        }
                    } else {
                        in_word = !c.is_whitespace();
                    }
                }
                
                return Ok(word_count.to_string());
            },
            Self::CountChars => {
                let mut file = fs::File::open(file_path)?;
                let mut buffer = String::new();
                file.read_to_string(&mut buffer)?;

                return Ok(buffer.chars().count().to_string());
            },
        }
    }
}

struct CommandLine {
    actions: Vec<Command>,
    file_path: path::PathBuf,
}

impl CommandLine {
    fn process(&self) -> io::Result<()> {
        let counts : String = self.actions
            .iter()
            .map(|action| action.process(self.file_path.as_path()))
            .collect::<io::Result<Vec<String>>>()?.join(" ");

        println!("{} {}", counts, self.file_path.display());

        Ok(())
    }
}

fn parse_args() -> Option<CommandLine> {
    let mut args_iter = env::args();
    if args_iter.len() < 2 || args_iter.len() > 3 {
        return None;
    }

    let first_arg = args_iter.nth(1)?;
    let (action, file_path) = match &first_arg[..] {
        "-c" => (vec![Command::CountBytes], args_iter.nth(0)?),
        "-l" => (vec![Command::CountLines], args_iter.nth(0)?),
        "-w" => (vec![Command::CountWords], args_iter.nth(0)?),
        "-m" => (vec![Command::CountChars], args_iter.nth(0)?),
        _ => (vec![Command::CountLines, Command::CountWords, Command::CountBytes], first_arg),
    };

    Some(CommandLine {
        actions: action,
        file_path: path::PathBuf::from(file_path),
    })
}

fn main() -> Result<(), WcError> {
    match parse_args() {
        Some(command_line) => {
            command_line
                .process()
                .map_err(|err: io::Error| WcError::Io(err))?;
        }
        None => {
            println!("{}", WcError::CommandLine);
            return Err(WcError::CommandLine);
        }
    }

    Ok(())
}
