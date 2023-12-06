filename = "input.txt"

lines = File.readlines(filename)

times = lines[0].split(":")[1].split(" ").map(&:to_i)
distances = lines[1].split(":")[1].split(" ").map(&:to_i)

margin_of_error = 1

times.each_with_index do |time, index|
    record_to_beat = distances[index]

    num_ways_to_beat = 0

    for i in 0..time do
        speed = i
        running_time = time - i

        distance_traveled = speed * running_time

        if distance_traveled > record_to_beat
            num_ways_to_beat += 1
        end
    end

    margin_of_error *= num_ways_to_beat
end

puts margin_of_error