
NUMBER_PANEL = [["7", "8", "9"],
                ["4", "5", "6"],
                ["1", "2", "3"],
                ["X", "0", "A"]]

DIRECTION_PANEL = [["X", "^", "A"],
                   ["<", "v", ">"]]


def precompute_best_moves(panel)
  result = {}

  width = panel[0].length
  height = panel.length

  (0...width).each do |from_col|
    (0...height).each do |from_row|
      from_cell = panel.fetch(from_row).fetch(from_col)

      (0...width).each do |to_col|
        (0...height).each do |to_row|
          to_cell = panel.fetch(to_row).fetch(to_col)

          moves = []

          if from_cell == 'X' || to_cell == 'X'
            moves << []
            next
          end

          row_offset = to_row - from_row
          col_offset = to_col - from_col

          # Row first
          begin
            move = []

            if row_offset != 0
              if row_offset < 0
                move << ['^', row_offset.abs]
              else
                move << ['v', row_offset.abs]
              end
            end

            if col_offset != 0
              if col_offset < 0
                move << ['<', col_offset.abs]
              else
                move << ['>', col_offset.abs]
              end
            end

            moves << move
          end

          # Col first
          begin
            move = []

            if col_offset != 0
              if col_offset < 0
                move << ['<', col_offset.abs]
              else
                move << ['>', col_offset.abs]
              end
            end

            if row_offset != 0
              if row_offset < 0
                move << ['^', row_offset.abs]
              else
                move << ['v', row_offset.abs]
              end
            end

            moves << move
          end

          # Walk the path and make sure we never hit an X
          moves.uniq.each do |move|
            row_pos = from_row
            col_pos = from_col

            ignore_move = false

            move.each do |(direction, movement)|
              movement.times do
                if direction == '^'
                  row_pos -= 1
                elsif direction == 'v'
                  row_pos += 1
                elsif direction == '<'
                  col_pos -= 1
                elsif direction == '>'
                  col_pos += 1
                end

                if panel.fetch(row_pos).fetch(col_pos) == 'X'
                  ignore_move = true
                end
              end
            end

            unless ignore_move
              result[from_cell] ||= {}
              result[from_cell][to_cell] ||= []
              result[from_cell][to_cell] << move
            end
          end
        end
      end
    end
  end

  pp result
end

# precompute_best_moves(NUMBER_PANEL)
# precompute_best_moves(DIRECTION_PANEL)


