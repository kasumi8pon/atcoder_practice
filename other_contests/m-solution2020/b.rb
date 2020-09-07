a, b, c = gets.split.map(&:to_i)
k = gets.to_i

answer = 'No'

k.times do
  if a >= b
    b *= 2
  elsif b >= c
    c *= 2
  end

  if a < b && b < c
    answer = 'Yes'
    break
  end
end

puts answer
