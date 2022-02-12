export interface SaveState<State> {
    restoreSaveState(state: State): void;

    toSaveState(): State;
}
