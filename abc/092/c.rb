n = gets.to_i
spots = gets.split.map(&:to_i)

spots.unshift 0
spots.push 0

distances = spots.each_cons(2).map { |a, b| (a - b).abs }
sum = distances.sum

answers = (0...n).map { |i|
  sum - (distances[i] + distances[i + 1]) + (spots[i] - spots[i + 2]).abs
}

puts answers
