import {TabCategory} from "./TabCategory";
import {StateServices} from "./StateServices";
import {SaveState} from "./SaveState";
import {FilesTabState} from "./FilesTabState";
import {ImportTabState} from "./ImportTabState";

export type TabSaveState = {
    uuid: number,
    category: TabCategory,
}

export class TabState implements SaveState<TabSaveState> {
    constructor(
        public uuid: number,
        public category: TabCategory,
        protected services: StateServices
    ) {
    }

    public toSaveState(): TabSaveState {
        return {
            uuid: this.uuid,
            category: this.category
        };
    }

    public restoreSaveState(state: TabSaveState): void {
        this.uuid = state.uuid;
        this.category = state.category;
    }

    /**
     * Converts the state into a files tab state
     * Warning: This should only be called when it's guaranteed that this tab is a files tab
     * @returns {ImportTabState}
     */
    public filesTab(): FilesTabState {
        return this as unknown as FilesTabState;
    }

    /**
     * Converts the state into an import tab state
     * Warning: This should only be called when it's guaranteed that this tab is an import tab
     * @returns {ImportTabState}
     */
    public importTab(): ImportTabState {
        return this as unknown as ImportTabState;
    }
}
