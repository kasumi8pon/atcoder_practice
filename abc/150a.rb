k, x= gets.split.map(&:to_i)

answer =
  if 500 * k >= x
    'Yes'
  else
    'No'
  end

puts answer
