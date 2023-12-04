puts "Hello, world!"

filename = "./input.txt"

lines = File.readlines(filename)

total_points = 0

lines.each do |card|
    card_points = 0
    numbers = card.split(":")[1]
    winning_numbers_str = numbers.split("|")[0]
    hand_numbers_str = numbers.split("|")[1]
    
    winning_numbers = winning_numbers_str.split(" ")
    hand_numbers = hand_numbers_str.split(" ")

    hand_numbers.each do |num|
      if winning_numbers.include?(num)
        if card_points == 0
          card_points = 1
        else
          card_points = card_points * 2
        end
      end
    end

    total_points = total_points + card_points
end

puts total_points
