t = gets.chomp

puts t.chars.map { |char| char == '?' ? 'D' : char }.join
