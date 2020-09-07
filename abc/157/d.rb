n, m, k = gets.split.map(&:to_i)
friends = []
blocks = []

m.times { friends << gets.split.map(&:to_i) }
n.times { blocks << gets.split.map(&:to_i) }

(1..n).each do |person|
  person