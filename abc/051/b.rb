k, s = gets.split.map(&:to_i)

answer = 0

(0..k).each do |x|
  (0..k).each do |y|
    z = s - (x + y)
    answer += 1 if z >= 0 && z <= k
  end
end

puts answer
