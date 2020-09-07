r, c = gets.split.map(&:to_i)
sy, sx = gets.split.map(&:to_i)
gy, gx = gets.split.map(&:to_i)

grid = []
r.times do
  grid << gets.chomp.chars
end

queue = [[sy - 1, sx - 1]]
answer = Array.new(r) { [] }
answer[sy - 1][sx - 1] = 0

def neighborhoods(y, x)
  [[y - 1, x], [y, x - 1], [y, x + 1], [y + 1, x]]
end

until queue.empty?
  current_y, current_x = queue.shift

  neighborhoods(current_y, current_x).each do |y, x|
    next if grid[y][x] == '#' || answer[y][x]

    answer[y][x] = answer[current_y][current_x] + 1
    queue << [y, x]
  end
end

puts answer[gy - 1][gx - 1]
