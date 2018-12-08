using System;
using System.Text.RegularExpressions;

namespace AdventOfCode06
{
    public class Instruction
    {
        private readonly string op;
        private readonly int startLine;
        private readonly int stopLine;
        private readonly int startColumn;
        private readonly int stopColumn;

        public Instruction(string line)
        {
            // turn off 887,131 through 889,670
            // turn on 896,994 through 938,999
            // toggle 401,580 through 493,728
            var regex = new Regex(
                @"(.*) (\d+),(\d+) through (\d+),(\d+)");

            var match = regex.Match(line);
            op = match.Groups[1].Value;
            startLine = int.Parse(match.Groups[2].Value);
            startColumn = int.Parse(match.Groups[3].Value);
            stopLine = int.Parse(match.Groups[4].Value);
            stopColumn = int.Parse(match.Groups[5].Value);
        }

        public void ApplyTo(LightsGrid lights)
        {
            switch (op)
            {
                case "turn on":
                    lights.TurnOn(startLine, stopLine, startColumn, stopColumn);
                    break;
                case "turn off":
                    lights.TurnOff(startLine, stopLine, startColumn, stopColumn);
                    break;
                case "toggle":
                    lights.Toggle(startLine, stopLine, startColumn, stopColumn);
                    break;
                default:
                    throw new InvalidOperationException($"Don't know how to do {op}");
            }
        }
    }
}