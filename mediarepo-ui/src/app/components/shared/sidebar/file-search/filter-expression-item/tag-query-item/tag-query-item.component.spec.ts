import {ComponentFixture, TestBed} from "@angular/core/testing";

import {TagQueryItemComponent} from "./tag-query-item.component";

describe("TagQueryItemComponent", () => {
    let component: TagQueryItemComponent;
    let fixture: ComponentFixture<TagQueryItemComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [TagQueryItemComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(TagQueryItemComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
