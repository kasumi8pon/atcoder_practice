w, h, x, y = gets.split("").map(&:to_i)

area = w * h
half_area = area.fdiv(2)

cut = (y == h.fdiv(2) && x == w.fdiv(2)) ? 1 : 0

puts "#{half_area} #{cut}"
