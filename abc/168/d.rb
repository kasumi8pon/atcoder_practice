n, m = gets.split.map(&:to_i)
graph = Array.new(n) { [] }

m.times do
  a, b = gets.split.map(&:to_i)
  graph[a - 1] << b - 1
  graph[b - 1] << a - 1
end

queue = [0]
answer = [0]

until queue.empty?
  current = queue.shift
  graph[current].each do |to|
    next if answer[to]

    answer[to] = current + 1
    queue << to
  end
end

puts 'Yes'
puts answer[1...n]
