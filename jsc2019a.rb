m, d = gets.split.map(&:to_i)

all_days = Array.new(d) { |i| i + 1 }

multiplication_days =
  all_days.delete_if { |x| x.to_s[0].to_i < 2 || x.to_s[1].to_i < 2 }
  .delete_if { |x| x.to_s[0].to_i * x.to_s[1].to_i > m }

puts multiplication_days.size
