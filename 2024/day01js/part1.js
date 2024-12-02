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

  const sum = arr1.reduce((prev, curr, idx) => {
    return prev + Math.abs(curr - arr2[idx]);
  }, 0);

  console.log(sum);
}

main();
