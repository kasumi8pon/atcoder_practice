n, m = gets.split.map(&:to_i)
relations = []
m.times do
  relations << gets.split.map(&:to_i).sort
end

# graph = Array.new(n) { [] }
# relations.each do |a, b|
#   graph[a - 1] << b - 1
#   graph[b - 1] << a - 1
# end

# groups = []

# 0.upto(n - 1) do |leader|
#   next if groups[leader]

#   queue = [leader]

#   until queue.empty?
#     current = queue.shift
#     next if groups[current]

#     graph[current].each do |friend|
#       next if groups[friend]

#       queue << friend
#     end

#     groups[current] = leader
#   end
# end

# puts groups.tally.values.max

union_find = (0...n).to_a

def root(union_find, a)
  return a if union_find[a] == a

  union_find[a] = root(union_find, union_find[a])
end

def unite(union_find, x, y)
  x = root(union_find, x)
  y = root(union_find, y)
  return if x == y

  union_find[x] = y
end

relations.each do |a, b|
  unite(union_find, a - 1, b - 1)
end

normalized_union_find = (0...n).map { |num| root(union_find, union_find[num])}

pp normalized_union_find.tally.values.max

