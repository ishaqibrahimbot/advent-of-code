def get_indices(grid)
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

    [row_indices, column_indices]
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
    puts "\nSolving: #{filename}"

    file = File.open(filename)
    grid = file.readlines.map { |line| line.chomp.split('')  }

    row_indices, col_indices = get_indices(grid)

    galaxies = find_galaxies(grid)

    puts "Galaxies: #{galaxies.length}"

    combinations = galaxies.combination(2).to_a

    puts "Combinations: #{combinations.length}"

    factor = 999999
    total_steps = 0

    combinations.each_with_index do |pair, index|
        d_zero = manhattan_distance(pair[0], pair[1])

        x1, y1 = pair[0]
        x2, y2 = pair[1]

        # find num_rows and num_columns between these pairs
        num_rows_in_between = row_indices.select { |row_index| row_index > [y1, y2].min && row_index < [y1, y2].max }.length
        num_cols_in_between = col_indices.select { |col_index| col_index > [x1, x2].min && col_index < [x1, x2].max }.length

        d_factor = d_zero + (num_rows_in_between * factor) + (num_cols_in_between * factor)

        total_steps += d_factor
    end

    puts "Total steps: #{total_steps}"
end

solve('sample.txt')
solve('input.txt')