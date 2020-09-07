n = gets.to_i
b_numbers = gets.split.map(&:to_i)

mid = b_numbers.each_cons(2).map { |x, y| [x, y].min }.inject(:+) || 0

puts b_numbers.first + mid + b_numbers.last
