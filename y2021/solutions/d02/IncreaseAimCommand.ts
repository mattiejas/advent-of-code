import { Command } from './Command';
import { SubmarineState } from './SubmarineState';

export class IncreaseAimCommand extends Command {
  public constructor(private aim: number) {
    super();
  }

  public execute(state: SubmarineState): SubmarineState {
    return { ...state, aim: state.aim + this.aim };
  }
}
