n = gets.to_i
x = gets.split.map(&:to_i)

count = 0

(1...n).each do |i|
  count += (1.0 / i) * 1 + ((i - 1.0) / i) * 2
end

puts count * (1...n).inject(:*)
