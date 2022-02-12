import {Pipe, PipeTransform} from "@angular/core";

@Pipe({
    name: "formatBytes"
})
export class FormatBytesPipe implements PipeTransform {

    static round(number: number, decimals: number) {
        return Math.round(number * (10 ** decimals)) / (10 ** decimals);
    }

    transform(value: number): string {
        const units = ["B", "KiB", "MiB", "GiB"];
        let formattedValue = value;

        for (const unit of units) {
            if (formattedValue < 1000) {
                return `${formattedValue} ${unit}`;
            }
            formattedValue = FormatBytesPipe.round(formattedValue / 1024, 2);
        }
        return formattedValue + " GiB";
    }
}
