import {ComponentFixture, TestBed} from "@angular/core/testing";

import {ExternalUrlComponent} from "./external-url.component";

describe("ExternalUrlComponent", () => {
    let component: ExternalUrlComponent;
    let fixture: ComponentFixture<ExternalUrlComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [ExternalUrlComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(ExternalUrlComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
