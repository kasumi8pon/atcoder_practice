s = gets.chomp.split('')
t = gets.chomp.split('')
size = t.size

answers = []

s.each_cons(size) do |chars|
  count = 0
  chars.each.with_index do |char, i|
    count += 1 unless char == t[i]
  end
  answers << count
end

puts answers.min
