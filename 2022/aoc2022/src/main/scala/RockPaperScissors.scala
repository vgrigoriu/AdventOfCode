object RockPaperScissors extends Puzzle, Input:
    def solve(): Int =
        val input = readInput()
        input.map(scoreForRound).sum
    
    private def scoreForRound(round: String): Int =
        // The score for a single round is the score for the shape you selected
        // (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the
        // outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
        //
        // A for Rock, B for Paper, and C for Scissors.
        // X for Rock, Y for Paper, and Z for Scissors.
        round match
            case "A X" => 1 + 3
            case "B X" => 1 + 0
            case "C X" => 1 + 6
            case "A Y" => 2 + 6
            case "B Y" => 2 + 3
            case "C Y" => 2 + 0
            case "A Z" => 3 + 0
            case "B Z" => 3 + 6
            case "C Z" => 3 + 3

object RockPaperScissors2 extends Puzzle, Input:
    def solve(): Int =
        val input = readInput()
        input.map(scoreForRound).sum
    
    private def scoreForRound(round: String): Int =
        // The score for a single round is the score for the shape you selected
        // (1 for Rock, 2 for Paper, and 3 for Scissors) plus the score for the
        // outcome of the round (0 if you lost, 3 if the round was a draw, and 6 if you won).
        //
        // A for Rock, B for Paper, and C for Scissors.
        // X means you need to lose, Y means you need to end the round in a draw, and Z means you need to win.
        round match
            case "A X" => 0 + 3
            case "B X" => 0 + 1
            case "C X" => 0 + 2
            case "A Y" => 3 + 1
            case "B Y" => 3 + 2
            case "C Y" => 3 + 3
            case "A Z" => 6 + 2
            case "B Z" => 6 + 3
            case "C Z" => 6 + 1