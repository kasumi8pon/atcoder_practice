n = gets.to_i
points = gets.split.map(&:to_i)

answers = []

answers << points.inject(0) { |result, num| result += num.abs }
answers << Math.sqrt(points.inject(0) { |result, num| result += (num.abs ** 2 ) })
answers << points.map(&:abs).max

puts answers.join(' ')
