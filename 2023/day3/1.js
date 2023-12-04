const fs = require("fs");

const input = fs.readFileSync("./input.txt", "ascii").split("\n");

class Point {
  constructor(x, y, val) {
    this.x = x;
    this.y = y;
    this.value = val;
  }

  markAsOriginal() {
    this.original = true;
  }

  setValue(val) {
    this.value = val;
  }

  getValue() {
    return this.value;
  }

  getPosition() {
    return [this.x, this.y];
  }
}

const map = [];

// regex that matches anything except a dot and a number
const regex = /[^.0-9]/;

input.forEach((line, y) => {
  let row = [];

  line.split("").forEach((char, x) => {
    if (regex.test(char)) {
      const point = new Point(x, y, 1);
      point.markAsOriginal();
      row.push(point);
    } else {
      row.push(new Point(x, y, 0));
    }
  });

  map.push(row);
});

const getPoint = (x, y) => {
  if (x < 0 || x >= map[0].length || y < 0 || y >= map.length) return null;

  return map[y][x];
};

const switchOn = (point) => {
  if (!point) return;

  point.setValue(1);
};

const switchOnNeighbors = (x, y) => {
  const above = getPoint(x, y - 1);
  const below = getPoint(x, y + 1);
  const left = getPoint(x - 1, y);
  const right = getPoint(x + 1, y);
  const topLeft = getPoint(x - 1, y - 1);
  const topRight = getPoint(x + 1, y - 1);
  const bottomLeft = getPoint(x - 1, y + 1);
  const bottomRight = getPoint(x + 1, y + 1);

  switchOn(above);
  switchOn(below);
  switchOn(left);
  switchOn(right);
  switchOn(topLeft);
  switchOn(topRight);
  switchOn(bottomLeft);
  switchOn(bottomRight);
};

map.forEach((row) => {
  row.forEach((point) => {
    if (point.getValue() === 1 && point.original) {
      switchOnNeighbors(point.x, point.y);
    }
  });
});

console.log(
  map.map((row) => row.map((point) => point.getValue()).join("")).join("\n")
);

const digitRegex = /[0-9]/;

const partNumbers = [];

let memory = "";
let isTouching = false;

input.forEach((row, y) => {
  row.split("").forEach((char, x) => {
    if (digitRegex.test(char)) {
      const point = getPoint(x, y);
      if (point.getValue() == 1) {
        isTouching = true;
      }
      memory += char;
    } else {
      if (memory && isTouching) {
        partNumbers.push(parseInt(memory));
        isTouching = false;
      }
      memory = "";
    }
  });
});

console.log(partNumbers.reduce((a, b) => a + b, 0));
