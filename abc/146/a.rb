s = gets.chomp

week = %w(SUN MON TUE WED THU FRI SAT)

puts 7 - week.find_index(s)
