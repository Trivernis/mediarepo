import {ComponentFixture, TestBed} from "@angular/core/testing";

import {FlapButtonComponent} from "./flap-button.component";

describe("FlapButtonComponent", () => {
    let component: FlapButtonComponent;
    let fixture: ComponentFixture<FlapButtonComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [FlapButtonComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(FlapButtonComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
