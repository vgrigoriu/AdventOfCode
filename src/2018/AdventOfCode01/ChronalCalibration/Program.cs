using System;
using System.IO;
using System.Linq;
using System.Threading.Tasks;

namespace ChronalCalibration
{
    class Program
    {
        static async Task Main(string[] args)
        {
            var lines = await File.ReadAllLinesAsync("input.txt");
            var frequencies = from line in lines
                              select ParseInt(line);

            Console.WriteLine(frequencies.Sum());
        }

        private static int ParseInt(string line)
        {
            return int.Parse(line);
        }
    }
}
