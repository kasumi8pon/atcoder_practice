s = gets.chomp

answer = 'Yes'

0.upto(s.size - 1) do |i|
  if i.even?
    if s[i] == 'L'
      answer = 'No'
      break
    end
  else
    if s[i] == 'R'
      answer = 'No'
      break
    end
  end
end

puts answer
