import java.nio.file.Paths

object Utils {
    fun readFile(fileName: String): List<String> {
        val url = javaClass.getResource(fileName)
        return Paths.get(url.toURI()).toFile().readLines()
    }

    fun gcd(a: Long, b: Long): Long {
        var x = a
        var y = b
        while (y > 0) {
            val temp = y
            y = x % y // % is remainder
            x = temp
        }
        return x
    }

    fun gcd(vararg input: Long): Long {
        var result = input[0]
        for (i in 1 until input.size) result = gcd(result, input[i])
        return result
    }

    fun lcm(a: Long, b: Long): Long {
        return a * (b / gcd(a, b))
    }

    fun lcm(vararg input: Long): Long {
        var result = input[0]
        for (i in 1 until input.size) result = lcm(result, input[i])
        return result
    }
}
