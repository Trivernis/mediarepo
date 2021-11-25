import {ComponentFixture, TestBed} from "@angular/core/testing";

import {ImportTabSidebarComponent} from "./import-tab-sidebar.component";

describe("ImportTabSidebarComponent", () => {
    let component: ImportTabSidebarComponent;
    let fixture: ComponentFixture<ImportTabSidebarComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [ImportTabSidebarComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(ImportTabSidebarComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
