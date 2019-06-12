const cheerio = require('cheerio')
const fs = require('fs')

const contents = fs.readFileSync('rosters.html', 'utf8')
const $ = cheerio.load(contents)

const teams = $('div.Bd').find('div.Grid-u-1-2.Pend-xl')
let allPlayers = []
teams.each((i, elem) => {
    const team = $(elem).find('p a').first().text()
    const players = $(elem).find('tbody tr').map((i, el) => {
        const position = $(el).find('td.pos').first().text()
        const role = ['SP', 'RP', 'P'].includes(position) ? 'Pitcher' : 'Batter'
        const name = $(el).find('td.player div.Grid-bind-end div.ysf-player-name a.name').first().text()

        return { name, role, team }
    }).get()

    allPlayers = allPlayers.concat(players)
})

console.log(JSON.stringify({ players: allPlayers }, null, 2))
