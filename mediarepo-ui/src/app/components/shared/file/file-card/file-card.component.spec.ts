import {ComponentFixture, TestBed} from "@angular/core/testing";

import {FileCardComponent} from "./file-card.component";

describe("FileGridEntryComponent", () => {
    let component: FileCardComponent;
    let fixture: ComponentFixture<FileCardComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [FileCardComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(FileCardComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
