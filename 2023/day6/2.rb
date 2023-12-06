filename = "input.txt"

lines = File.readlines(filename)

time = lines[0].split(":")[1].split(" ").join('').to_i()
distance = lines[1].split(":")[1].split(" ").join('').to_i()

record_to_beat = distance

num_ways_to_beat = 0

for i in 0..time do
    speed = i
    running_time = time - i

    distance_traveled = speed * running_time

    if distance_traveled > record_to_beat
        num_ways_to_beat += 1
    end
end

puts num_ways_to_beat