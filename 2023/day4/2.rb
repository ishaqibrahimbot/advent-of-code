filename = "input.txt"

lines = File.readlines(filename)

num_matches_hash = {}
cards_queue = []

lines.each_with_index do |card, index|
    num_matches = 0
    numbers = card.split(":")[1]
    winning_numbers_str = numbers.split("|")[0]
    hand_numbers_str = numbers.split("|")[1]
    
    winning_numbers = winning_numbers_str.split(" ")
    hand_numbers = hand_numbers_str.split(" ")

    hand_numbers.each do |num|
      if winning_numbers.include?(num)
        num_matches = num_matches + 1
    end
end

    cards_queue.push(index + 1)
    num_matches_hash[index + 1] = num_matches
end

total_cards = 0

def get_copies(card, num_matches)
    copies = []
    
    for i in card..card + num_matches - 1
        copies.push(i + 1)
    end

    return copies
end

while cards_queue.length > 0
    card = cards_queue.shift
    num_matches = num_matches_hash[card]

    copies = get_copies(card, num_matches)

    cards_queue.concat(copies)

    total_cards += 1
end

puts total_cards