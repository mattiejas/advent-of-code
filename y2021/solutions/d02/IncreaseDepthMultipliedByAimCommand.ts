import { Command } from './Command';
import { SubmarineState } from './SubmarineState';

export class IncreaseDepthMultipliedByAimCommand extends Command {
  constructor(private param: number) {
    super();
  }

  execute(state: SubmarineState): SubmarineState {
    return {
      ...state,
      depth: state.depth + this.param * state.aim,
    };
  }
}
