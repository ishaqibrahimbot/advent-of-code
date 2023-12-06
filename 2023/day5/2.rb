filename = "input.txt"

lines = File.readlines(filename)

seed_values = lines[0].split(":")[1].split(" ")

seeds = []

seed_start = 0
seed_values.each_with_index do |value, index|
    if index % 2 == 0
        seed_start = value.to_i()
    else
        seed_end = seed_start + value.to_i() - 1
        (seed_start..seed_end).each do |seed|
            seeds.push(seed)
        end
    end
end

locations = []

puts "Num seeds: #{seeds.length}"


seeds.each_with_index do |seed, index|
    skip = false
    seed_temp = seed.to_i()
    lines.each_with_index do |line, index|
        if index > 2
            if line.match(/^[a-z]/)
                skip = false
            end
            if line.match(/^[0-9]/) && skip == false
                destination_start = line.split(" ")[0].to_i()
                source_start = line.split(" ")[1].to_i()
                offset = line.split(" ")[2].to_i()
    
                if seed_temp >= source_start && seed_temp < source_start + offset
                    # it's in this range
    
                    # now find the corresponding destination and set a flag to skip to the next map
                    seed_offset = seed_temp - source_start
                    seed_temp = destination_start + seed_offset
                    skip = true
                end
    
            end
        end
    end

    locations.push(seed_temp)

    if index % 10000 == 0
        puts index
    end
end

puts locations.min