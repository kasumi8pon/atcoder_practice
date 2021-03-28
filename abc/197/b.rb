h, w, x, y = gets.split.map(&:to_i)
s = []
h.times { s << gets.chomp.chars }

visibles = 1

(x - 2).step(0, -1).each do |i|
  break if i < 0
  break if s[i][y - 1] == '#'

  visibles += 1
end

x.step(h - 1, 1).each do |i|
  break if i < 0
  break if s[i][y - 1] == '#'

  visibles += 1
end

(y - 2).step(0, -1).each do |i|
  break if i < 0
  break if s[x - 1][i] == '#'

  visibles += 1
end

y.step(w - 1, 1).each do |i|
  break if i < 0
  break if s[x - 1][i] == '#'

  visibles += 1
end

puts visibles
