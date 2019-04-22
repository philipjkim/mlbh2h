# mlbh2h

Daily MLB Fantasy Head-to-Head Point Viewer

## Usage

```
# Build the binary and alias it
cargo build && alias mlbh2h='./target/debug/mlbh2h'

# Show help messages
mlbh2h -h

# Create a new league for setting up rosters and scoring rules
# ex> mlbh2h new-league -l my_league
mlbh2h new-league -l <LEAGUE_NAME>

# Prints fantasy points for given league and date
# ex> mlbh2h -k $SPORTRADAR_API_KEY -l my_league -d 2019-04-08
mlbh2h -k <SPORTRADAR_API_KEY> -l <LEAGUE_NAME> -d <yyyy-mm-dd>
```

- Output format is CSV, but available output formats will be added more (ex: JSON, tabular).
- Stats data from Sportradar and league settings (scoring & rosters) are stored under `$HOME/.mlbh2h/`.

## How to get a free Sportradar API key

1. Register (or sign in) [Sportradar](https://developer.sportradar.com/).
2. Create a new app [here](https://developer.sportradar.com/apps/myapps). You should check `Issue a new key for MLB Trial v6` for using MLB v6 APIs.
3. If all is done well, you can see the API key [here](https://developer.sportradar.com/apps/mykeys).
