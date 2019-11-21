a, b = gets.split.map(&:to_i)

width = a - b * 2
puts width < 0 ? 0 : width
