n, k = gets.split.map(&:to_i)
tally = Hash.new(0)

n.times do
  number, count = gets.split.map(&:to_i)
  tally[number] += count
end

answer = nil

tally.keys.sort.each do |number|
  if k <= tally[number]
    answer = number
    break
  else
    k -= tally[number]
  end
end

puts answer
