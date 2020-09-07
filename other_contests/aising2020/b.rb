n = gets.to_i
numbers = gets.split.map(&:to_i)

puts numbers.select.with_index(1) { |num, i| num.odd? && i.odd? }.size
