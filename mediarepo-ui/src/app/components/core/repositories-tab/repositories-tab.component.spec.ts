import {ComponentFixture, TestBed} from "@angular/core/testing";

import {RepositoriesTabComponent} from "./repositories-tab.component";

describe("RepositoriesComponent", () => {
    let component: RepositoriesTabComponent;
    let fixture: ComponentFixture<RepositoriesTabComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [RepositoriesTabComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(RepositoriesTabComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
