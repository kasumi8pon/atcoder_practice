h, w = gets.split.map(&:to_i)
grid = []
h.times { grid << gets.chomp.split('') }

answer = 0

(0...h).each do |i|
  (0...w).each do |j|
    next if grid[i][j] == '#'

    if grid[i][j + 1] == '.'
      answer += 1
    end
    if grid[i + 1] && grid[i + 1][j] == '.'
      answer += 1
    end
  end
end

puts answer
