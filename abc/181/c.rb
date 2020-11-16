n = gets.to_i
points = []
n.times { points << gets.split.map(&:to_i) }

answer = 'No'

points.combination(3) do |a, b, c|
  if a[0] == b[0] && b[0] == c[0]
    answer = 'Yes'
    break
  elsif a[0] == b[0] || b[0] == c[0]
    next
  else
    tilt1 = (a[1] - b[1]) / (a[0] - b[0]).to_f
    tilt2 = (b[1] - c[1]) / (b[0] - c[0]).to_f

    if tilt1 == tilt2
      answer = 'Yes'
      break
    end
  end
end

puts answer
