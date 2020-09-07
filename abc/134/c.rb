n = gets.chomp.to_i
a = []
n.times do
  a.push(gets.chomp.to_i)
end

sort_a = a.sort.reverse

a.each do |i|
  puts sort_a[0] == i ? sort_a[1] : sort_a[0]
end