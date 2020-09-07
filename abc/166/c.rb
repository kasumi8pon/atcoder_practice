n, m = gets.split.map(&:to_i)
heights = gets.split.map(&:to_i)
roads = []
m.times do
  roads << gets.split.map(&:to_i)
end

graph = (1..n).each_with_object({}) { |peak, result| result[peak] = [] }

roads.each_with_object(graph) do |road, result|
  result[road[0]] << road[1]
  result[road[1]] << road[0]
end

good_peaks = graph.select { |from, neighborhoods|
  neighborhoods.empty? ||
    heights[from - 1] > neighborhoods.map { |neighborhood| heights[neighborhood - 1] }.max
}

puts good_peaks.size
