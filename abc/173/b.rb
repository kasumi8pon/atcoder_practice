n = gets.to_i
judges = []
n.times do
  judges << gets.chomp
end

tally = judges.tally

puts "AC x #{tally['AC'] || 0}"
puts "WA x #{tally['WA'] || 0}"
puts "TLE x #{tally['TLE'] || 0}"
puts "RE x #{tally['RE'] || 0}"
