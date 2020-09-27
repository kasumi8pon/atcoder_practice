n = gets.to_i
names = []
n.times do
  names << gets.chomp
end

name_tally = Hash.new { 0 }

names.each_with_object(name_tally) do |name, tally|
  next unless name.start_with?(/M|A|R|C|H/)
  tally[name[0]] += 1
end

answer = 0

%w(M A R C H).combination(3) { |a, b, c|
  answer += name_tally[a] * name_tally[b] * name_tally[c]
}

puts answer
