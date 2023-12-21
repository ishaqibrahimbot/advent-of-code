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

class Node
    attr_accessor :position, :parent, :g, :h, :f

    def initialize(position, parent=nil)
        @position = position
        @parent = parent
        @g = 0 # from start
        @h = 0 # heuristic-based till goal
        @f = 0 # g + h
    end
end

def a_star(start, goal, grid)
    open_set = [Node.new(start)]
    closed_set = []

    while open_set.any?
        current_node = open_set.min_by { |node| node.f }

        return retrace_path(current_node) if current_node.position == goal

        open_set.delete(current_node)
        closed_set << current_node

        neighbors(current_node, grid).each do |neighbor_pos|
            next if closed_set.any? { |node| node.position == neighbor_pos }

            neighbor = Node.new(neighbor_pos, current_node)

            tentative_g_score = current_node.g + 1
            next if open_set.any? { |node| node.position == neighbor_pos && node.g <= tentative_g_score }

            neighbor.g = tentative_g_score
            neighbor.h = heuristic(neighbor_pos, goal)
            neighbor.f = neighbor.g + neighbor.h
            open_set << neighbor unless open_set.any? { |node| node.position == neighbor.position }
        end
    end

    return 0
end

def neighbors(node, grid)
    x, y = node.position
    [[x-1, y], [x+1, y], [x, y-1], [x, y+1]].select { |nx, ny| grid[ny] && grid[ny][nx] }
end

def heuristic(pos_a, pos_b)
    x1, y1 = pos_a
    x2, y2 = pos_b

    (x1 - x2).abs + (y1 - y2).abs
end

def retrace_path(node)
    num_steps = -1 # don't want to count the start node

    current_node = node
    while current_node
        num_steps += 1
        current_node = current_node.parent
    end

    num_steps
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
        total_steps += a_star(pair[0], pair[1], grid)
        if index % 1000 == 0
            puts "#{index} / #{combinations.length}"
        end
    end

    puts "Total steps: #{total_steps}"
end

solve('sample.txt')
solve('input.txt')