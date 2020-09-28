n = gets.to_i
numbers = gets.split.map(&:to_i)

answer = numbers.tally.inject(0) do |result, (number, count)|
  result += count < number ? count : (count - number)
end

puts answer
