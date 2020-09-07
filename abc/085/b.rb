n = gets.to_i
mochis = []
n.times do
  mochis << gets.to_i
end

puts mochis.uniq.size
