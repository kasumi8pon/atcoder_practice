x, k, d = gets.split.map(&:to_i)

first_min_count, first_min = x.abs.divmod(d)
second_min = first_min > 0 ? first_min - d : first_min + d

if first_min_count <= k
  if k.odd? && first_min_count.odd? || k.even? && first_min_count.even?
    puts first_min.abs
  else
    puts second_min.abs
  end
else
  puts [(x - k * d).abs, (x + k * d).abs].min
end
