n = gets.to_i
a = gets.split.map(&:to_i)

inverse_a = a.map { |number| 1.0 / number}

puts 1 / inverse_a.inject(:+)
