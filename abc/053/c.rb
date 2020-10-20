x = gets.to_i

div, mod = x.divmod(11)

answer = div * 2

if mod > 6
  answer += 2
elsif mod > 0
  answer += 1
end

puts answer
