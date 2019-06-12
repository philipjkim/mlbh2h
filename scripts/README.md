# Scripts

## `roster-parser.js`

Used to get active rosters of your league.

Usage:

1. Save html source code of `https://baseball.fantasysports.yahoo.com/b1/<YOUR_LEAGUE_ID>/startingrosters` page as `rosters.html` in this directory.
2. Run `npm install` (for the first time only).
3. Run `node roster-parser.js`, then roster JSON will be printed.
4. Save printed JSON to `~/.mlbh2h/leagues/<YOUR_LEAGUE_NAME>/roster.json`.
