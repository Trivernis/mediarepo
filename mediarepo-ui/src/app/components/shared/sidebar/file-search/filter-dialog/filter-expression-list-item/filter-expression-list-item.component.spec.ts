import {ComponentFixture, TestBed} from "@angular/core/testing";

import {FilterExpressionListItemComponent} from "./filter-expression-list-item.component";

describe("FilterExpressionListItemComponent", () => {
    let component: FilterExpressionListItemComponent;
    let fixture: ComponentFixture<FilterExpressionListItemComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [FilterExpressionListItemComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(FilterExpressionListItemComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
