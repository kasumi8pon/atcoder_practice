n = gets.chomp.to_i

divide_counts = []

1.upto(n) do |i|
  count = 0
  while i.even?
    count += 1
    i /= 2
  end
  divide_counts.push(count)
end

puts divide_counts.index(divide_counts.max) + 1