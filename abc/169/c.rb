a, b = gets.split

require 'bigdecimal'

puts (BigDecimal(a) * BigDecimal(b)).floor
