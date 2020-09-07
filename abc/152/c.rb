n = gets.to_i
pontas = gets.split.map(&:to_i)

answer = 0
minimum = pontas[0]

pontas.each do |ponta|
  answer += 1 if ponta <= minimum
  minimum = ponta if minimum > ponta
end

puts answer
