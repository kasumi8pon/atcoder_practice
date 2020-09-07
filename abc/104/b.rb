s = gets.chomp

puts s[/^A[a-z]+C[a-z]+$/] ? "AC" : "WA"
