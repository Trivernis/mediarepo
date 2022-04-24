import {ComponentFixture, TestBed} from "@angular/core/testing";

import {RepositoryMaintenanceComponent} from "./repository-maintenance.component";

describe("RepositoryMaintenanceComponent", () => {
    let component: RepositoryMaintenanceComponent;
    let fixture: ComponentFixture<RepositoryMaintenanceComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [RepositoryMaintenanceComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(RepositoryMaintenanceComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
