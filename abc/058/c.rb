n = gets.to_i
papers = []
n.times { papers << gets.chomp.split('').tally }

counter = papers[0]

(1...n).each do |i|
  ('a'..'z').each do |char|
    counter[char] = [counter[char] || 0, (papers[i][char] || 0)].min
  end
end

puts counter.map { |char, count| char * count }.flatten.sort.join
