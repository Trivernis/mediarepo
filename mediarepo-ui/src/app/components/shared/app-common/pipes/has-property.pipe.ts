import {Pipe, PipeTransform} from "@angular/core";

@Pipe({
    name: "hasProperty"
})
export class HasPropertyPipe implements PipeTransform {

    transform(value: any, propertyName: string): unknown {
        return propertyName in value;
    }
}
