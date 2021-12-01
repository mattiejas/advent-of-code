export enum Part {
  ONE,
  TWO,
}

export abstract class Day {
  public abstract partOne(): string;
  public abstract partTwo(): string;

  public run(part: Part): string {
    switch (part) {
      case Part.ONE:
        return this.partOne();
      case Part.TWO:
        return this.partTwo();
      default:
        throw new Error('Invalid part');
    }
  }
}
