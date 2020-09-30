n = gets.to_i
ps = gets.split.map(&:to_i)

answer = 0

(0...n).each do |i|
  next if ps[i] != i + 1

  answer += 1
  ps[i], ps[i + 1] = ps[i + 1], ps[i]
end

answer += 1 if ps[n - 1] == n

puts answer
