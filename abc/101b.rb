d, n = gets.split(" ").map(&:to_i)

favorite_number = 
  if n == 100
    100 ** d * (n + 1)
  else
    100 ** d * n
  end

puts favorite_number