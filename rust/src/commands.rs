use std::{fmt, fs::File, io::Write};

use crate::{constants::DEFAULT_WIDTH, hanb::Navigator, print_level_board};

#[derive(Debug)]
enum ArgTypes {
    String,
    Int,
}

impl std::fmt::Display for ArgTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgTypes::String => write!(f, "string"),
            ArgTypes::Int => write!(f, "int"),
        }
    }
}

pub enum ArgValue<T> {
    String(String),
    Int(u16),
    Err(T),
}

struct CmdArg<'a> {
    /// Name of the argument
    name: &'a str,
    /// Description
    description: &'a str,
    /// Default value for this argument. If None, this argument is required
    default: Option<&'a str>,
    /// Type of the argument
    type_: ArgTypes,
}

impl<'a> CmdArg<'a> {
    /// Converts this argument to its defined type.
    fn parse(&self, value: &str) -> ArgValue<String> {
        match self.type_ {
            ArgTypes::String => ArgValue::String(value.to_string()),
            ArgTypes::Int => {
                let default = value.parse::<i32>();
                if let Ok(default) = default {
                    ArgValue::Int(default as u16)
                } else {
                    ArgValue::Err(format!("{} is not a valid integer", value))
                }
            }
        }
    }
    fn default(&self) -> ArgValue<String> {
        match self.default {
            Some(value) => self.parse(value),
            None => ArgValue::Err(format!(
                "Argument {} is required. Check 'help command'",
                self.name
            )),
        }
    }
}

pub struct Command<'a> {
    /// Name of the command
    pub command: &'a str,
    /// Callback function of the command taking the hanb navigator and the arguments
    pub action: fn(&Command, &mut Navigator, &str) -> Result<String, String>,
    /// Short form of the command
    pub short: &'a str,
    /// Should the command output to stdout?
    pub stdout: bool,
    /// Is this command only available in the repl?
    pub repl_only: bool,
    /// Description of the command
    help: &'a str,
    /// Arguments of the command
    args: &'a [CmdArg<'a>],
}

impl fmt::Display for Command<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.command)
    }
}

impl<'a> Command<'a> {
    /// Parses arguments into a vector of strings. Arguments are separated by spaces, and will
    /// collapse last argument if the command can't take more
    pub fn argparse(&self, args: &str) -> Result<Vec<ArgValue<String>>, String> {
        let mut args = args.split_whitespace();
        let mut parsed_args = Vec::new();
        for arg in self.args.iter() {
            let value = match args.next() {
                Some(value) => arg.parse(value),
                None => arg.default(),
            };
            if let ArgValue::Err(e) = value {
                return Err(e);
            }
            parsed_args.push(value);
        }
        // Colapse next args into last of parsed_args
        if let Some(last) = parsed_args.last_mut() {
            match last {
                ArgValue::String(s) => {
                    for arg in args {
                        s.push(' ');
                        s.push_str(arg);
                    }
                }
                ArgValue::Int(i) => {
                    for arg in args {
                        let parsed = arg.parse::<i32>();
                        if parsed.is_err() {
                            return Err(format!(
                                "Invalid argument for {}: '{}'",
                                self.command, arg
                            ));
                        }
                        *i += parsed.unwrap() as u16;
                    }
                }
                ArgValue::Err(_) => return Err("Invalid argument".to_string()),
            }
        }
        Ok(parsed_args)
    }
}

enum CommonArgs {
    CommandName,
    Board,
    Cell,
    Filename,
    ScriptName,
}

impl CommonArgs {
    const fn value(&self) -> CmdArg {
        match self {
            CommonArgs::CommandName => CmdArg {
                name: "command",
                description: "Command name to get help from",
                default: None,
                type_: ArgTypes::String,
            },
            CommonArgs::Board => CmdArg {
                name: "board",
                description: "Board as string",
                default: Some("."),
                type_: ArgTypes::String,
            },
            CommonArgs::Cell => CmdArg {
                name: "cell",
                description: "Cell number to use",
                default: Some("0"),
                type_: ArgTypes::Int,
            },
            CommonArgs::Filename => CmdArg {
                name: "filename",
                description: "File to use",
                default: Some("hanb.hsit"),
                type_: ArgTypes::String,
            },
            CommonArgs::ScriptName => CmdArg {
                name: "script",
                description: ".hanb script path",
                default: Some("script.hanb"),
                type_: ArgTypes::String,
            },
        }
    }
}

