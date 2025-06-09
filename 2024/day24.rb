assignments = {}

swaps = {
  'z22' => 'gjh',
  'z08' => 'ffj',
  'dwp' => 'kfm',
  'jdr' => 'z31',
}

File.open("input_files/day24.txt").each do |line|
  line.chomp!

  next unless line =~ / -> /

  if line =~ /\A(.+) -> (.+)\z/
    expr = $1
    target = $2

    assignments[target] = expr
  end
end

swaps.each do |(a, b)|
  tmp = assignments.fetch(a)
  assignments[a] = assignments[b]
  assignments[b] = tmp
end

running = true

while running
  running = false
  assignments.keys.each do |target|
    expr = assignments.fetch(target)

    replaced = expr.gsub(/[a-z]{3}/) do |var|
      "(" + assignments.fetch(var) + ")"
    end

    if replaced != expr
      assignments[target] = replaced
      running = true
    end
  end
end

assignments.each do |target, expr|
  puts [target, expr].join("\t")
end
