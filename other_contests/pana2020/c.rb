a, b, c = gets.split.map(&:to_i)

require 'complex'

puts (Math.sqrt(a) + Math.sqrt(b) < Math.sqrt(c)) ? 'Yes' : 'No'
