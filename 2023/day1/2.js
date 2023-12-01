const fs = require("fs");

const input = fs.readFileSync("./input_1.txt", "utf-8").split("\n");

const dict = {
  one: "1",
  two: "2",
  three: "3",
  four: "4",
  five: "5",
  six: "6",
  seven: "7",
  eight: "8",
  nine: "9",
};

const numbers = [];

input.forEach((line) => {
  // need to use a lookahead (positive) to catch overlapping combinations --> ?=
  const regex = /(?=(\d|one|two|three|four|five|six|seven|eight|nine))/g;

  const matches = Array.from(line.matchAll(regex), (match) => match[1]);

  let firstNumber = matches[0];
  let lastNumber = matches[matches.length - 1];

  if (firstNumber in dict) {
    firstNumber = dict[firstNumber];
  }

  if (lastNumber in dict) {
    lastNumber = dict[lastNumber];
  }

  numbers.push(parseInt(`${firstNumber}${lastNumber}`));
});

console.log(numbers.reduce((acc, curr) => acc + curr, 0));
