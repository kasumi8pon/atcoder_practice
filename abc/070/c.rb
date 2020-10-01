n = gets.to_i
times = []
n.times { times << gets.to_i }

answer = 1

times.each do |time|
  answer = answer.lcm(time)
end

puts answer
