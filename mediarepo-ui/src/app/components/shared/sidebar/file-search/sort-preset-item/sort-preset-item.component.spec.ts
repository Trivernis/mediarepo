import {ComponentFixture, TestBed} from "@angular/core/testing";

import {SortPresetItemComponent} from "./sort-preset-item.component";

describe("SortPresetItemComponent", () => {
    let component: SortPresetItemComponent;
    let fixture: ComponentFixture<SortPresetItemComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [SortPresetItemComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(SortPresetItemComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
