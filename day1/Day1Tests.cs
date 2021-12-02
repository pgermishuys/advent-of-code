using Shouldly;
using Xunit;
using Xunit.Abstractions;

namespace Day1
{

    public class Day1Tests
    {
        private ITestOutputHelper _outputHelper;

        public Day1Tests(ITestOutputHelper outputHelper)
        {
            _outputHelper = outputHelper;
        }

        [Fact]
        public async Task Part1Sample()
        {
            var depths = File.ReadLines("sample_report.txt").ToArray();
            var analysis = new List<int>();
            var currentDepth = int.Parse(depths[0]);
            analysis.Add(0);
            for (int i = 1; i < depths.Length; i++)
            {
                var newDepth = int.Parse(depths[i]);
                if (newDepth > currentDepth)
                {
                    analysis.Add(1);
                }
                else if (newDepth < currentDepth)
                {
                    analysis.Add(-1);
                }
                else
                {
                    analysis.Add(0);
                }

                currentDepth = newDepth;
            }

            //expected result
            /*
                199 (N/A - no previous measurement)
                200 (increased)
                208 (increased)
                210 (increased)
                200 (decreased)
                207 (increased)
                240 (increased)
                269 (increased)
                260 (decreased)
                263 (increased)
            */
            analysis.ToArray().ShouldBe(new[]
            {
                0, 1, 1, 1, -1, 1, 1, 1, -1, 1
            });

            analysis.Count(x => x > 0).ShouldBe(7);
        }

        [Fact]
        public async Task Part1()
        {
            var depths = File.ReadLines("day1.txt").ToArray();
            var analysis = new List<int>();
            var currentDepth = int.Parse(depths[0]);
            analysis.Add(0);
            for (int i = 1; i < depths.Length; i++)
            {
                var newDepth = int.Parse(depths[i]);
                if (newDepth > currentDepth)
                {
                    analysis.Add(1);
                }
                else if (newDepth < currentDepth)
                {
                    analysis.Add(-1);
                }
                else
                {
                    analysis.Add(0);
                }

                currentDepth = newDepth;
            }

            analysis.Count(x => x > 0).ShouldBe(1390);
        }

        [Fact]
        public async Task SamplePart2()
        {
            var depths = File.ReadLines("sample_report.txt").ToArray();
            var analysis = new List<int>();
            var currentDepth = int.Parse(depths[0]) + int.Parse(depths[1]) + int.Parse(depths[2]);
            analysis.Add(0);
            for (int i = 1; i < depths.Length; i++)
            {
                if (i + 2 >= (depths.Length)) break;
                var newDepth = int.Parse(depths[i]) + int.Parse(depths[i + 1]) + int.Parse(depths[i + 2]);
                if (newDepth > currentDepth)
                {
                    analysis.Add(1);
                }
                else if (newDepth < currentDepth)
                {
                    analysis.Add(-1);
                }
                else
                {
                    analysis.Add(0);
                }

                currentDepth = newDepth;
            }

            //expected result
            /*
             * A: 607 (N/A - no previous sum)
               B: 618 (increased)
               C: 618 (no change)
               D: 617 (decreased)
               E: 647 (increased)
               F: 716 (increased)
               G: 769 (increased)
               H: 792 (increased)
            */
            analysis.ToArray().ShouldBe(new[]
            {
                0, 1, 0, -1, 1, 1, 1, 1
            });

            analysis.Count(x => x > 0).ShouldBe(5);
        }

        [Fact]
        public async Task Part2()
        {
            var depths = File.ReadLines("day1.txt").ToArray();
            var analysis = new List<int>();
            var currentDepth = int.Parse(depths[0]) + int.Parse(depths[1]) + int.Parse(depths[2]);
            analysis.Add(0);
            for (int i = 1; i < depths.Length; i++)
            {
                if (i + 2 >= (depths.Length)) break;
                var newDepth = int.Parse(depths[i]) + int.Parse(depths[i + 1]) + int.Parse(depths[i + 2]);
                if (newDepth > currentDepth)
                {
                    analysis.Add(1);
                }
                else if (newDepth < currentDepth)
                {
                    analysis.Add(-1);
                }
                else
                {
                    analysis.Add(0);
                }

                currentDepth = newDepth;
            }
            analysis.Count(x => x > 0).ShouldBe(1457);
        }
    }
}