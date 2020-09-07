n, k = gets.split.map(&:to_i)
destinations = gets.split.map(&:to_i)

teleporters = [1]
loop_start = nil
loop_end = nil
arrived = [0]

1.upto(2000000) do |i|
  next_destination = destinations[teleporters.last - 1]
  if arrived[next_destination - 1]
    loop_start = arrived[next_destination - 1]
    loop_end = i - 1
    break
  else
    teleporters << next_destination
    arrived[next_destination - 1] = i
  end
end

answer = nil

if k < loop_start
  answer = teleporters[k]
else
  teleporters.shift(loop_start)
  answer = teleporters[(k - loop_start) % (loop_end - loop_start + 1)]
end

puts answer
