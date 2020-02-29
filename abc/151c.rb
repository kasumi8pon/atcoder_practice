n, m = gets.split.map(&:to_i)
submits = []
m.times do
  task, result = gets.split
  submits << [task.to_i, result]
end

ac = 0
acs = Hash.new {0}
tmp_penalties = Hash.new {0}
penalty = 0
now_task = 0


submits.each do |task, result|
  if result == 'AC'
    if acs[task.to_s.to_sym].zero?
      ac += 1
      acs[task.to_s.to_sym] = 1
      penalty += tmp_penalties[task.to_s.to_sym]
    end
  else
    tmp_penalties[task.to_s.to_sym] += 1
  end
end

puts "#{ac} #{penalty}"
