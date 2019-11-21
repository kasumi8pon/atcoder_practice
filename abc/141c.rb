n, k, q = gets.split.map(&:to_i)
count = Hash.new(0)
q.times { count[gets.to_i] += 1 }

answer = []

1.upto(n) do |i|
  minus_point = q - count[i]
  result = (k - minus_point > 0) ? 'Yes' : 'No'
  answer << result
end

puts answer
