n, m = gets.split.map(&:to_i)

answer =
  if m >= n * 2
    n + (m - n * 2) / 4
  else
    m / 2
  end

puts answer
