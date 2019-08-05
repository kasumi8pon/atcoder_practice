n = gets.chomp.to_i

answer = 
  case n.to_s.size
  when 1
    n
  when 2
    9
  when 3
    9 + n - 99
  when 4
    900 + 9
  when 5
    900 + 9 + n - 9999
  when 6
    90000 + 900 + 9
  end

puts answer
