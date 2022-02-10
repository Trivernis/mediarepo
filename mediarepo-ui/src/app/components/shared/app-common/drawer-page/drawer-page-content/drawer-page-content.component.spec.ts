import {ComponentFixture, TestBed} from "@angular/core/testing";

import {DrawerPageContentComponent} from "./drawer-page-content.component";

describe("DrawerPageContentComponent", () => {
    let component: DrawerPageContentComponent;
    let fixture: ComponentFixture<DrawerPageContentComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [DrawerPageContentComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(DrawerPageContentComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
