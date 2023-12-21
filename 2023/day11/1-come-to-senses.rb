def expand(grid)
    # find all rows and columns where there are only .

    row_indices = []
    column_indices = []

    grid.each_with_index do |row, i|
        if row.all? { |cell| cell == '.' }
            row_indices << i
        end
    end

    for i in 0..grid[0].length-1
        if grid.map { |row| row[i] }.all? { |cell| cell == '.' }
            column_indices << i
        end
    end

    row_indices.each_with_index do |row_index, i|
        grid.insert(row_index + i, Array.new(grid[0].length, '.'))
    end

    column_indices.each_with_index do |column_index, i|
        grid.each do |row|
            row.insert(column_index + i, '.')
        end
    end

    grid
end

def find_galaxies(grid)
    galaxies = []

    grid.each_with_index do |row, y|
        row.each_with_index do |cell, x|
            if cell == "#"
                galaxies << [x, y]
            end
        end
    end

    galaxies
end

def manhattan_distance(pos_a, pos_b)
    x1, y1 = pos_a
    x2, y2 = pos_b

    (x1 - x2).abs + (y1 - y2).abs
end

def solve(filename)
    puts "Solving: #{filename}"

    file = File.open(filename)
    grid = file.readlines.map { |line| line.chomp.split('')  }

    grid = expand(grid)

    galaxies = find_galaxies(grid)

    puts "Galaxies: #{galaxies.length}"

    combinations = galaxies.combination(2).to_a

    puts "Combinations: #{combinations.length}"

    total_steps = 0

    combinations.each_with_index do |pair, index|
        total_steps += manhattan_distance(pair[0], pair[1])
        if index % 1000 == 0
            puts "#{index} / #{combinations.length}"
        end
    end

    puts "Total steps: #{total_steps}"
end

solve('sample.txt')
solve('input.txt')