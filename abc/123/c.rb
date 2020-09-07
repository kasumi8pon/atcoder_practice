n = gets.to_i
a = gets.to_i
b = gets.to_i
c = gets.to_i
d = gets.to_i
e = gets.to_i

bottleneck = [a, b, c, d, e].min

bottleneck_count, mod = n.divmod(bottleneck)
bottleneck_count += 1 unless mod.zero?

puts bottleneck_count + 4
