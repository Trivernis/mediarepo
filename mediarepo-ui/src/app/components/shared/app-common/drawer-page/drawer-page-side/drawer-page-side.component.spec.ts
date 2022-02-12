import {ComponentFixture, TestBed} from "@angular/core/testing";

import {DrawerPageSideComponent} from "./drawer-page-side.component";

describe("DrawerPageSideComponent", () => {
    let component: DrawerPageSideComponent;
    let fixture: ComponentFixture<DrawerPageSideComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [DrawerPageSideComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(DrawerPageSideComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
