use std::{env, process::exit};

use clap::{Args, Parser};

/* An argument that acts as a command on it's own, and has no required arguments. */
struct CommandArgument<'a> {
    argument: &'a str,
    command: &'a (dyn Fn() -> i32 + Sync),
}
static COMMAND_ARGUMENTS: &[CommandArgument] = &[CommandArgument {
    argument: "--list-codecs",
    command: &list_codecs,
}];

pub fn parse_args() -> CommandLineArgs {
    let mut args = env::args();

    /* There does not appear to (easily) have an argument override any requirement
     * flags and be able to run a command. So for now we are just manually looking
     * for these special arguments. */
    for carg in COMMAND_ARGUMENTS {
        if args.any(|arg| arg == carg.argument) {
            let ret = (carg.command)();
            exit(ret);
        }
    }

    CommandLineArgs::parse()
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct CommandLineArgs {
    #[command(flatten)]
    pub core: CoreArgs,
    #[command(flatten)]
    pub rename: RenameArgs,

    /* Dummy arguments to populate help text, since we are not using these to
     * trigger their behaviour. */
    /// List all available codecs we can transcode to, along with how to specify
    /// options.
    #[arg(long = "list-codecs", help_heading = "Help commands")]
    list_codecs: bool,
}

#[derive(Args, Debug)]
pub struct CoreArgs {
    /// Source directory to copy files from
    #[arg(short = 's', long = "source")]
    pub source: String,
    /// Destination directory to copy files to
    #[arg(short = 'd', long = "destination")]
    pub dest: String,
    /// Playlist to use to select files, can be specified multiple times. If none
    /// are specified, all files are copied recursively
    #[arg(short = 'p', long = "playlist", id = "PLAYLIST")]
    pub playlists: Vec<String>,
    /// Format allowed at the destination, can be specified multiple times. If
    /// none are specified, no transcoding takes place.
    #[arg(short = 'f', long = "format", id = "FORMAT")]
    pub formats: Vec<String>,
}

#[derive(Args, Debug)]
#[clap(next_help_heading = "File rename options")]
pub struct RenameArgs {
    /// Rename special characters to support more restrictive file-systems
    #[arg(long = "rename-special")]
    pub rename_special: bool,
    /// Character to replace special characters with
    #[arg(long = "replacement-char", default_value_t = '_')]
    pub replacement_char: char,
    /// Characters that should be replaced, if --rename-special is enabled
    #[arg(long = "special-characters", default_value = "\"*:<>?|\\")]
    pub special_characters: String,
}

fn list_codecs() -> i32 {
    println!("TODO");
    0
}
