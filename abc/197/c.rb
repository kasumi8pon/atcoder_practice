# n = gets.to_i
# a = gets.split.map(&:to_i)

# array = []

# [0, 1].repeated_permutation(n - 1) do |bit|
#   array << [a[0]]
#   bit.each_with_index do |b, i|
#     if b == 0
#       array.last[-1] = array.last[-1] | a[i + 1]
#     else
#       array.last << a[i + 1]
#     end
#   end
# end

# puts array.map { |combi|
#   combi.inject { |result, num| result ^ num }
# }.min
