n = gets.to_i
p = gets.split.map(&:to_i)
q = gets.split.map(&:to_i)

numbers = (1..n).to_a.map(&:to_s)
perms = numbers.permutation.map(&:join)

a = perms.find_index(p.map(&:to_s).join)
b = perms.find_index(q.map(&:to_s).join)

puts (a - b).abs
