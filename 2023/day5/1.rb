filename = "input.txt"

lines = File.readlines(filename)

seeds = lines[0].split(":")[1].split(" ")

locations = []


seeds.each do |seed|
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
end

puts locations.min