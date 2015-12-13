using System;

namespace AdventOfCode06
{
    public class Program
    {
        public static void Main()
        {
            var lights = new LightsGrid(7, 15);

            lights.TurnOn(2, 3, 0, 3);

            lights.TurnOff(0, 2, 2, 10);

            lights.Toggle(0, 6, 0, 14);

            Console.WriteLine(lights);
        }
    }
}
