s = gets.chomp.split("")

s.sort!
answer = (s[0] == s[1] && s[2] == s[3] && s[0] != s[3]) ? "Yes" : "No"

puts answer