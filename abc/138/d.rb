n, q = gets.split.map(&:to_i)
graph = Array.new(n) { [] }

(n - 1).times do
  a, b = gets.split.map(&:to_i)
  graph[a - 1] << b - 1
  graph[b - 1] << a - 1
end

counts = Array.new(n) { 0 }
q.times do
  p, x = gets.split.map(&:to_i)
  counts[p - 1] += x
end

queue = [0]
answer = [counts[0]]

until queue.empty?
  current = queue.shift
  graph[current].each do |child|
    next if answer[child]

    answer[child] = answer[current] + counts[child]
    queue << child
  end
end

puts answer.join(' ')
