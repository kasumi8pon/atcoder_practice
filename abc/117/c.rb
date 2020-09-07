n, m = gets.split.map(&:to_i)
points = gets.split.map(&:to_i)

sorted_points = points.sort

total_distance = sorted_points.last - sorted_points.first
distances = sorted_points.each_cons(2).map { |from, to| to - from }

answer =
  if n == 1
    total_distance
  elsif n > m
    0
  else
    total_distance - distances.sort.reverse[0...n - 1].inject(:+)
  end

puts answer
