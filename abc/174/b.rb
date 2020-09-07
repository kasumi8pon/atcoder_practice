n, d = gets.split.map(&:to_i)
points = []
n.times do
  points << gets.split.map(&:to_i)
end

puts points.select { |x, y| Math.sqrt(x**2 + y**2) <= d }.count
