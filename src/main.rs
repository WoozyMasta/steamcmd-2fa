use clap::Parser;
use simple_steam_totp::generate;

fn find_default_steamcmd() -> &'static str {
    if cfg!(target_os = "windows") {
        "C:\\steamcmd\\steamcmd.exe"
    } else {
        if (std::path::Path::new("/home/steam/steamcmd/steamcmd.sh")).exists() {
            "/home/steam/steamcmd/steamcmd.sh"
        } else if (std::path::Path::new("/usr/bin/steamcmd")).exists() {
            "/usr/bin/steamcmd"
        } else {
            "/home/steam/steamcmd"
        }
    }
}

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(short, long, env, help = "Username to login")]
    username: String,

    #[clap(
        short,
        long,
        env,
        default_value = "",
        help = "Password to login",
        hide_env_values = true,
        hide_default_value = true
    )]
    password: String,

    #[clap(
        short,
        long,
        env,
        default_value = "",
        help = "2FA shared_secret (base64 encoded)",
        hide_env_values = true,
        hide_default_value = true
    )]
    secret: String,

    #[clap(short, long, help = "Generates only TOTP code for authentication")]
    code_only: bool,

    #[clap(
        short,
        long,
        default_value = "+@ShutdownOnFailedCommand 1 +quit",
        help = "Arguments to pass to steamcmd"
    )]
    args: String,

    #[clap(
        short = 'b',
        long,
        default_value = "",
        help = "First positional args like +force_install_dir",
        hide_default_value = true
    )]
    before_args: String,

    #[clap(
        short = 'd',
        long,
        env = "STEAMCMD_BIN_PATH",
        default_value = find_default_steamcmd(),
        help = "Path to steamcmd binary"
    )]
    path: String,
}

fn main() {
    let args = Args::parse();

    let totp = match generate(&args.secret) {
        Ok(code) => code,
        Err(e) => {
            println!("Failed to generate Steam TOTP code: {}", e);
            std::process::exit(1);
        }
    };

    if args.code_only {
        print!("{}", totp);
        std::process::exit(0);
    } else {
        if args.password == "" {
            println!("Password is required");
            std::process::exit(1);
        }

        if args.username == "" {
            println!("Username is required");
            std::process::exit(1);
        }
    }

    if !std::path::Path::new(&args.path).exists() {
        println!(
            "Steamcmd executable not found at {}. Please specify with --path",
            args.path
        );
        std::process::exit(1);
    }

    let status = std::process::Command::new(&args.path)
        .args(shellwords::split(&args.before_args).unwrap())
        .arg("+login")
        .arg(&args.username)
        .arg(&args.password)
        .arg(&totp)
        .args(shellwords::split(&args.args).unwrap())
        .status();

    std::process::exit(status.unwrap().code().unwrap());
}
