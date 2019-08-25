n = gets.to_i
v = gets.split.map(&:to_i)

puts v.sort.inject { |result, num| (result + num) / 2.0 }
