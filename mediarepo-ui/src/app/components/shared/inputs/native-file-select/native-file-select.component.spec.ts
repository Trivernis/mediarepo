import {ComponentFixture, TestBed} from "@angular/core/testing";

import {NativeFileSelectComponent} from "./native-file-select.component";

describe("NativeFileSelectComponent", () => {
    let component: NativeFileSelectComponent;
    let fixture: ComponentFixture<NativeFileSelectComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [NativeFileSelectComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(NativeFileSelectComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
