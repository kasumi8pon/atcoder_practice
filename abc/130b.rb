n, x = gets.split(" ").map(&:to_i)
l = gets.split(" ").map(&:to_i)

d = [0]
1.upto(n) do |i|
  d[i] = d[i - 1] + l[i - 1]
end
d.delete_if{ |i| i > x }

puts d.size