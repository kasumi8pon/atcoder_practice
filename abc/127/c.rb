n, m = gets.chomp.split(" ").map(&:to_i)
l = []
r = []
m.times do
  l_i, r_i = gets.chomp.split(" ").map(&:to_i)
  l << l_i
  r << r_i
end

answer = (r.min > n ? n : r.min) - l.max + 1
puts answer < 0 ? 0 : answer
