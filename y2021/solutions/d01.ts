import { Day } from '../Day';

export default class Day1 extends Day {
  public partOne(): string {
    const input = this.input();
    console.log(input);
    return input;
  }

  public partTwo(): string {
    throw new Error('Method not implemented.');
  }
}
