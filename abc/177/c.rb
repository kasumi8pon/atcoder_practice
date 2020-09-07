n = gets.to_i
numbers = gets.split.map(&:to_i)

sum = numbers.sum
rest_sum = sum

answer = numbers.map { |num|
  rest_sum -= num
  num * rest_sum
}.sum

puts answer % (10**9 + 7)
