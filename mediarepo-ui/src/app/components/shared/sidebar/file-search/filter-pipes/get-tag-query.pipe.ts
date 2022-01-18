import {Pipe, PipeTransform} from "@angular/core";
import {FilterQuery, FilterQueryTag, TagQuery} from "../../../../../../api/api-types/files";

@Pipe({
    name: "getTagQuery"
})
export class GetTagQueryPipe implements PipeTransform {

    transform(value: FilterQuery): TagQuery {
        return (value as FilterQueryTag).Tag;
    }
}
