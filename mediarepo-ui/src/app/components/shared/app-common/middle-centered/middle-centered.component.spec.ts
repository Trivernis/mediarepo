import {ComponentFixture, TestBed} from "@angular/core/testing";

import {MiddleCenteredComponent} from "./middle-centered.component";

describe("MiddleCenteredComponent", () => {
    let component: MiddleCenteredComponent;
    let fixture: ComponentFixture<MiddleCenteredComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [MiddleCenteredComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(MiddleCenteredComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
