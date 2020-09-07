n = gets.chomp

puts n.split('').map(&:to_i).sum % 9 == 0 ? 'Yes' : 'No'
