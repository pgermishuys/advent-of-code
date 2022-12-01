using Shouldly;
using Xunit;
using Xunit.Abstractions;

namespace Day2
{
    public class Day2Tests
    {
        private ITestOutputHelper _outputHelper;

        public Day2Tests(ITestOutputHelper outputHelper)
        {
            _outputHelper = outputHelper;
        }

        [Fact]
        public async Task Part1Sample()
        {
            var directions = File.ReadLines("sample_directions.txt").ToArray();
            var depth = 0;
            var horizontalPosition = 0;
            var answer = 0;
            foreach (var direction in directions)
            {
                _outputHelper.WriteLine($"direction: {direction}");
                var actionAndDistance = direction.Split(' ');
                var action = actionAndDistance[0];
                var distance = int.Parse(actionAndDistance[1]);
                switch (action)
                {
                    case "forward":
                        horizontalPosition += distance;
                        break;
                    case "up":
                        depth -= distance;
                        break;
                    case "down":
                        depth += distance;
                        break;
                }
            }

            answer = horizontalPosition * depth;

            var expectedHorizontalPosition = 15;
            var expectedDepth = 10;
            var expectedAnswer = 150;

            depth.ShouldBe(expectedDepth);
            horizontalPosition.ShouldBe(expectedHorizontalPosition);
            answer.ShouldBe(expectedAnswer);
        }

        [Fact]
        public async Task Part1()
        {
            var directions = File.ReadLines("day2.txt").ToArray();
            var depth = 0;
            var horizontalPosition = 0;
            var answer = 0;
            var aim = 0;
            foreach (var direction in directions)
            {
                var actionAndDistance = direction.Split(' ');
                var action = actionAndDistance[0];
                var distance = int.Parse(actionAndDistance[1]);
                switch (action)
                {
                    case "forward":
                        horizontalPosition += distance;
                        break;
                    case "up":
                        depth -= distance;
                        break;
                    case "down":
                        depth += distance;
                        break;
                }
            }

            answer = horizontalPosition * depth;

            answer.ShouldBe(1250395);
        }

        [Fact]
        public async Task SamplePart2()
        {
            var directions = File.ReadLines("sample_directions.txt").ToArray();
            var depth = 0;
            var horizontalPosition = 0;
            var answer = 0;
            var aim = 0;
            foreach (var direction in directions)
            {
                var actionAndDistance = direction.Split(' ');
                var action = actionAndDistance[0];
                var distance = int.Parse(actionAndDistance[1]);
                switch (action)
                {
                    case "forward":
                        horizontalPosition += distance;
                        depth += aim * distance;
                        break;
                    case "up":
                        aim -= distance;
                        break;
                    case "down":
                        aim += distance;
                        break;
                }
            }

            answer = horizontalPosition * depth;
            answer.ShouldBe(900);
        }

        [Fact]
        public async Task Part2()
        {
            var directions = File.ReadLines("day2.txt").ToArray();
            var depth = 0;
            var horizontalPosition = 0;
            var answer = 0;
            var aim = 0;
            foreach (var direction in directions)
            {
                var actionAndDistance = direction.Split(' ');
                var action = actionAndDistance[0];
                var distance = int.Parse(actionAndDistance[1]);
                switch (action)
                {
                    case "forward":
                        horizontalPosition += distance;
                        depth += aim * distance;
                        break;
                    case "up":
                        aim -= distance;
                        break;
                    case "down":
                        aim += distance;
                        break;
                }
            }

            answer = horizontalPosition * depth;
            answer.ShouldBe(900);
        }
    }
}