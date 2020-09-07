n = gets.to_i
titles = []
times = []
n.times do
  title, time = gets.split
  titles << title
  times << time.to_i
end
x = gets.chomp


index = titles.find_index(x) + 1
puts times[index..-1].inject(:+) || 0
