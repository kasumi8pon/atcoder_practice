n, d = gets.chomp.split(" ").map(&:to_i)

puts n.fdiv(d * 2 + 1).ceil
