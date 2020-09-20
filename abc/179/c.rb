n = gets.to_i

answer = 0

1.upto(10**6) do |a|
  break if a >= n

  1.upto(10**6) do |b|
    break if a * b >= n

    answer += 1
  end
end

puts answer
