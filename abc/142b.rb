n, k = gets.split.map(&:to_i)
h = gets.split.map(&:to_i)

sorted_h = h.sort
index = sorted_h.bsearch_index { |height| height >= k }

puts n - (index || n)
