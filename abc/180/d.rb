x, y, a, b = gets.split.map(&:to_i)

experience_point = 0

if (x * a) <= (x + b)
  select_a = 0
  while true
    break if x * a >= (x + b)
    break if x * a >= y
    x = x * a
    select_a += 1
  end
  experience_point += select_a
end

if x + b < y
  select_b = (y - x) / b
  select_b -= 1 if (y - x) % b == 0
  experience_point += select_b
  x += (select_b * b)
end

puts experience_point
