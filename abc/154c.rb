n = gets.to_i
a = gets.split.map(&:to_i)

puts a.uniq.size == a.size ? "YES" : "NO"
