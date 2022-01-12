import {ComponentFixture, TestBed} from "@angular/core/testing";

import {PropertyQueryItemComponent} from "./property-query-item.component";

describe("PropertyQueryItemComponent", () => {
    let component: PropertyQueryItemComponent;
    let fixture: ComponentFixture<PropertyQueryItemComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [PropertyQueryItemComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(PropertyQueryItemComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
