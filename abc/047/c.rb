s = gets.chomp.split('')

answer = 0

s.each_cons(2) do |pr, nx|
  answer += 1 if pr != nx
end

puts answer
