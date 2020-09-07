n = gets.to_i

puts (1..n).select { |num| num % 3 != 0 && num % 5 != 0 }.sum
