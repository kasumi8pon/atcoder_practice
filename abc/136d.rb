s = gets.chomp
answer = Array.new(s.size, 0)

0.upto(s.size - 1) do |i|
  if (s[i] == "R" && s[i + 1] == "L") || (s[i] == "L" && s[i - 1] == "R")
    answer[i] += 1
  else 
    now_position = i
    next_move = s[i] == "R" ? 1 : -1
    count = 0
    while s[now_position] == s[now_position + next_move]
      count += 1
      now_position += next_move
    end
    count.even? ? answer[i + count * next_move] += 1 : answer[i + (count + 1)* next_move] += 1
  end
end

puts answer.join(" ")
