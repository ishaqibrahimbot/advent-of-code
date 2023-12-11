filename = "input.txt"

lines = File.readlines(filename)

instructions = lines[0].chomp().split("")

map = {}

for i in 2..lines.length - 1
  source = lines[i].split(" = ")[0]
  destinations = lines[i].split(" = ")[1].gsub('(', '').gsub(')', '').chomp().split(", ")
  map[source] = {
    'L'=> destinations[0],
    'R'=> destinations[1]
  }
end


starting_positions = map.keys.select { |key| key[2] == 'A' }

steps_for_each = {}

puts "Num instructions: #{instructions.length}"

starting_positions.each do |position|
    placeholder_position = position
    temp_instructions = instructions.dup
    steps = 0
    while placeholder_position[2] != 'Z'
        current_instruction = temp_instructions.shift
        placeholder_position = map[placeholder_position][current_instruction]
        temp_instructions.push(current_instruction)
        steps += 1
    end
    steps_for_each[position] = steps
end


puts steps_for_each.values.reduce(1, :lcm)

puts steps_for_each