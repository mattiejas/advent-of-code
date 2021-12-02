import { Day } from '../Day';

interface SubmarineState {
  horizontal: number;
  depth: number;
}

abstract class Command {
  abstract execute(state: SubmarineState, param: number): SubmarineState;
}

class IncreaseHorizontalCommand extends Command {
  public constructor(private param: number) {
    super();
  }

  execute(state: SubmarineState): SubmarineState {
    return {
      horizontal: state.horizontal + this.param,
      depth: state.depth,
    };
  }
}

class IncreaseDepthCommand extends Command {
  public constructor(private param: number) {
    super();
  }

  execute(state: SubmarineState): SubmarineState {
    return {
      horizontal: state.horizontal,
      depth: state.depth + this.param,
    };
  }
}

export default class Day2 extends Day {
  private _commands = this.input()
    .split('\n')
    .filter((command) => command.length > 0)
    .map((line) => line.split(' '))
    .map(([command, value]) => this.getCommand(command, parseInt(value, 10)));

  private _state: SubmarineState = {
    horizontal: 0,
    depth: 0,
  };

  public partOne(): string {
    this._state = { horizontal: 0, depth: 0 };
    this._commands.forEach((command) => { this._state = command.execute(this._state, 0); });
    return `${this._state.horizontal * this._state.depth}`;
  }

  public partTwo(): string {
    throw new Error('Method not implemented.');
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
}
