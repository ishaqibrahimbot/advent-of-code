const fs = require("fs");

const input = fs.readFileSync("./input.txt", "ascii").split("\n");

class Point {
  constructor(x, y, val) {
    this.x = x;
    this.y = y;
    this.value = val;
  }

  addPartNumber(num) {
    if (!this.partNumbers) {
      this.partNumbers = [];
    }

    if (!this.partNumbers.includes(num)) {
      this.partNumbers.push(num);
    }
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

// regex that matches *
const regex = /\*/;

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

const gears = [];

map.forEach((row) => {
  row.forEach((point) => {
    if (point.getValue() === 1 && point.original) {
      gears.push(point);
    }
  });
});

// console.log(
//   map.map((row) => row.map((point) => point.getValue()).join("")).join("\n")
// );

const partNumbers = [];

const pointToPartNumber = {};

const digitRegex = /\d/;
input.forEach((row, y) => {
  for (let x = 0; x < row.length; x++) {
    const char = row[x];
    if (digitRegex.test(char)) {
      // assum max length 3
      if (x + 1 < row.length && digitRegex.test(row[x + 1])) {
        if (x + 2 < row.length && digitRegex.test(row[x + 2])) {
          const num = parseInt(row.slice(x, x + 3));
          partNumbers.push(num);
          pointToPartNumber[`${x},${y}`] = num;
          pointToPartNumber[`${x + 1},${y}`] = num;
          pointToPartNumber[`${x + 2},${y}`] = num;
          x += 2;
        } else {
          const num = parseInt(row.slice(x, x + 2));
          partNumbers.push(parseInt(row.slice(x, x + 2)));
          pointToPartNumber[`${x},${y}`] = num;
          pointToPartNumber[`${x + 1},${y}`] = num;
          x += 1;
        }
      } else {
        const num = parseInt(row.slice(x, x + 1));
        partNumbers.push(parseInt(row.slice(x, x + 1)));
        pointToPartNumber[`${x},${y}`] = num;
      }
    }
  }
});

gears.forEach((gear) => {
  const [x, y] = gear.getPosition();

  const above = `${x},${y - 1}`;
  const below = `${x},${y + 1}`;
  const left = `${x - 1},${y}`;
  const right = `${x + 1},${y}`;
  const topLeft = `${x - 1},${y - 1}`;
  const topRight = `${x + 1},${y - 1}`;
  const bottomLeft = `${x - 1},${y + 1}`;
  const bottomRight = `${x + 1},${y + 1}`;

  if (pointToPartNumber[above]) {
    gear.addPartNumber(pointToPartNumber[above]);
  }
  if (pointToPartNumber[below]) {
    gear.addPartNumber(pointToPartNumber[below]);
  }
  if (pointToPartNumber[left]) {
    gear.addPartNumber(pointToPartNumber[left]);
  }
  if (pointToPartNumber[right]) {
    gear.addPartNumber(pointToPartNumber[right]);
  }
  if (pointToPartNumber[topLeft]) {
    gear.addPartNumber(pointToPartNumber[topLeft]);
  }
  if (pointToPartNumber[topRight]) {
    gear.addPartNumber(pointToPartNumber[topRight]);
  }
  if (pointToPartNumber[bottomLeft]) {
    gear.addPartNumber(pointToPartNumber[bottomLeft]);
  }
  if (pointToPartNumber[bottomRight]) {
    gear.addPartNumber(pointToPartNumber[bottomRight]);
  }
});

console.log(
  gears
    .filter((gear) => gear.partNumbers.length === 2)
    .reduce((acc, gear) => acc + gear.partNumbers[0] * gear.partNumbers[1], 0)
);
