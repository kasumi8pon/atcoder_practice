a, b, h, m = gets.split.map(&:to_i)

time = h * 60 + m
short = time * 0.5
long = m * 6

angle = (long - short).abs
angle = (angle < 360 - angle) ? angle : 360 - angle

answer = (a ** 2) + (b ** 2) - (2 * a * b * Math.cos(angle / 180.0 * Math::PI))

puts Math.sqrt(answer)
