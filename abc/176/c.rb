n = gets.to_i
heights = gets.split.map(&:to_i)

boards = []
min = heights.first

heights.each do |height|
  if height < min
    boards << (min - height)
  else
    min = height
  end
end

puts boards.sum

