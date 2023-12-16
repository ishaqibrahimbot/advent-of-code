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

class Point
    attr_accessor :x, :y, :type

    def initialize(x, y, type = nil)
        @x = x
        @y = y
        @type = type
    end

    def ==(other_point)
        x == other_point.x && y == other_point.y
    end
end

def is_point_inside_polygon(point, polygon)
    counter = 0
    j = polygon.length - 1

    for i in 0..polygon.length - 1
        point_a = polygon[i]
        point_b = polygon[j]

        if point.y > [point_a.y, point_b.y].min
            if point.y <= [point_a.y, point_b.y].max
                if point.x <= [point_a.x, point_b.x].max
                    if point_a.y != point_b.y
                        x_intersection = (point.y - point_a.y) * (point_b.x - point_a.x) / (point_b.y - point_a.y) + point_a.x
                        if point_a.x == point_b.x || point.x <= x_intersection
                            counter += 1
                        end
                    end
                end
            end
        end

        j = i
    end


    if counter % 2 == 0
        return false
    else
        return true
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

        neighbors = get_valid_neighors(current, grid)

        if i == 0
            open_set.push(neighbors[0])
        else
            neighbors.each do |neighbor|
                if i > 0 && neighbor && !closed_set.include?(neighbor)
                    open_set.push(neighbor)
                end
            end
        end

        closed_set.push(current)
        i += 1

        if open_set.length == 0
            break
        end
    end

    polygon = closed_set.filter { |node| node.type != "|" && node.type != "-" }.map {  |node| Point.new(node.position[0], node.position[1]) }
    path = closed_set.map { |node| Point.new(node.position[0], node.position[1]) }
    
    all_points = []

    grid.each_with_index do |row, y|
        row.each_with_index do |cell, x|
            all_points.push(Point.new(x, y))
        end
    end

    points_inside = []
    all_points.each do |point|
        if path.include?(point)
            next
        end

        if is_point_inside_polygon(point, polygon)
            points_inside.push(point)
        end
    end

    puts "Num points: #{points_inside.length}"

end


solve("input.txt", "F")