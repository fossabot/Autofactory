object Main:
    def main(args: Array[String]): Unit =
        val solver = new Solver(2)
        "aaabbbccaaabaaaaabbbbbaw".getBytes().foreach(solver.addCharacter)
        println(solver.terminate())
