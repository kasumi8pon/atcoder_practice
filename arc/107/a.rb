a, b, c = gets.split.map(&:to_i)

puts [a, b, c].inject(1) { |result, x| result *= (x * (x + 1) / 2) } % 998244353
