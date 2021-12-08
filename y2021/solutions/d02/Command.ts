import { SubmarineState } from './SubmarineState';

export abstract class Command {
  abstract execute(state: SubmarineState, param: number): SubmarineState;
}
