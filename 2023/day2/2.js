const fs = require("fs");

const input = fs.readFileSync("./input.txt", "utf8").split("\n");

const gameIdRegex = /Game (\d+)/;
const colorsRegex = /(\d+) (red|blue|green)/g;

const games = [];

input.forEach((line) => {
  const gameId = line.match(gameIdRegex);
  const game = {
    gameId: gameId[1],
  };

  const rounds = line.split(":")[1].split(";");

  const roundColors = [];

  rounds.forEach((round) => {
    const colors = Array.from(round.matchAll(colorsRegex), (element) => ({
      [element[2]]: element[1],
    })).reduce((acc, curr) => ({ ...acc, ...curr }), {});
    roundColors.push(colors);
  });

  game.rounds = roundColors;
  games.push(game);
});

const powers = [];

games.forEach((game) => {
  const max = game.rounds.reduce(
    (prev, next) => {
      return {
        red: Math.max(prev.red, next.red ?? 0),
        green: Math.max(prev.green, next.green ?? 0),
        blue: Math.max(prev.blue, next.blue ?? 0),
      };
    },
    { red: 0, green: 0, blue: 0 }
  );

  const power = max.red * max.green * max.blue;
  powers.push(power);
});

console.log(powers.reduce((acc, curr) => acc + curr, 0));
