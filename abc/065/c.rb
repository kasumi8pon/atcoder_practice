n, m = gets.split.map(&:to_i)


answer =
  if (n - m).abs > 1
    0
  elsif (n - m).zero?
    (1..n).inject(1) { |result, num| (result * num) % (10 ** 9 + 7) } *
      (1..m).inject(1) { |result, num| (result * num) % (10 ** 9 + 7) } * 2
  else
    (1..n).inject(1) { |result, num| (result * num) % (10 ** 9 + 7) } *
      (1..m).inject(1) { |result, num| (result * num) % (10 ** 9 + 7) }
  end

puts answer % (10 ** 9 + 7)
