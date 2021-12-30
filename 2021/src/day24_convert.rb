#!/usr/bin/env ruby
#
# Used this to convert ASM-ish MONAD code into Rust to work with it myself.

def prefix(s)
  s
end

s = $stdin.read

current_digit = 0

s.split("\n").each do |line|
  line = line.strip

  if line =~ /^inp (.*)$/
    if current_digit > 0
      puts "z"
      puts "}"
    end

    current_digit += 1
    puts "fn digit_#{current_digit}_orig(w: i64, mut z: i64) -> i64 {"
    puts "let mut x: i64 = 0;"
    puts "let mut y: i64 = 0;"
    #puts "#{prefix($1)} = state.next_input();"
  elsif line =~ /^add (.+?) (.+?)$/
    puts "#{prefix($1)} = #{prefix($1)} + #{prefix($2)};"
  elsif line =~ /^div (.+?) (.+?)$/
    puts "#{prefix($1)} = #{prefix($1)} / #{prefix($2)};"
  elsif line =~ /^mul (.+?) (.+?)$/
    puts "#{prefix($1)} = #{prefix($1)} * #{prefix($2)};"
  elsif line =~ /^mod (.+?) (.+?)$/
    puts "#{prefix($1)} = #{prefix($1)} % #{prefix($2)};"
  elsif line =~ /^eql (.+?) (.+?)$/
    puts "#{prefix($1)} = if #{prefix($1)} == #{prefix($2)} { 1 } else { 0 };"
  else
    puts line
  end
end

puts "z"
puts "}"
