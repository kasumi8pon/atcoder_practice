n = gets.to_i

x = (n / 1.08).ceil
puts n == (x * 1.08).floor ? x : ":("
