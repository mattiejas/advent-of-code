import { Command } from './Command';
import { SubmarineState } from './SubmarineState';

export class IncreaseHorizontalCommand extends Command {
  public constructor(private param: number) {
    super();
  }

  execute(state: SubmarineState): SubmarineState {
    return {
      horizontal: state.horizontal + this.param,
      depth: state.depth,
      aim: state.aim,
    };
  }
}
