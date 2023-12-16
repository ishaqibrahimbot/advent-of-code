def find_starting_point(grid)
    start = []
    grid.each_with_index do |row, y|
        row.each_with_index do |cell, x|
            if cell == "S"
                start = [x, y]
            end
        end
    end

    return start
end

pipes = ["|", "-", "J", "F", "L", "7"]

def get_node_at_position(position, grid, parent)
    x, y = position
    if !grid[y] || !grid[y][x]
        return nil
    end

    return Node.new(position, grid[y][x], parent)
end

def get_new_position(position, diff)
    return [position[0] + diff[0], position[1] + diff[1]]
end

def get_valid_neighors(pipe, grid)
    # pipe is a node with position and type
    up = [0, -1]
    down = [0, 1]
    left = [-1, 0]
    right = [1, 0]

    if pipe.type == '|'
        return [get_node_at_position(get_new_position(pipe.position, up), grid, pipe), get_node_at_position(get_new_position(pipe.position, down), grid, pipe)]
    elsif pipe.type == '-'
        return [get_node_at_position(get_new_position(pipe.position, left), grid, pipe), get_node_at_position(get_new_position(pipe.position, right), grid, pipe)]
    elsif pipe.type == 'J'
        return [get_node_at_position(get_new_position(pipe.position, up), grid, pipe), get_node_at_position(get_new_position(pipe.position, left), grid, pipe)]
    elsif pipe.type == 'F'
        return [get_node_at_position(get_new_position(pipe.position, down), grid, pipe), get_node_at_position(get_new_position(pipe.position, right), grid, pipe)]
    elsif pipe.type == 'L'
        return [get_node_at_position(get_new_position(pipe.position, up), grid, pipe), get_node_at_position(get_new_position(pipe.position, right), grid, pipe)]
    elsif pipe.type == '7'
        return [get_node_at_position(get_new_position(pipe.position, down), grid, pipe), get_node_at_position(get_new_position(pipe.position, left), grid, pipe)]
    end
end

class Node
    attr_accessor :position, :parent, :type

    def initialize(position, type, parent = nil)
        @position = position
        @type = type
        @parent = parent
    end

    def ==(other_node)
        position == other_node.position
    end
end

def solve(filename, s_type)
    puts "Solving #{filename}"

    file = File.open(filename)
    grid = file.readlines.map() { |line| line.chomp().split("") }

    starting_position = find_starting_point(grid)
    
    grid[starting_position[1]][starting_position[0]] = s_type
    start = Node.new(starting_position, s_type)
    open_set = [start]
    closed_set = []

    i = 0    
    while true
        current = open_set.shift

        if (current == start && i > 0)
            break
        end

        neighbors = get_valid_neighors(current, grid)

        neighbors.each do |neighbor|
            if neighbor && !closed_set.include?(neighbor)
                open_set.push(neighbor)
            end
        end

        closed_set.push(current)
        i += 1

        if open_set.length == 0
            break
        end
    end

    steps = 1

    while current.parent != start
        current = current.parent
        steps += 1
    end

    puts "Steps: #{steps}"
end


solve("sample.txt", "F")
solve("sample-2.txt", "F")
solve("input.txt", "|")