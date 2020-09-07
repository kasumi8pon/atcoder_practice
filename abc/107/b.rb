h, w = gets.split.map(&:to_i)
grid = []
h.times do
  grid << gets.chomp.split('')
end

grid.delete_if { |row| row.all? { |cell| cell == '.' } }
grid = grid.transpose
grid.delete_if { |row| row.all? { |cell| cell == '.' } }
grid = grid.transpose

puts grid.map(&:join)
