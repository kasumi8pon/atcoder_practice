n = gets.to_i
r = gets.to_i

inside_rating = 
  if n >= 10
    r
  else
    100 * (10 - k)
  end

puts inside_rating