NUMBER_PANEL_MOVES = {"7"=>
                      {"7"=>[[]],
                       "4"=>[[['v', 1]]],
                       "1"=>[[['v', 2]]],
                       "8"=>[[['>', 1]]],
                       "5"=>[[['v', 1], ['>', 1]], [['>', 1], ['v', 1]]],
                       "2"=>[[['v', 2], ['>', 1]], [['>', 1], ['v', 2]]],
                       "0"=>[[['>', 1], ['v', 3]]],
                       "9"=>[[['>', 2]]],
                       "6"=>[[['v', 1], ['>', 2]], [['>', 2], ['v', 1]]],
                       "3"=>[[['v', 2], ['>', 2]], [['>', 2], ['v', 2]]],
                       "A"=>[[['>', 2], ['v', 3]]]},
                      "4"=>
                      {"7"=>[[['^', 1]]],
                       "4"=>[[]],
                       "1"=>[[['v', 1]]],
                       "8"=>[[['^', 1], ['>', 1]], [['>', 1], ['^', 1]]],
                       "5"=>[[['>', 1]]],
                       "2"=>[[['v', 1], ['>', 1]], [['>', 1], ['v', 1]]],
                       "0"=>[[['>', 1], ['v', 2]]],
                       "9"=>[[['^', 1], ['>', 2]], [['>', 2], ['^', 1]]],
                       "6"=>[[['>', 2]]],
                       "3"=>[[['v', 1], ['>', 2]], [['>', 2], ['v', 1]]],
                       "A"=>[[['>', 2], ['v', 2]]]},
                      "1"=>
                      {"7"=>[[['^', 2]]],
                       "4"=>[[['^', 1]]],
                       "1"=>[[]],
                       "8"=>[[['^', 2], ['>', 1]], [['>', 1], ['^', 2]]],
                       "5"=>[[['^', 1], ['>', 1]], [['>', 1], ['^', 1]]],
                       "2"=>[[['>', 1]]],
                       "0"=>[[['>', 1], ['v', 1]]],
                       "9"=>[[['^', 2], ['>', 2]], [['>', 2], ['^', 2]]],
                       "6"=>[[['^', 1], ['>', 2]], [['>', 2], ['^', 1]]],
                       "3"=>[[['>', 2]]],
                       "A"=>[[['>', 2], ['v', 1]]]},
                      "8"=>
                      {"7"=>[[['<', 1]]],
                       "4"=>[[['v', 1], ['<', 1]], [['<', 1], ['v', 1]]],
                       "1"=>[[['v', 2], ['<', 1]], [['<', 1], ['v', 2]]],
                       "8"=>[[]],
                       "5"=>[[['v', 1]]],
                       "2"=>[[['v', 2]]],
                       "0"=>[[['v', 3]]],
                       "9"=>[[['>', 1]]],
                       "6"=>[[['v', 1], ['>', 1]], [['>', 1], ['v', 1]]],
                       "3"=>[[['v', 2], ['>', 1]], [['>', 1], ['v', 2]]],
                       "A"=>[[['v', 3], ['>', 1]], [['>', 1], ['v', 3]]]},
                      "5"=>
                      {"7"=>[[['^', 1], ['<', 1]], [['<', 1], ['^', 1]]],
                       "4"=>[[['<', 1]]],
                       "1"=>[[['v', 1], ['<', 1]], [['<', 1], ['v', 1]]],
                       "8"=>[[['^', 1]]],
                       "5"=>[[]],
                       "2"=>[[['v', 1]]],
                       "0"=>[[['v', 2]]],
                       "9"=>[[['^', 1], ['>', 1]], [['>', 1], ['^', 1]]],
                       "6"=>[[['>', 1]]],
                       "3"=>[[['v', 1], ['>', 1]], [['>', 1], ['v', 1]]],
                       "A"=>[[['v', 2], ['>', 1]], [['>', 1], ['v', 2]]]},
                      "2"=>
                      {"7"=>[[['^', 2], ['<', 1]], [['<', 1], ['^', 2]]],
                       "4"=>[[['^', 1], ['<', 1]], [['<', 1], ['^', 1]]],
                       "1"=>[[['<', 1]]],
                       "8"=>[[['^', 2]]],
                       "5"=>[[['^', 1]]],
                       "2"=>[[]],
                       "0"=>[[['v', 1]]],
                       "9"=>[[['^', 2], ['>', 1]], [['>', 1], ['^', 2]]],
                       "6"=>[[['^', 1], ['>', 1]], [['>', 1], ['^', 1]]],
                       "3"=>[[['>', 1]]],
                       "A"=>[[['v', 1], ['>', 1]], [['>', 1], ['v', 1]]]},
                      "0"=>
                      {"7"=>[[['^', 3], ['<', 1]]],
                       "4"=>[[['^', 2], ['<', 1]]],
                       "1"=>[[['^', 1], ['<', 1]]],
                       "8"=>[[['^', 3]]],
                       "5"=>[[['^', 2]]],
                       "2"=>[[['^', 1]]],
                       "0"=>[[]],
                       "9"=>[[['^', 3], ['>', 1]], [['>', 1], ['^', 3]]],
                       "6"=>[[['^', 2], ['>', 1]], [['>', 1], ['^', 2]]],
                       "3"=>[[['^', 1], ['>', 1]], [['>', 1], ['^', 1]]],
                       "A"=>[[['>', 1]]]},
                      "9"=>
                      {"7"=>[[['<', 2]]],
                       "4"=>[[['v', 1], ['<', 2]], [['<', 2], ['v', 1]]],
                       "1"=>[[['v', 2], ['<', 2]], [['<', 2], ['v', 2]]],
                       "8"=>[[['<', 1]]],
                       "5"=>[[['v', 1], ['<', 1]], [['<', 1], ['v', 1]]],
                       "2"=>[[['v', 2], ['<', 1]], [['<', 1], ['v', 2]]],
                       "0"=>[[['v', 3], ['<', 1]], [['<', 1], ['v', 3]]],
                       "9"=>[[]],
                       "6"=>[[['v', 1]]],
                       "3"=>[[['v', 2]]],
                       "A"=>[[['v', 3]]]},
                      "6"=>
                      {"7"=>[[['^', 1], ['<', 2]], [['<', 2], ['^', 1]]],
                       "4"=>[[['<', 2]]],
                       "1"=>[[['v', 1], ['<', 2]], [['<', 2], ['v', 1]]],
                       "8"=>[[['^', 1], ['<', 1]], [['<', 1], ['^', 1]]],
                       "5"=>[[['<', 1]]],
                       "2"=>[[['v', 1], ['<', 1]], [['<', 1], ['v', 1]]],
                       "0"=>[[['v', 2], ['<', 1]], [['<', 1], ['v', 2]]],
                       "9"=>[[['^', 1]]],
                       "6"=>[[]],
                       "3"=>[[['v', 1]]],
                       "A"=>[[['v', 2]]]},
                      "3"=>
                      {"7"=>[[['^', 2], ['<', 2]], [['<', 2], ['^', 2]]],
                       "4"=>[[['^', 1], ['<', 2]], [['<', 2], ['^', 1]]],
                       "1"=>[[['<', 2]]],
                       "8"=>[[['^', 2], ['<', 1]], [['<', 1], ['^', 2]]],
                       "5"=>[[['^', 1], ['<', 1]], [['<', 1], ['^', 1]]],
                       "2"=>[[['<', 1]]],
                       "0"=>[[['v', 1], ['<', 1]], [['<', 1], ['v', 1]]],
                       "9"=>[[['^', 2]]],
                       "6"=>[[['^', 1]]],
                       "3"=>[[]],
                       "A"=>[[['v', 1]]]},
                      "A"=>
                      {"7"=>[[['^', 3], ['<', 2]]],
                       "4"=>[[['^', 2], ['<', 2]]],
                       "1"=>[[['^', 1], ['<', 2]]],
                       "8"=>[[['^', 3], ['<', 1]], [['<', 1], ['^', 3]]],
                       "5"=>[[['^', 2], ['<', 1]], [['<', 1], ['^', 2]]],
                       "2"=>[[['^', 1], ['<', 1]], [['<', 1], ['^', 1]]],
                       "0"=>[[['<', 1]]],
                       "9"=>[[['^', 3]]],
                       "6"=>[[['^', 2]]],
                       "3"=>[[['^', 1]]],
                       "A"=>[[]]}}


