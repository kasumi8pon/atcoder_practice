this_week_contest = gets.chomp

next_week_contest = { 'ABC' => 'ARC', 'ARC' => 'ABC' }[this_week_contest]

puts next_week_contest
