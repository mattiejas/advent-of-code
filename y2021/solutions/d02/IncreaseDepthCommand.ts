import { Command } from './Command';
import { SubmarineState } from './SubmarineState';

export class IncreaseDepthCommand extends Command {
  public constructor(private param: number) {
    super();
  }

  execute(state: SubmarineState): SubmarineState {
    return {
      horizontal: state.horizontal,
      depth: state.depth + this.param,
      aim: state.aim,
    };
  }
}
