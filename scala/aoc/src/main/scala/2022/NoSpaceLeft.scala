import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

object NoSpaceLeft extends IntPuzzle:
    def solve(): Int =
        val input = readInput()
        val fileSystem = FileSystem(Directory("/", ArrayBuffer(), mutable.Map()), List())
        
        // $ cd /
        val Cd = "\\$ cd (.+)".r
        // $ ls
        val Ls = "\\$ ls".r
        // dir a
        val Dir = "dir (.+)".r
        // 14848514 b.txt
        val File = "(\\d+) (.+)".r

        val wholeFs = input.foldLeft(fileSystem)((fs, command) =>
            command match
                case Cd(subDirectory) =>
                    fs.cd(subDirectory)
                case Ls() =>
                    fs
                case Dir(subDirectory) =>
                    fs.addDir(subDirectory)
                case File(size, name) =>
                    fs.addFile(name, size.toInt)
                case _: String =>
                    fs
        )

        val sizeLimit = 100000
        val dirSizes = getAllFoldersWithSize(wholeFs)
        dirSizes.values.filter(_ < sizeLimit).sum

object NoSpaceLeft2 extends IntPuzzle:
    def solve(): Int =
        val input = readInput()
        val fileSystem = FileSystem(Directory("/", ArrayBuffer(), mutable.Map()), List())
        
        // $ cd /
        val Cd = "\\$ cd (.+)".r
        // $ ls
        val Ls = "\\$ ls".r
        // dir a
        val Dir = "dir (.+)".r
        // 14848514 b.txt
        val File = "(\\d+) (.+)".r

        val wholeFs = input.foldLeft(fileSystem)((fs, command) =>
            command match
                case Cd(subDirectory) =>
                    fs.cd(subDirectory)
                case Ls() =>
                    fs
                case Dir(subDirectory) =>
                    fs.addDir(subDirectory)
                case File(size, name) =>
                    fs.addFile(name, size.toInt)
                case _: String =>
                    fs
        )

        val dirSizes = getAllFoldersWithSize(wholeFs)
        val used = dirSizes(wholeFs.root)
        val free = 70000000 - used
        val needToFree = 30000000 - free
        dirSizes.values.filter(needToFree <= _).min

case class File(name: String, size: Int)

def getAllFoldersWithSize(fs: FileSystem): mutable.Map[Directory, Int] =
    var result = mutable.Map[Directory, Int]()
    getSize(fs.root, result)
    result

def getSize(dir: Directory, cache: mutable.Map[Directory, Int]): Int =
    if !cache.contains(dir) then
        val filesSize = dir.files.map(_.size).sum
        val subDirsSize = dir.directories.values.map(d => getSize(d, cache)).sum
        cache(dir) = filesSize + subDirsSize
    
    cache(dir)

case class Directory(name: String, files: ArrayBuffer[File], directories: mutable.Map[String, Directory]):
    def addSubDirectory(name: String): Unit =
        directories.update(name, Directory(name, ArrayBuffer(), mutable.Map()))
    def addFile(name: String, size: Int): Unit =
        files.append(File(name, size))
    def getSubDirectory(path: List[String]): Directory =
        path.foldLeft(this)((currentFolder, subDirectoryName) => currentFolder.directories(subDirectoryName))

case class FileSystem(root: Directory, cwd: List[String]):
    def cd(path: String): FileSystem =
        if path == "/" then
            this.copy(cwd = List())
        else if path == ".." then
            this.copy(cwd = this.cwd.init)
        else
            this.copy(cwd = this.cwd.appended(path))
    
    def addDir(dirName: String): FileSystem =
        val currentDirectory = root.getSubDirectory(cwd)
        currentDirectory.addSubDirectory(dirName)
        this

    def addFile(fileName: String, fileSize: Int): FileSystem =
        val currentDirectory = root.getSubDirectory(cwd)
        currentDirectory.addFile(fileName, fileSize)
        this
