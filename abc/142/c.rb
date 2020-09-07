n = gets.to_i
a = gets.split.map(&:to_i)

hash_a = Hash.new(n)
a.each.with_index(1) do |num, i|
  hash_a[num] = i
end

puts hash_a.sort.to_h.values.join(" ")
