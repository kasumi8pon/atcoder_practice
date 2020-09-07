n = gets.to_i
s, t = gets.chomp.split

answer = ''

(0...n).each do |i|
  answer << "#{s[i]}#{t[i]}" 
end

puts answer
