import collection.mutable.HashSet
import scala.language.implicitConversions

case class Solution(var remainingUniqueCharacters: Int, var length: Int = 0, val map: Array[Boolean] = Array.ofDim[Boolean](256)):
    private val rand = util.Random.nextInt
    def addCharacter(c: Byte): Boolean =
        if (!map(c) && remainingUniqueCharacters == 0) then
            false
        else
            if (!map(c)) then remainingUniqueCharacters -= 1
            map(c) = true
            length += 1
            true
    override def hashCode(): Int = rand

class Solver(uniqueCharacterMax: Int):
    private var currentSolution = Solution(uniqueCharacterMax)
    private var allSolutions = HashSet[Solution]()
    def addCharacter(c: Byte): Unit =
        allSolutions += Solution(uniqueCharacterMax)
        allSolutions.filterInPlace((a) => {
            val result = a.addCharacter(c)
            if (!result && a.length > currentSolution.length) currentSolution = a
            result
        })
    def terminate(): Int =
        allSolutions.foreach((a) => {
            if (a.length > currentSolution.length) currentSolution = a
        })
        currentSolution.length