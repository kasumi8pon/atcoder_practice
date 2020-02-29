s = gets.chomp

reverse_s = s.reverse

count = 0
s.chars.each_with_index do |char, i|
  count += 1 unless char == reverse_s[i]
  break if i >= s.size / 2 - 1
end

puts count
