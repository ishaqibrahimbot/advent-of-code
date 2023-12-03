const fs = require("fs");

const input = fs.readFileSync("./input.txt", "utf8").split("\n");

const MAX_CUBES = {
  red: 12,
  green: 13,
  blue: 14,
};

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

const isGameValid = (game) => {
  let isValid = true;
  game.rounds.forEach((round) => {
    if (
      round.red > MAX_CUBES.red ||
      round.green > MAX_CUBES.green ||
      round.blue > MAX_CUBES.blue
    ) {
      isValid = false;
    }
  });

  return isValid;
};

const validGames = games.filter((game) => isGameValid(game));

console.log(validGames.reduce((acc, curr) => acc + parseInt(curr.gameId), 0));
