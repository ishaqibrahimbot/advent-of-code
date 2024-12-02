const fs = require("fs/promises");

async function main() {
  const fileContents = await fs.readFile("./input1.txt", {
    encoding: "utf-8",
  });

  const arr1 = [];
  const arr2 = [];

  fileContents.split("\n").forEach((line) => {
    const parts = line.split("   ");
    arr1.push(parts[0]);
    arr2.push(parts[1]);
  });

  arr1.sort();
  arr2.sort();

  const frequency = new Map();

  arr2.forEach((val) => {
    const currFreq = frequency.get(val);
    if (currFreq) {
      frequency.set(val, currFreq + 1);
    } else {
      frequency.set(val, 1);
    }
  });

  const sum = arr1.reduce((sum, curr) => {
    const freqInArr2 = frequency.get(curr);
    if (freqInArr2) {
      return sum + curr * freqInArr2;
    }
    return sum;
  }, 0);

  console.log(sum);
}

main();
