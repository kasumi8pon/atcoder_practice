require "complex"

n, d = gets.chomp.split(" ").map(&:to_i)
x = []
n.times do |n|
  array = gets.split(" ").map(&:to_i).to_a
  x.push(array)
end

count = 0

x.combination(2) do |a, b|
  sum = 0
  d.times do |i|
    sum += (a[i - 1] - b[i - 1]) ** 2
  end
  distance = Math.sqrt(sum)
  count += 1 if distance % 1 == 0
end

puts count
