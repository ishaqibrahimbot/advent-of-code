filename = "input.txt"

lines = File.readlines(filename)

hands = []
hand_to_bid = {}

lines.each do |line|
    hand = line.split(" ")[0]
    bid = line.split(" ")[1]
    hands << hand

    hand_to_bid[hand] = bid
end

cards = ['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4','3', '2']
cards.reverse!

card_powers = {}

cards.each_with_index do |card, index|
    card_powers[card] = index
end

hand_powers = {
    'five_of_a_kind' => 6,
    'four_of_a_kind' => 5,
    'full_house' => 4,
    'three_of_a_kind' => 3,
    'two_pair' => 2,
    'one_pair' => 1,
    'high_card' => 0
}

def get_hand_power(hand, hand_powers)
    unique_cards = hand.split("").uniq
    num_unique_cards = unique_cards.length
    
    if num_unique_cards == 1
        return hand_powers['five_of_a_kind']
    elsif num_unique_cards == 4
        return hand_powers['one_pair']
    elsif num_unique_cards == 5
        return hand_powers['high_card']
    elsif num_unique_cards == 2
        num_of_occurrences_of_first_card = hand.count(unique_cards[0])
        if num_of_occurrences_of_first_card == 1 || num_of_occurrences_of_first_card == 4
            return hand_powers['four_of_a_kind']
        else
            return hand_powers['full_house']
        end
    else
        first_card_occurrences = hand.count(unique_cards[0])
        second_card_occurrences = hand.count(unique_cards[1])
        third_card_occurrences = hand.count(unique_cards[2])

        if first_card_occurrences == 3 || second_card_occurrences == 3 || third_card_occurrences == 3
            return hand_powers['three_of_a_kind']
        else
            return hand_powers['two_pair']
        end
    end
end

sorted_hands = hands.sort do |hand_a, hand_b|
    power_of_a = get_hand_power(hand_a, hand_powers)
    power_of_b = get_hand_power(hand_b, hand_powers)
    
    if power_of_a > power_of_b
        1
    elsif power_of_a < power_of_b
        -1
    else
        result = 0
        for i in 0..hand_a.length - 1
            a_card = hand_a[i]
            b_card = hand_b[i]

            if card_powers[a_card] > card_powers[b_card]
                result = 1
                break
            elsif card_powers[a_card] < card_powers[b_card]
                result = -1
                break
            else
                next
            end
        end
        result
    end
end

total_winnings = 0
sorted_hands.each_with_index do |hand, index|
    rank = index + 1
    total_winnings += rank * hand_to_bid[hand].to_i
end

puts total_winnings