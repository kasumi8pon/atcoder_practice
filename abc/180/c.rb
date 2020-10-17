n = gets.to_i

require 'prime'

if n == 1
  puts 1
else
  puts Prime.prime_division(n).map { |e|
    Array.new(e[1]+ 1).map.with_index { |element, i|
      e[0] ** i
    }
  }.inject{ |p, q| p.product(q) }.map { |a|
    [a].flatten.inject(&:*)
  }.sort
end
