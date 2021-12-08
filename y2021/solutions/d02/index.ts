import { Day } from '../../Day';
import { Command } from './Command';
import { IncreaseAimCommand } from './IncreaseAimCommand';
import { IncreaseDepthCommand } from './IncreaseDepthCommand';
import { IncreaseDepthMultipliedByAimCommand } from './IncreaseDepthMultipliedByAimCommand';
import { IncreaseHorizontalCommand } from './IncreaseHorizontalCommand';
import { SubmarineState } from './SubmarineState';

export default class Day2 extends Day {
  private _state: SubmarineState = {
    horizontal: 0,
    depth: 0,
    aim: 0,
  };

  private parsePart(part: 1 | 2): Command[] {
    return this.input(true)
      .split('\n')
      .filter((command) => command.length > 0)
      .map((line) => line.split(' '))
      .map(([command, value]) => {
        if (part === 1) return [this.getCommand(command, parseInt(value, 10))];
        return this.getCommandsWithAim(command, parseInt(value, 10));
      })
      .flat();
  }

  public partOne(): string {
    this._state = { horizontal: 0, depth: 0, aim: 0 };
    this.parsePart(1).forEach((command) => { this._state = command.execute(this._state, 0); });
    return `${this._state.horizontal * this._state.depth}`;
  }

  public partTwo(): string {
    this._state = { horizontal: 0, depth: 0, aim: 0 };
    this.parsePart(2).forEach((command) => { this._state = command.execute(this._state, 0); });
    return `${this._state.horizontal * this._state.depth}`;
  }

  private getCommand(command: string, param: number): Command {
    switch (command) {
      case 'forward':
        return new IncreaseHorizontalCommand(param);
      case 'up':
        return new IncreaseDepthCommand(-param);
      case 'down':
        return new IncreaseDepthCommand(param);
      default:
        throw new Error(`Unknown command ${command}`);
    }
  }

  getCommandsWithAim(command: string, param: number): Command[] {
    switch (command) {
      case 'forward':
        return [
          new IncreaseHorizontalCommand(param),
          new IncreaseDepthMultipliedByAimCommand(param),
        ];
      case 'up':
        return [new IncreaseAimCommand(-param)];
      case 'down':
        return [new IncreaseAimCommand(param)];
      default:
        throw new Error(`Unknown command ${command}`);
    }
  }
}
