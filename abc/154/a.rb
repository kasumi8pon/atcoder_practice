s, t = gets.chomp.split
a, b = gets.split.map(&:to_i)
u = gets.chomp

case u
when s
  a -= 1
when t
  b -= 1
end

puts "#{a} #{b}"
