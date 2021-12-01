import fs from 'fs';
import path from 'path';

export enum Part {
  ONE,
  TWO,
}

export abstract class Day {
  public constructor(public name: string) {}

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

  protected input(): string {
    return fs.readFileSync(path.join(__dirname, 'solutions', 'inputs', this.name), 'utf8');
  }
}
