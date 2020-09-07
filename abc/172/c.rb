n, m, k = gets.split.map(&:to_i)
a_books = gets.split.map(&:to_i)
b_books = gets.split.map(&:to_i)

a_times = (0...n).each_with_object([0]) { |num, result|
  result << result.last + a_books[num]
}

b_times = (0...m).each_with_object([0]) { |num, result|
  result << result.last + b_books[num]
}

sum_counts = [0]
(0..n).each_with_object(sum_counts) { |a_count, result|
  rest_time = k - a_times[a_count]
  next if rest_time < 0

  over = b_times.bsearch_index { |b_time| b_time > rest_time } || m + 1
  result << a_count + over - 1
}

puts sum_counts.max
