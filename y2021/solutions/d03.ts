import { Day } from '../Day';

export default class Day3 extends Day {
  private _input = this.input()
    .split('\n')
    .filter((x) => x.length > 0)
    .map((x) => x.split('').map((y) => parseInt(y, 10)));

  private countOnes(input: number[][]): number[] {
    return input.reduce((acc, bitstring) => bitstring.map((x, i) => (acc[i] ?? 0) + x), [] as number[]);
  }

  public partOne(): string {
    const mostOnesBitstring = this.countOnes(this._input).map((x) => Math.round(x / this._input.length));
    const flipped = mostOnesBitstring.map((x) => (x === 1 ? 0 : 1));

    const mostOnes = parseInt(mostOnesBitstring.join(''), 2);
    const leastOnes = parseInt(flipped.join(''), 2);

    return (mostOnes * leastOnes).toString();
  }

  public partTwo(): string {
    const x = parseInt(this.findRating(true).join(''), 2);
    const y = parseInt(this.findRating(false).join(''), 2);
    return (x * y).toString();
  }

  private findRating(equalityStrat = true): number[] {
    let filtered = this._input;
    let index = 0;

    while (true) {
      let occurences = this.countOnes(filtered).map((x) => Math.round(x / filtered.length));
      if (!equalityStrat) {
        occurences = occurences.map((x) => (x === 1 ? 0 : 1));
      }

      const mostCommonValueAtPosition = occurences[index];

      filtered = filtered.filter((x) => x[index] === mostCommonValueAtPosition);

      if (filtered.length <= 1) {
        return filtered[0];
      }

      index += 1;
    }
  }

  private bitCriteriaStrategy(a: number, b: number, least = false): boolean {
    if (least) {
      return a < b;
    }
    return a > b;
  }
}
