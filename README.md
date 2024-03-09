# steamcmd-2fa

A simple tool that automatically generates a 2FA Steam Guard code and then
runs `steamcmd` with that login and the rest of your arguments.

> [!IMPORTANT]
> Please keep your seed/secret VERY safe as people with your seed can easily
> bypass your 2FA by simply generating the Steam Guard codes themselves!

## Installation

Install a pre-built application,
[direct download](https://github.com/WoozyMasta/steamcmd-2fa/releases)
or use curl:

```bash
curl -#SfLo /usr/bin/steamcmd-2fa \
  https://github.com/WoozyMasta/steamcmd-2fa/releases/latest/download/steamcmd-2fa
chmod +x /usr/bin/steamcmd-2fa
```

You can either clone this and build it yourself or use the supplied binaries
for both Linux and Windows.

```bash
git clone https://github.com/WoozyMasta/steamcmd-2fa
cd steamcmd-2fa
cargo build
```

## Usage

```txt
Usage: steamcmd-2fa.exe [OPTIONS] --username <USERNAME> --password <PASSWORD> --secret <SECRET>

Options:
  -u, --username <USERNAME>        Username to login [env: USERNAME=user]
  -p, --password <PASSWORD>        Password to login [env: PASSWORD]
  -s, --secret <SECRET>            2FA shared_secret (base64 encoded) [env: SECRET]
  -c, --code-only                  Generates only TOTP code for authentication
  -a, --args <ARGS>                Arguments to pass to steamcmd [default: "+@ShutdownOnFailedCommand 1 +quit"]
  -b, --before-args <BEFORE_ARGS>  First positional args like +force_install_dir
  -d, --path <PATH>                Path to steamcmd binary, [env: STEAMCMD_BIN_PATH=]
                                   default one of:
                                    - /home/steam/steamcmd/steamcmd.sh
                                    - /usr/bin/steamcmd
                                    - /home/steam/steamcmd
                                   or
                                    - C:\steamcmd\steamcmd.exe
  -h, --help                       Print help
  -V, --version                    Print version
```

For example, instead of running `steamcmd +login USER +quit`, you would run:

```bash
steamcmd-2fa --path /some/path/steamcmd --username USER --password PASSWORD --secret SECRET --args +quit
steamcmd-2fa -u USER -p PASSWORD -s SECRET -b +force_install_dir -a '+app_update 223350 +quit'
USER=user PASSWORD=pass SECRET=secret steamcmd-2fa
```

To simply retrieve a token:

```bash
steamcmd-2fa -s SECRET -c
SECRET=secret steamcmd-2fa -c
```

You can get your 2FA seed by
[various methods](https://github.com/SteamTimeIdler/stidler/wiki/Getting-your-%27shared_secret%27-code-for-use-with-Auto-Restarter-on-Mobile-Authentication).
Your seed here is the `shared_secret`.

> [!NOTE]
> I am not responsible if you somehow manage to lose/get yourself locked out of your account.

## License

[MIT](https://choosealicense.com/licenses/mit/)
