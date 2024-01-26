#![deny(missing_docs)]

use clap::{Parser, Subcommand, ValueHint};
use clap_complete::Shell;
use libium::config::structs::ModLoader;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
#[clap(arg_required_else_help = true)]
pub struct Ferium {
    #[clap(subcommand)]
    pub subcommand: SubCommands,
    /// Sets the number of worker threads the tokio runtime will use.
    /// You can also use the environment variable `TOKIO_WORKER_THREADS`.
    #[clap(long, short)]
    pub threads: Option<usize>,
    /// Set a GitHub personal access token for increasing the GitHub API rate limit.
    /// You can also use the environment variable `GITHUB_TOKEN`.
    #[clap(long)]
    pub github_token: Option<String>,
    /// Set a custom CurseForge API key.
    /// You can also use the environment variable `CURSEFORGE_API_KEY`.
    #[clap(long)]
    pub curseforge_api_key: Option<String>,
    /// Set the file to read the config from.
    /// This does not change the `cache` and `tmp` directories.
    /// You can also use the environment variable `CONFIG_FILE`.
    #[clap(long, short)]
    #[clap(value_hint(ValueHint::FilePath))]
    pub config_file: Option<PathBuf>,
}

#[derive(Subcommand)]
pub enum SubCommands {
    /// Add a mod to the profile
    Add {
        /// The identifier of the mod/project/repository
        ///
        /// The Modrinth project ID is specified at the bottom of the left sidebar under 'Technical information'.
        /// You can also use the project slug in the URL.
        /// The CurseForge project ID is specified at the top of the right sidebar under 'About Project'.
        /// The GitHub identifier is the repository's full name, e.g. `gorilla-devs/ferium`.
        identifier: String,
        /// The game version will not be checked for this mod
        #[clap(long)]
        dont_check_game_version: bool,
        /// The mod loader will not be checked for this mod
        #[clap(long)]
        dont_check_mod_loader: bool,
    },
    /// Print shell auto completions for the specified shell
    Complete {
        /// The shell to generate auto completions for
        #[clap(value_enum)]
        shell: Shell,
    },
    /// List all the mods in the profile, and with some their metadata if verbose
    List {
        /// Show additional information about the mod
        #[clap(long, short)]
        verbose: bool,
        /// Output information in markdown format and alphabetical order
        ///
        /// Useful for creating modpack mod lists.
        /// Complements the verbose flag.
        #[clap(long, short)]
        markdown: bool,
    },
    /// Add, configure, delete, switch, list, or upgrade modpacks
    #[clap(arg_required_else_help = true)]
    Modpack {
        #[clap(subcommand)]
        subcommand: ModpackSubCommands,
    },
    /// Create, configure, delete, switch, or list profiles
    #[clap(arg_required_else_help = true)]
    Profile {
        #[clap(subcommand)]
        subcommand: ProfileSubCommands,
    },
    /// Remove mods and repositories from the profile.
    /// Optionally, provide a list of names or IDs of the mods to remove.
    Remove {
        /// List of project IDs or case-insensitive names of mods to remove
        mod_names: Vec<String>,
    },
    /// Download and install the latest compatible version of your mods
    Upgrade,
}

#[derive(Subcommand)]
pub enum ProfileSubCommands {
    /// Configure the current profile's name, Minecraft version, mod loader, and output directory.
    /// Optionally, provide the settings to change as arguments.
    Configure {
        /// The Minecraft version to check compatibility for
        #[clap(long, short = 'v')]
        game_version: Option<String>,
        /// The mod loader to check compatibility for
        #[clap(long, short)]
        #[clap(value_enum)]
        mod_loader: Option<ModLoader>,
        /// The name of the profile
        #[clap(long, short)]
        name: Option<String>,
        /// The directory to output mods to
        #[clap(long, short)]
        #[clap(value_hint(ValueHint::DirPath))]
        output_dir: Option<PathBuf>,
    },
    /// Create a new profile.
    /// Optionally, provide the settings as arguments.
    /// Use the import flag to import mods from another profile.
    Create {
        /// Copy over the mods from an existing profile.
        /// Optionally, provide the name of the profile to import mods from.
        #[clap(long, short)]
        #[allow(clippy::option_option)]
        import: Option<Option<String>>,
        /// The Minecraft version to check compatibility for
        #[clap(long, short = 'v')]
        game_version: Option<String>,
        /// The mod loader to check compatibility for
        #[clap(long, short)]
        #[clap(value_enum)]
        mod_loader: Option<ModLoader>,
        /// The name of the profile
        #[clap(long, short)]
        name: Option<String>,
        /// The directory to output mods to
        #[clap(long, short)]
        #[clap(value_hint(ValueHint::DirPath))]
        output_dir: Option<PathBuf>,
    },
    /// Delete a profile.
    /// Optionally, provide the name of the profile to delete.
    Delete {
        /// The name of the profile to delete
        profile_name: Option<String>,
    },
    /// List all the profiles with their data
    List,
    /// Switch between different profiles.
    /// Optionally, provide the name of the profile to switch to.
    Switch {
        /// The name of the profile to switch to
        profile_name: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ModpackSubCommands {
    /// Add a modpack to the config
    Add {
        /// The identifier of the modpack/project
        ///
        /// The Modrinth project ID is specified at the bottom of the left sidebar under 'Technical information'.
        /// You can also use the project slug for this.
        /// The CurseForge project ID is specified at the top of the right sidebar under 'About Project'.
        identifier: String,
        /// The Minecraft instance directory to install the modpack to
        #[clap(long, short)]
        #[clap(value_hint(ValueHint::DirPath))]
        output_dir: Option<PathBuf>,
        /// Whether to install the modpack's overrides to the output directory.
        /// This will override existing files when upgrading.
        #[clap(long, short)]
        install_overrides: Option<bool>,
    },
    /// Configure the current modpack's output directory and installation of overrides.
    /// Optionally, provide the settings to change as arguments.
    Configure {
        /// The Minecraft instance directory to install the modpack to
        #[clap(long, short)]
        #[clap(value_hint(ValueHint::DirPath))]
        output_dir: Option<PathBuf>,
        /// Whether to install the modpack's overrides to the output directory.
        /// This will override existing files when upgrading.
        #[clap(long, short)]
        install_overrides: Option<bool>,
    },
    /// Delete a modpack.
    /// Optionally, provide the name of the modpack to delete.
    Delete {
        /// The name of the modpack to delete
        modpack_name: Option<String>,
    },
    /// List all the modpacks
    List,
    /// Switch between different modpacks.
    /// Optionally, provide the name of the modpack to switch to.
    Switch {
        /// The name of the modpack to switch to
        modpack_name: Option<String>,
    },
    /// Download and install the latest version of the modpack
    Upgrade,
}
