n, w = gets.split.map(&:to_i)
persons = []
n.times do
  persons << gets.split.map(&:to_i)
end

imos = Array.new(2 * 10 ** 5 + 1) { 0 }
persons.each do |s, t, p|
  imos[s] += p
  imos[t] -= p
end

cumulative_sum = [0]
imos.each_with_object(cumulative_sum) do |imo, sum|
  sum << sum.last + imo
end

pp imos[0..11]
pp cumulative_sum[0..12]

puts cumulative_sum.max <= w ? 'Yes' : 'No'
