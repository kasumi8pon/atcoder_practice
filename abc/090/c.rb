n, m = gets.split.map(&:to_i)

back_n = n == 1 ? 1 : (n - 2)
back_m = m == 1 ? 1 : (m - 2)

puts back_n * back_m
