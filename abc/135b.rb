n = gets.to_i
p = gets.split(" ").map(&:to_i)

unmatch_count = p.map.with_index { |n, i| n == p.sort[i] }.count(false)
puts unmatch_count == 2 || unmatch_count == 0 ? "YES" : "NO"
