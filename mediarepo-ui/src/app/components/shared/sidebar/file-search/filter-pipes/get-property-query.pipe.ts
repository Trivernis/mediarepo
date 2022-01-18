import {Pipe, PipeTransform} from "@angular/core";
import {FilterQuery, FilterQueryProperty, PropertyQuery} from "../../../../../../api/api-types/files";

@Pipe({
    name: "getPropertyQuery"
})
export class GetPropertyQueryPipe implements PipeTransform {

    transform(value: FilterQuery): PropertyQuery {
        return (value as FilterQueryProperty).Property;
    }

}
