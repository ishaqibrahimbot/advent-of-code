filename = "input.txt"

lines = File.readlines(filename)

instructions = lines[0].split("")
instructions.pop

map = {}

for i in 2..lines.length - 1
  source = lines[i].split(" = ")[0]
  destinations = lines[i].split(" = ")[1].gsub('(', '').gsub(')', '').chomp().split(", ")
  map[source] = {
    'L'=> destinations[0],
    'R'=> destinations[1]
  }
end


position = 'AAA'

steps = 0
while position != 'ZZZ'
    current_instruction = instructions.shift
    position = map[position][current_instruction]
    instructions.push(current_instruction)
    steps += 1
end

puts steps