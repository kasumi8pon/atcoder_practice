n = gets.to_i
numbers = []
n.times { numbers << gets.split.map(&:to_i) }

answer = 0

numbers.each do |a, b|
  answer += ((b * (b + 1)) / 2) - (((a - 1) * a) / 2)
end

puts answer
