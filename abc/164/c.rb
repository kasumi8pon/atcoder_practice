n = gets.to_i
strings = []
n.times do
  strings << gets.chomp
end

puts strings.uniq.size
