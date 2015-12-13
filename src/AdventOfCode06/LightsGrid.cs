using System;
using System.Linq;

namespace AdventOfCode06
{
    public class LightsGrid
    {
        private readonly int columns;
        private readonly int[][] lights;
        private readonly int lines;

        public LightsGrid(int lines, int columns)
        {
            this.lines = lines;
            this.columns = columns;

            lights = new int[lines][];
            for (var line = 0; line < lines; line++)
            {
                lights[line] = new int[columns];
            }
        }

        public int HowManyLightsAreLit()
        {
            return lights.Sum(line => line.Sum());
        }

        public void TurnOn(int startLine, int stopLine, int startColumn, int stopColumn)
        {
            ValidateParams(startLine, stopLine, startColumn, stopColumn);

            for (var line = startLine; line <= stopLine; line++)
            {
                for (var column = startColumn; column <= stopColumn; column++)
                {
                    lights[line][column] = 1;
                }
            }
        }

        public void TurnOff(int startLine, int stopLine, int startColumn, int stopColumn)
        {
            ValidateParams(startLine, stopLine, startColumn, stopColumn);

            for (var line = startLine; line <= stopLine; line++)
            {
                for (var column = startColumn; column <= stopColumn; column++)
                {
                    lights[line][column] = 0;
                }
            }
        }

        public void Toggle(int startLine, int stopLine, int startColumn, int stopColumn)
        {
            ValidateParams(startLine, stopLine, startColumn, stopColumn);

            for (var line = startLine; line <= stopLine; line++)
            {
                for (var column = startColumn; column <= stopColumn; column++)
                {
                    lights[line][column] = (lights[line][column] + 1)%2;
                }
            }
        }

        private void ValidateParams(int startLine, int stopLine, int startColumn, int stopColumn)
        {
            if (startLine < 0 || lines <= startLine)
            {
                throw new ArgumentOutOfRangeException(nameof(startLine));
            }

            if (stopLine < 0 || lines <= stopLine)
            {
                throw new ArgumentOutOfRangeException(nameof(stopLine));
            }

            if (startColumn < 0 || columns <= startColumn)
            {
                throw new ArgumentOutOfRangeException(nameof(startColumn));
            }

            if (stopColumn < 0 || columns <= stopColumn)
            {
                throw new ArgumentOutOfRangeException(nameof(stopColumn));
            }

            if (stopLine < startLine)
            {
                throw new ArgumentOutOfRangeException(nameof(stopLine), "should not be less than startLine");
            }

            if (stopColumn < startColumn)
            {
                throw new ArgumentOutOfRangeException(nameof(stopColumn), "should not be less than startColumn");
            }
        }

        public override string ToString()
        {
            return string.Join(Environment.NewLine, lights.Select(GetLine));
        }

        private static string GetLine(int[] lightsLine)
        {
            return new string(lightsLine.Select(l => l == 0 ? '·' : '▮').ToArray());
        }
    }
}
