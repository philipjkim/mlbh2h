# mlbh2h

Daily MLB Fantasy Head-to-Head Point Viewer

## Usage

```
USAGE:
    mlbh2h [FLAGS] [OPTIONS] [SUBCOMMAND]

FLAGS:
    -a, --all               If set, all FA players are also shown
    -h, --help              Prints help information
    -t, --topn              If set, top 10 * (number of t's) batters/pitchers are shown separately (-ttt for top 30
                            batters/pitchers)
    -V, --version           Prints version information
    -w, --weekly-changes    If set, fantasy points per team for 7 days are shown

OPTIONS:
    -k, --apikey <SPORTRADAR_API_KEY>    Sets sportsradar API key.
                                         Get a free api key at https://developer.sportradar.com/
                                         if you don't have one yet.
                                         Environment variable `SPORTRADAR_API_KEY` should be set
                                         if you don't want to set this option.
                                         The option value precedes env.
    -d, --date <YYYY-MM-DD>              Sets the date for stats [default: 2019-07-09]
    -f, --format <FORMAT>                Sets the output format, available values: pretty, csv [default: pretty]
    -l, --league <LEAGUE_NAME>           Sets the league name for scoring and roster [default: sample]
    -r, --range <RANGE>                  Sets the range for stats (1d, 1w, 2w, 1m, all) [default: 1d]

SUBCOMMANDS:
    help            Prints this message or the help of the given subcommand(s)
    list-leagues    lists previously added leagues
    new-league      adds a new league settings (scoring rules + rosters)
```

Line-by-line examples:

```
# for debugging
RUST_BACKTRACE=1 RUST_LOG=mlbh2h=info cargo run -- -l my_league -d 2019-06-17

# Build the binary and alias it
cargo build --release && alias mlbh2h='./target/release/mlbh2h'

# Show help messages
mlbh2h -h

# Create a new league for setting up rosters and scoring rules
mlbh2h new-league -l my_league

# Prints fantasy points for given league and date
mlbh2h -k $SPORTRADAR_API_KEY -l my_league -d 2019-04-08

# Prints top 10 batters/pitchers (by fantasy points during a month) 
# for given league and date, including FA players
mlbh2h -k $SPORTRADAR_API_KEY -l my_league -d 2019-06-08 -r 1m -a -t
```

- Stats data from Sportradar and league settings (scoring & rosters) are stored under `$HOME/.mlbh2h/`.

## How to get a free Sportradar API key

1. Register (or sign in) [Sportradar](https://developer.sportradar.com/).
2. Create a new app [here](https://developer.sportradar.com/apps/myapps). You should check `Issue a new key for MLB Trial v6` for using MLB v6 APIs.
3. If all is done well, you can see the API key [here](https://developer.sportradar.com/apps/mykeys).
