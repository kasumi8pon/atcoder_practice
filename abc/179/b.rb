n = gets.to_i
dices = []
n.times do
  dices << gets.split.map(&:to_i)
end

answer = 'No'

dices.each_cons(3) do |pairs|
  if pairs.map { |pair| pair.uniq.size }.all?(1)
    answer = 'Yes'
    break
  end
end

puts answer
