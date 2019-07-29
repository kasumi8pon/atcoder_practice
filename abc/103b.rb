s = gets.chomp.to_s
t = gets.chomp.to_s

class RotatableString < String
  attr_accessor :string

  def initialize(string)
    @string = (string)
  end

  def rotate!
    @string = @string[-1] + @string[0..-2]
  end
end

answer = "No"
rotate_string = RotatableString.new(s)

s.length.times do
  if rotate_string.rotate! == t
    answer = "Yes"
    break
  end
end

puts answer