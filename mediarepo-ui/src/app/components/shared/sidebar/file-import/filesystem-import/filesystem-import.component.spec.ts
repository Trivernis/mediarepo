import {ComponentFixture, TestBed} from "@angular/core/testing";

import {FilesystemImportComponent} from "./filesystem-import.component";

describe("FilesystemImportComponent", () => {
    let component: FilesystemImportComponent;
    let fixture: ComponentFixture<FilesystemImportComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [FilesystemImportComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(FilesystemImportComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
