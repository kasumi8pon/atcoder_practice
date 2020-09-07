n = gets.to_i
s = gets.chomp

answer = 'No'

if n % 2 == 0
  half = n/2
  answer = 'Yes' if s[0...half] == s[half..-1]
end

puts answer
