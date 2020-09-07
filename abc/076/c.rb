s_dash = gets.chomp
t = gets.chomp

regexp = /#{t.chars.map { |char| "[#{char}?]" }.join}/

strings = s_dash.size.times.map do |pos|
  if s_dash.match(regexp, pos)
    (s_dash[0...pos] + s_dash[pos..-1].sub(regexp, "#{$1}#{t}")).gsub(/\?/, 'a')
  end
end

pp strings
