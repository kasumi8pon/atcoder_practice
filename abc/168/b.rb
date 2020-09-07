k = gets.to_i
s = gets.chomp

answer =
  if s.size > k
    "#{s[0..k - 1]}..."
  else
    s
  end

puts answer
