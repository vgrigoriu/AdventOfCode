using System;
using System.Security.Cryptography;
using System.Text;

namespace AdventOfCode04
{
    public class Program
    {
        public static void Main(string[] args)
        {
            var n = 1;
            var md5 = MD5.Create();
            while (true)
            {
                var input = $"ckczppom{n}";
                var hash = md5.ComputeHash(Encoding.UTF8.GetBytes(input));
                var hex = BitConverter.ToString(hash).Replace("-", "");
                if (hex.StartsWith("000000"))
                {
                    Console.WriteLine(n);
                    break;
                }

                n = n + 1;
            }
        }
    }
}
