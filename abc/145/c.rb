n = gets.to_i
cities = []
n.times do
  x, y = gets.split.map(&:to_i)
  cities << [x, y]
end

def distance(a, b)
  Math.sqrt((a[0] - b[0]) ** 2 + (a[1] - b[1]) ** 2)
end

total_distance = 
  cities.permutation.to_a.map do |perm|
    (0...n - 1).inject(0) { |d, i| d + distance(perm[i], perm[i + 1]) }
  end

puts total_distance.inject(:+) / total_distance.size
