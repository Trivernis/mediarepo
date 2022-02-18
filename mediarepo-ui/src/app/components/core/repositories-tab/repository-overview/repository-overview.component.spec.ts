import {ComponentFixture, TestBed} from "@angular/core/testing";

import {RepositoryOverviewComponent} from "./repository-overview.component";

describe("RepositoryOverviewComponent", () => {
    let component: RepositoryOverviewComponent;
    let fixture: ComponentFixture<RepositoryOverviewComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [RepositoryOverviewComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(RepositoryOverviewComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
