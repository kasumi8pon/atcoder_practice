n, k, m = gets.split.map(&:to_i)
a = gets.split.map(&:to_i)

answer = 0
c_score = a.inject(:+)

0.upto(k) do |num|
  if (m * n) -  (c_score + num) <= 0
    answer = num
    break
  end
  answer = -1 if num == k
end

puts answer
