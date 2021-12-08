import { Day } from '../Day';

export default class Day1 extends Day {
  private measurements = this.input()
    .split('\n')
    .filter((x) => x.length > 0)
    .map((x) => parseInt(x, 10));

  public partOne(): string {
    return Day1.countIncreasingMeasurements(this.measurements).toString();
  }

  public partTwo(): string {
    const aggregatedMeasurements = Day1.getSlidingWindow(this.measurements, 3);
    return Day1.countIncreasingMeasurements(aggregatedMeasurements).toString();
  }

  private static getSlidingWindow(values: number[], windowSize: number): number[] {
    return values.reduce((acc, curr, index) => {
      if (index + windowSize <= values.length) { // if we have enough values to fill the window
        return [...acc, values.slice(index, index + windowSize).reduce((a, b) => a + b)];
      }
      return acc;
    }, []);
  }

  private static countIncreasingMeasurements(values: number[]): number {
    return values.reduce((acc, curr) => {
      if (curr > acc.previous) {
        return { previous: curr, count: acc.count + 1 };
      }
      return { previous: curr, count: acc.count };
    }, { count: -1, previous: 0 }).count; // -1 to account for the first measurement
  }
}
