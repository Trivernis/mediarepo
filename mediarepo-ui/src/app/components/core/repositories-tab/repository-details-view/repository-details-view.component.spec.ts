import {ComponentFixture, TestBed} from "@angular/core/testing";

import {RepositoryDetailsViewComponent} from "./repository-details-view.component";

describe("RepositoryDetailsViewComponent", () => {
    let component: RepositoryDetailsViewComponent;
    let fixture: ComponentFixture<RepositoryDetailsViewComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [RepositoryDetailsViewComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(RepositoryDetailsViewComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
