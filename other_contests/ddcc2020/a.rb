x, y = gets.split.map(&:to_i)

answer = 0

answer +=
  case x
  when 3
    100000
  when 2
    200000
  when 1
    300000
  else
    0
  end

answer +=
  case y
  when 3
    100000
  when 2
    200000
  when 1
    300000
  else
    0
  end

answer += 400000 if x == 1 && y == 1

puts answer 
