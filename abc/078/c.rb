n, m = gets.split.map(&:to_i)

puts (1900 * m + 100 * (n - m)) * 2 ** m
