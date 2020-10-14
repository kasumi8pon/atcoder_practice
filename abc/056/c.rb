x = gets.to_i

total_distance = 0
time = 0
answer = nil

until answer
  time += 1
  total_distance += time
  if total_distance >= x
    answer = time
    break
  end
end

puts answer
