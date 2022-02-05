import {ComponentFixture, TestBed} from "@angular/core/testing";

import {MetadataEntryComponent} from "./metadata-entry.component";

describe("MetadataEntryComponent", () => {
    let component: MetadataEntryComponent;
    let fixture: ComponentFixture<MetadataEntryComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [MetadataEntryComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(MetadataEntryComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
