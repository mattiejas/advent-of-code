import java.nio.file.Paths

class Utils {
    fun readFile(fileName: String): List<String> {
        val url = javaClass.getResource(fileName)
        return Paths.get(url.toURI()).toFile().readLines()
    }
}
