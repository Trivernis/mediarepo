import {ComponentFixture, TestBed} from "@angular/core/testing";

import {FilterExpressionItemComponent} from "./filter-expression-item.component";

describe("FilterItemComponent", () => {
    let component: FilterExpressionItemComponent;
    let fixture: ComponentFixture<FilterExpressionItemComponent>;

    beforeEach(async () => {
        await TestBed.configureTestingModule({
            declarations: [FilterExpressionItemComponent]
        })
            .compileComponents();
    });

    beforeEach(() => {
        fixture = TestBed.createComponent(FilterExpressionItemComponent);
        component = fixture.componentInstance;
        fixture.detectChanges();
    });

    it("should create", () => {
        expect(component).toBeTruthy();
    });
});
