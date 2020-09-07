x = gets.to_i

answer =
  case x
  when 400..599
    8
  when 600..799
    7
  when 800..999
    6
  when 1000..1199
    5
  when 1200..1399
    4
  when 1400..1599
    3
  when 1600..1799
    2
  when 1800..1999
    1
  end

puts answer
