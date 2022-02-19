import {ChangeDetectionStrategy, ChangeDetectorRef, Component, Input, OnChanges, SimpleChanges} from "@angular/core";
import {ChartOptions} from "chart.js";

export type ChartType = "doughnut";
export type Dataset = {
    label?: string,
    data: number[],
};

@Component({
    selector: "app-chart",
    templateUrl: "./chart.component.html",
    styleUrls: ["./chart.component.scss"],
    changeDetection: ChangeDetectionStrategy.OnPush
})
export class ChartComponent implements OnChanges {

    @Input() chartType?: ChartType;
    @Input() title?: string;
    @Input() labels: string[] = [];
    @Input() datasets: Dataset[] = [];

    public data: any = {};
    public options: ChartOptions = {
        responsive: true,
        elements: {
            arc: {
                borderWidth: 0
            }
        },
        plugins: {
            legend: {
                labels: {
                    color: "#FFF",
                    boxHeight: 20,
                    font: {
                        size: 16,
                    }
                },
            },
            tooltip: {
                titleFont: {
                    size: 16
                },
                bodyFont: {
                    size: 14
                }
            }
        }
    };
    private readonly colors = [
        "#771e86",
        "#4650b5",
        "#0073d0",
        "#0091d6",
        "#00aacb",
        "#00c0b7"
    ];

    constructor(private changeDetector: ChangeDetectorRef) {
    }

    public ngOnChanges(changes: SimpleChanges): void {
        if (changes["labels"] || changes["dataset"]) {
            this.generateData();
            this.changeDetector.markForCheck();
        }
        if (changes["chartType"]) {
            this.changeDetector.markForCheck();
        }
    }

    private generateData() {
        this.data = {
            labels: this.labels,
            datasets: this.datasets.map(set => {
                return {
                    label: set.label,
                    data: set.data,
                    backgroundColor: this.colors,
                    hoverBackgroundColor: this.colors,
                };
            })
        };
    }
}
