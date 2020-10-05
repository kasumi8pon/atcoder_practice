n = gets.to_i
numbers = gets.split.map(&:to_i)

answer = []

n.times do |i|
  if i.even?
    answer.push numbers[i]
  else
    answer.unshift numbers[i]
  end
end

answer.reverse! if n.odd?

puts answer.join(' ')
