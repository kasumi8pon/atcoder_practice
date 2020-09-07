n = gets.to_i

answer = 0
num = 10

if n.even?
  while true
    count = n / num
    if count == 0
      break
    else
      answer += count
    end
    num *= 5
  end
end

puts answer
