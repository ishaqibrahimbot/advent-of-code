const fs = require("fs");

const input = fs.readFileSync("./input_1.txt", "utf-8").split("\n");

const numbers = [];

input.forEach((line) => {
  const regex = /\d/g;

  const matches = line.match(regex);

  const firstNumber = matches[0];
  const lastNumber = matches[matches.length - 1];

  numbers.push(parseInt(`${firstNumber}${lastNumber}`));
});

console.log(numbers.reduce((acc, curr) => acc + curr, 0));