pub const COMMANDS: &[Command] = &[
    Command {
        command: "help",
        short: "h",
        help: "Prints this help message",
        args: &[CommonArgs::CommandName.value()],
        stdout: true,
        repl_only: false,
        action: |cmd, _navigator, args| {
            println!("Hanb help:");
            let args_res = cmd.argparse(args);
            match args_res {
                Ok(args) => {
                    let cmdname = match args.first().unwrap() {
                        ArgValue::String(cmdname) => cmdname,
                        _ => unreachable!(),
                    };
                    for cmd in COMMANDS {
                        if cmd.command == cmdname {
                            println!("{}: {}", cmd.command, cmd.help);
                            println!("Arguments:");
                            for arg in cmd.args {
                                println!(
                                    "  {}: {} (default: {})",
                                    arg.name,
                                    arg.description,
                                    arg.default.unwrap_or("None")
                                );
                            }
                            return Ok("".to_string());
                        }
                    }
                    return Err(format!("Command {} not found", cmdname));
                }
                Err(_) => {
                    for cmd in COMMANDS.iter() {
                        let mut args = "".to_string();
                        for arg in cmd.args {
                            let default = match arg.default {
                                Some(value) => value.to_string(),
                                None => "None".to_string(),
                            };
                            args.push_str(&format!(" [{}: {} = {}]", arg.name, arg.type_, default));
                        }
                        // replace line breaks with nothing
                        args = args.replace('\n', "");
                        println!("  {} | {} {} -> {}", cmd.command, cmd.short, args, cmd.help);
                    }
                    println!("Arguments are represented like: [name: type = default]\n\n");
                }
            }
            Ok("".to_string())
        },
    },
    Command {
        command: "print",
        short: "p",
        help: "Prints the current board",
        args: &[],
        stdout: true,
        repl_only: false,
        action: |_cmd, navigator, _args| print_level_board(navigator, DEFAULT_WIDTH),
    },
    Command {
        command: "printseq",
        short: "ps",
        help: "Prints the current board as a string sequence",
        args: &[],
        stdout: true,
        repl_only: false,
        action: |_cmd, navigator, _args| {
            let board = navigator.current_board();
            let mut seq = String::new();
            for cell in board.cells.iter() {
                seq.push_str(&cell.to_string());
            }
            seq.push('\n');
            Ok(seq)
        },
    },
    Command {
        command: "up",
        short: "u",
        help: "Move up one level",
        stdout: false,
        repl_only: false,
        args: &[],
        action: |_cmd, navigator, _args| match navigator.ascend() {
            Ok(_) => {
                let board_str = print_level_board(navigator, DEFAULT_WIDTH).unwrap();
                Ok(format!("You ascend to the upper board. You see:\n{}", board_str))
            }
            Err(_) => {
                let board_str = print_level_board(navigator, DEFAULT_WIDTH).unwrap();
                Ok(format!("You can't ascend any further.\n{}", board_str))
            }
        },
    },
    Command {
        command: "down",
        short: "d",
        stdout: false,
        repl_only: false,
        help: "Move down one level",
        args: &[CommonArgs::Cell.value()],
        action: |cmd, navigator, args| {
            let args = cmd.argparse(args)?;
            let cell = args.first().unwrap();
            let cell = match cell {
                ArgValue::Int(cell) => *cell,
                _ => unreachable!(),
            };
            match navigator.descend(cell as usize) {
                Ok(_) => {
                    let board_str = print_level_board(navigator, DEFAULT_WIDTH).unwrap();
                    Ok(format!("You resolved cell {}. You see:\n{}", cell, board_str))
                }
                Err(msg) => Err(msg),
            }
        },
    },
    Command {
        command: "board",
        short: "b",
        help: "Set the current board",
        args: &[CommonArgs::Board.value()],
        stdout: false,
        repl_only: false,
        action: |cmd, navigator, args| {
            let args = cmd.argparse(args)?;
            let board_arg = args.first().unwrap();
            let board_arg = match board_arg {
                ArgValue::String(board) => board,
                _ => unreachable!(),
            };
            match navigator.set_board(board_arg) {
                Ok(_) => {
                    let board_str = print_level_board(navigator, DEFAULT_WIDTH).unwrap();
                    Ok(format!("You set the board. You see:\n{}", board_str))
                }
                Err(msg) => Err(msg),
            }
        },
    },
    Command {
        command: "save",
        short: "s",
        help: "Saves the current situation to a file",
        args: &[CommonArgs::Filename.value()],
        stdout: false,
        repl_only: false,
        action: |cmd, _navigator, args| {
            let args = cmd.argparse(args)?;
            let filename = args.first().unwrap();
            let filename = match filename {
                ArgValue::String(filename) => filename,
                _ => unreachable!(),
            };
            Err("To be done".to_string())
        },
    },
    Command {
        command: "load",
        short: "l",
        help: "Load a saved situation from a file",
        args: &[CommonArgs::Filename.value()],
        stdout: false,
        repl_only: false,
        action: |cmd, _navigator, args| {
            let args = cmd.argparse(args)?;
            let filename = args.first().unwrap();
            let filename = match filename {
                ArgValue::String(filename) => filename,
                _ => unreachable!(),
            };
            Err("To be done".to_string())
        },
    },
    Command {
        command: "export",
        short: "e",
        help: "Export the current repl history as a hanb script",
        args: &[CommonArgs::ScriptName.value()],
        stdout: false,
        repl_only: true,
        action: |_cmd, _navigator, args| {
            let arglist = args.split(' ').collect::<Vec<&str>>();
            let filename = arglist.first().unwrap().to_string();

            // Script is all strings in arguments after file name
            let script = arglist[1..].join(" ");
            let mut file = File::create(&filename);
            match file {
                Ok(ref mut file) => match file.write_all(script.as_bytes()) {
                    Ok(_) => Ok(format!("Script {} saved", filename)),
                    Err(err) => Err(format!("Error saving script: {}", err)),
                },
                Err(e) => Err(format!("Error saving script: {}", e)),
            }
        },
    },
    Command {
        command: "import",
        short: "i",
        help: "Import a hanb script and execute it into this repl",
        args: &[CommonArgs::ScriptName.value()],
        stdout: false,
        repl_only: true,
        action: |_cmd, _navigator, _args| Err("Implement this from repl".to_string()),
    },
];
