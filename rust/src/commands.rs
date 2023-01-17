struct CmdArg<'a> {
    name: &'a str,
    description: &'a str,
    default: &'a str,
}

pub struct Cmd<'a> {
    command: &'a str,
    short: &'a str,
    help: &'a str,
    args: &'a [CmdArg<'a>],
}

enum CommonArgs {
    Board,
    Cell,
    Filename,
}

impl CommonArgs {
    const fn value(&self) -> CmdArg {
        match self {
            CommonArgs::Board => CmdArg {
                name: "board",
                description: "Board to use",
                default: "Default board",
            },
            CommonArgs::Cell => CmdArg {
                name: "cell",
                description: "Cell to use",
                default: "Default cell",
            },
            CommonArgs::Filename => CmdArg {
                name: "filename",
                description: "File to use",
                default: "Default file",
            },
        }
    }
}

pub const COMMANDS: &[Cmd] = &[
    Cmd {
        command: "help",
        short: "h",
        help: "Prints this help message",
        args: &[],
    },
    Cmd {
        command: "print",
        short: "p",
        help: "Prints the current board",
        args: &[],
    },
    Cmd {
        command: "up",
        short: "u",
        help: "Move up one level",
        args: &[CommonArgs::Cell.value()],
    },
    Cmd {
        command: "down",
        short: "d",
        help: "Move down one level",
        args: &[CommonArgs::Cell.value()],
    },
    Cmd {
        command: "define",
        short: "df",
        help: "Define a new board",
        args: &[CommonArgs::Board.value()],
    },
    Cmd {
        command: "board",
        short: "b",
        help: "Set the current board",
        args: &[CommonArgs::Board.value()],
    },
    Cmd {
        command: "save",
        short: "s",
        help: "Save the current board",
        args: &[CommonArgs::Filename.value()],
    },
    Cmd {
        command: "load",
        short: "l",
        help: "Load a saved board",
        args: &[CommonArgs::Filename.value()],
    },
];

