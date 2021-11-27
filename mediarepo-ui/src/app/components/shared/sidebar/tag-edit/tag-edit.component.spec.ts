import {ComponentFixture, TestBed} from "@angular/core/testing";

import {TagEditComponent} from "./tag-edit.component";

describe("FileEditComponent", () => {
    let component: TagEditComponent;
    let fixture: ComponentFixture<TagEditComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [TagEditComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(TagEditComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
