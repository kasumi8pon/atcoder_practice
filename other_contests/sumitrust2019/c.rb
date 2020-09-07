x = gets.to_i

m = x / 100
n = x % 100

puts (n / 5.0).ceil <= m ? 1 : 0
