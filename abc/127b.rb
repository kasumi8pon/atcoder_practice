r, d, x2000 = gets.chomp.split(" ").map(&:to_i)

x = []
x[2000] = x2000

2001.upto(2010) do |i|
  x << r * x[i - 1] - d
  puts x[i]
end
