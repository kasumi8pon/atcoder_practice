n = gets.to_i
pontas = gets.chomp.chars

# left_w_count = (0...n).each_with_object([0]) { |i, result|
#   result << result.last + (pontas[i - 1] == 'W' ? 1 : 0)
# }

# puts left_w_count

# reverse_right_e_count = (n - 2).downto(0).each_with_object([0]) { |i, result|
#   result << result.last + (pontas[i + 1] == 'E' ? 1 : 0)
# }

# puts reverse_right_e_count

count_e = pontas.count('E')

# costs = pontas.each_with_object([]).with_index { |(ponta, result), i|
#   result <<
#     if i == 0
#       [count_e, 0]
#     else
#       [
#         ponta == 'E' ? result.last.first - 1 : result.last.first,
#         pontas[i - 1] == 'W' ? result.last.last + 1 : result.last.last
#       ]
#     end
# }

costs = (0...n).each_with_object([]) { |i, result|
  prev_right_e, prev_left_w = result.last || [count_e, 0]

  result <<
    [
      pontas[i] == 'E' ? prev_right_e - 1 : prev_right_e,
      i.zero? ? 0 : (pontas[i - 1] == 'W' ? prev_left_w + 1 : prev_left_w)
    ]
}

puts costs.map { |left_cost, right_cost| left_cost + right_cost }.min
