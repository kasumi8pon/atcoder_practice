n, k = gets.split.map(&:to_i)
snukes_have_treat = []
k.times do
  gets
  snukes_have_treat << gets.split.map(&:to_i)
end

puts n - snukes_have_treat.flatten.uniq.size
