def solve(filename)
    puts "Solving #{filename}"
    lines = File.readlines(filename).map { |line| line.chomp }

    next_nums = []
    lines.each do |line|
        container = []
        top_level = line.split(" ").map { |num| num.to_i }

        container.push(top_level)
        while true
            current_level = container[-1]
            if current_level.all? { |num| num == 0 }
                break
            end 
            next_level = []
            current_level.each_with_index do |num, index|
                if index != 0
                    next_level.push(num - current_level[index - 1])
                end
            end
            container.push(next_level)
        end

        # puts container.inspect
        reversed_levels = container.reverse
        next_in_history = reversed_levels.reduce(0) {|acc, level| acc + level[-1]}

        next_nums.push(next_in_history)
    end
    
    puts next_nums.reduce(0) { |acc, num| acc + num }
end

solve('sample.txt')
solve("input.txt")
