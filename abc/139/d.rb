n = gets.to_i

# (1..n).to_a.permutation(n).each_with_object({}) { |nums, hash|
#   hash[nums] = nums.each.with_index.inject(0) { |result, (num, i)| result + (i + 1) % num }
# }

puts (n - 1) * n / 2
