s = gets.chomp
t = gets.chomp

s_dash = s.chars.sort.join
t_dash = t.chars.sort.reverse.join

puts s_dash < t_dash ? 'Yes' : 'No'