DIRECTION_PANEL_MOVES = {"<"=>
                         {"<"=>[[]],
                          "^"=>[[['>', 1], ['^', 1]]],
                          "v"=>[[['>', 1]]],
                          "A"=>[[['>', 2], ['^', 1]]],
                          ">"=>[[['>', 2]]]},
                         "^"=>
                         {"<"=>[[['v', 1], ['<', 1]]],
                          "^"=>[[]],
                          "v"=>[[['v', 1]]],
                          "A"=>[[['>', 1]]],
                          ">"=>[[['v', 1], ['>', 1]], [['>', 1], ['v', 1]]]},
                         "v"=>
                         {"<"=>[[['<', 1]]],
                          "^"=>[[['^', 1]]],
                          "v"=>[[]],
                          "A"=>[[['^', 1], ['>', 1]], [['>', 1], ['^', 1]]],
                          ">"=>[[['>', 1]]]},
                         "A"=>
                         {"<"=>[[['v', 1], ['<', 2]]],
                          "^"=>[[['<', 1]]],
                          "v"=>[[['v', 1], ['<', 1]], [['<', 1], ['v', 1]]],
                          "A"=>[[]],
                          ">"=>[[['v', 1]]]},
                         ">"=>
                         {"<"=>[[['<', 2]]],
                          "^"=>[[['^', 1], ['<', 1]], [['<', 1], ['^', 1]]],
                          "v"=>[[['<', 1]]],
                          "A"=>[[['^', 1]]],
                          ">"=>[[]]}}


Position = Struct.new(:row, :col)

def lowest_cost(code, directional_panel_count)
  current_cell = 'A'

  total_cost = 0

  code.each_char do |digit|
    best_cost = 2**63
    best_move = nil

    NUMBER_PANEL_MOVES.fetch(current_cell).fetch(digit).each do |move|
      cost = lowest_directional_cost(move + [['A', 1]], directional_panel_count)

      if cost < best_cost
        best_cost = cost
        best_move = move
      end
    end

    current_cell = digit

    total_cost += best_cost
  end

  total_cost
end


MEMO = {}

def lowest_directional_cost(desired_move, depth)
  if MEMO[[desired_move, depth]]
    return MEMO[[desired_move, depth]]
  end

  result = if depth == 0
             desired_move.map {|movement| movement[1]}.sum
           else
             current_cell = 'A'

             total_cost = 0

             desired_move.each do |movement|
               best_cost = 2**63
               best_move = nil

               DIRECTION_PANEL_MOVES.fetch(current_cell).fetch(movement[0]).each do |move|
                 cost = lowest_directional_cost(move + [['A', 1]], depth - 1) + (movement[1] - 1)

                 if cost < best_cost
                   best_cost = cost
                   best_move = move
                 end
               end

               total_cost += best_cost

               current_cell = movement[0]
             end

             total_cost
           end

  MEMO[[desired_move, depth]] = result

  result
end

def part1
  codes_to_enter = File.read("input_files/day21.txt").scan(/[0-9A]+/).to_a

  total_complexity = 0

  codes_to_enter.each do |code|
    cost = lowest_cost(code, 2)

    numeric = Integer(code.gsub(/[^0-9]/, ''))

    total_complexity += (cost * numeric)
  end

  puts "Part 1: Total complexity: #{total_complexity}"
end


def part2
  codes_to_enter = File.read("input_files/day21.txt").scan(/[0-9A]+/).to_a

  total_complexity = 0

  codes_to_enter.each do |code|
    cost = lowest_cost(code, 25)

    numeric = Integer(code.gsub(/[^0-9]/, ''))

    total_complexity += (cost * numeric)
  end

  puts "Part 2: Total complexity: #{total_complexity}"
end

part1
part2
