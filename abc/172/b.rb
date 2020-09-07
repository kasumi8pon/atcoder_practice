s = gets.chomp
t = gets.chomp

answer = 0

s.size.times do |num|
  answer += 1 unless s[num - 1] == t [num - 1]
end

puts answer
