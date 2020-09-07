n, k = gets.split.map(&:to_i)
prices = gets.split.map(&:to_i)

puts prices.sort[0...k].sum
