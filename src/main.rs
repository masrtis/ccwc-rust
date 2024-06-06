use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::path;
use std::path::PathBuf;
use std::string::FromUtf8Error;

#[derive(Debug)]
enum WcError {
    Io(io::Error),
    InvalidUtf8(FromUtf8Error),
    CommandLine,
}

impl fmt::Display for WcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::InvalidUtf8(err) => err.fmt(f),
            Self::CommandLine => {
                write!(f, "Usage: ccwc -<c,l,w,m> <filename>")
            }
        }
    }
}

impl Error for WcError {}

enum CountCommandLine {
    Bytes,
    Lines,
    Words,
    Chars,
}

#[derive(Clone)]
enum CountCommand {
    Bytes(Vec<u8>),
    Lines(Vec<u8>),
    Words(Vec<u8>),
    Chars(Vec<u8>),
}

impl CountCommand {
    fn process(&self) -> Result<String, FromUtf8Error> {
        match self {
            Self::Bytes(bytes) => Ok(bytes.len().to_string()),
            Self::Lines(bytes) => Ok(bytes.lines().count().to_string()),
            Self::Words(bytes) => Ok(String::from_utf8(bytes.to_vec())?
                .split_whitespace()
                .filter(|s| !s.is_empty())
                .count()
                .to_string()),
            Self::Chars(bytes) => Ok(String::from_utf8(bytes.to_vec())?
                .chars()
                .count()
                .to_string()),
        }
    }

    fn prepare(
        command_string: CountCommandLine,
        file_contents: Vec<u8>,
    ) -> io::Result<CountCommand> {
        match command_string {
            CountCommandLine::Bytes => Ok(CountCommand::Bytes(file_contents)),
            CountCommandLine::Lines => Ok(CountCommand::Lines(file_contents)),
            CountCommandLine::Words => Ok(CountCommand::Words(file_contents)),
            CountCommandLine::Chars => Ok(CountCommand::Chars(file_contents)),
        }
    }
}

struct CommandLine {
    actions: Vec<CountCommand>,
    file_path: Option<path::PathBuf>,
}

impl CommandLine {
    fn process(&mut self) -> Result<(), FromUtf8Error> {
        let counts: String = self
            .actions
            .iter()
            .map(|action| action.clone().process())
            .collect::<Result<Vec<String>, FromUtf8Error>>()?
            .join(" ");

        match &self.file_path {
            Some(path) => {
                println!("{} {}", counts, path.display());
            }
            None => {
                println!("{}", counts);
            }
        }

        Ok(())
    }
}

fn is_file(path: &str) -> bool {
    fs::metadata(path)
        .map(|metadata| metadata.is_file())
        .unwrap_or(false)
}

struct ParseArgsResult {
    parsed_commands: Vec<CountCommandLine>,
    file_path: Option<PathBuf>,
}

fn parse_args() -> Result<ParseArgsResult, WcError> {
    let args_iter = env::args();
    if args_iter.len() > 3 {
        println!("{}", WcError::CommandLine);
        return Err(WcError::CommandLine);
    }

    let args: Vec<String> = args_iter.collect();
    let command = args
        .get(1)
        .map(|s| s.to_string())
        .filter(|s| !is_file(s))
        .unwrap_or_default();
    let last_arg_is_input_file = args.len() > 1 && is_file(args.last().unwrap());
    let file_path = if last_arg_is_input_file {
        Some(PathBuf::from(args.last().unwrap()))
    } else {
        None
    };

    Ok(ParseArgsResult {
        parsed_commands: match &command[..] {
            "-c" => vec![CountCommandLine::Bytes],
            "-w" => vec![CountCommandLine::Words],
            "-l" => vec![CountCommandLine::Lines],
            "-m" => vec![CountCommandLine::Chars],
            _ => vec![
                CountCommandLine::Bytes,
                CountCommandLine::Words,
                CountCommandLine::Lines,
            ],
        },
        file_path,
    })
}

fn prepare_commands(parsed_command_line: ParseArgsResult) -> io::Result<CommandLine> {
    let mut reader: Box<dyn io::Read> = if let Some(ref file_path) = parsed_command_line.file_path {
        println!("Opening file {}", file_path.display());
        let file = fs::File::open(file_path)?;
        Box::new(io::BufReader::new(file))
    } else {
        println!("Reading from stdin");
        Box::new(io::stdin())
    };

    let mut contents: Vec<u8> = Vec::new();
    reader.read_to_end(&mut contents)?;

    let preparation_result: io::Result<Vec<CountCommand>> = parsed_command_line
        .parsed_commands
        .into_iter()
        .map(|command_string| CountCommand::prepare(command_string, contents.clone()))
        .collect();

    match preparation_result {
        Ok(commands) => Ok(CommandLine {
            actions: commands,
            file_path: parsed_command_line.file_path,
        }),
        Err(io_error) => Err(io_error),
    }
}

fn main() -> Result<(), WcError> {
    let parsed_command_line = parse_args()?;

    prepare_commands(parsed_command_line)
        .map_err(WcError::Io)?
        .process()
        .map_err(WcError::InvalidUtf8)?;

    Ok(())
}
