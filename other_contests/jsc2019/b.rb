n, k = gets.split.map(&:to_i)
a = gets.split.map(&:to_i)

count = 0
(a.size - 1).times do |j|
  (a.size - 1 - j).times do |i|
    if a[i] > a[i + 1]
      a[i], a[i + 1] = a[i + 1], a[i] 
      count += 1
    end
  end
end

count *= k
a.each { |num| count += a.sort.find_index(num) * (k * (k - 1)/ 2) }

puts count % (10**9 + 7)
