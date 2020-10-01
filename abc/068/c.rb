n, m = gets.split.map(&:to_i)
ships = []
m.times { ships << gets.split.map(&:to_i) }

reach_n = []
start_1 = []

ships.select { |a, b|
  if a == n
    reach_n << b
  elsif b == n
    reach_n << a
  elsif a == 1
    start_1 << b
  elsif b == 1
    start_1 << a
  end
}

puts (reach_n & start_1).empty? ? 'IMPOSSIBLE' : 'POSSIBLE'
