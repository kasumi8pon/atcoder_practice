a, p = gets.chomp.split(" ").map(&:to_i)

number_of_pieces = a * 3 + p

number_of_apple_pie = number_of_pieces / 2

puts number_of_apple_pie
