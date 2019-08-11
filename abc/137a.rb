a, b = gets.chomp.split(" ").map(&:to_i)

answers = [ a + b, a - b, a * b]

puts answers.max
