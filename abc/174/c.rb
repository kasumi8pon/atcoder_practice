k = gets.to_i

answer = -1

mod = 0

(1..1000000).each do |n|
  mod = (mod * 10 + 7) % k

  if mod.zero?
    answer = n
    break
  end
end

puts answer
