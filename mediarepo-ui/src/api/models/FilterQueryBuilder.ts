import {FileStatus, FilterExpression, FilterQuery, PropertyQuery, ValueComparator} from "../api-types/files";

export type Comparator = "Less" | "Equal" | "Greater" | "Between";
export type PropertyType =
    "Status"
    | "FileSize"
    | "ImportedTime"
    | "ChangedTime"
    | "CreatedTime"
    | "TagCount"
    | "Cd"
    | "Id";

export class FilterQueryBuilder {

    public static tag(tag: string, negate: boolean): FilterQuery {
        return { Tag: { tag, negate } };
    }

    public static status(status: FileStatus): FilterQuery {
        return filterQuery({ Status: status });
    }

    public static fileSize(size: number, comparator: Comparator, max_size?: number): FilterQuery {
        return filterQuery(
            { FileSize: valuesToCompareEnum(size, comparator, max_size) });
    }

    public static importedTime(date: Date, comparator: Comparator, max_date: Date): FilterQuery {
        return filterQuery({
            ImportedTime: valuesToCompareEnum(date, comparator,
                max_date
            )
        });
    }

    public static changedTime(date: Date, comparator: Comparator, max_date: Date): FilterQuery {
        return filterQuery({
            ChangedTime: valuesToCompareEnum(date, comparator, max_date)
        });
    }

    public static createdTime(date: Date, comparator: Comparator, max_date: Date): FilterQuery {
        return filterQuery({
            CreatedTime: valuesToCompareEnum(date, comparator, max_date)
        });
    }

    public static tagCount(count: number, comparator: Comparator, max_count: number): FilterQuery {
        return filterQuery({
            TagCount: valuesToCompareEnum(count, comparator, max_count)
        });
    }

    public static contentDescriptor(descriptor: string): FilterQuery {
        return filterQuery({ Cd: descriptor });
    }

    public static fileId(id: number): FilterQuery {
        return filterQuery({ Id: id });
    }

    public static buildFilterExpressionsFromString(expressionStr: string): FilterExpression | undefined {
        const parts = expressionStr.split(/\s+or\s+/gi);
        const queries = parts.map(part => this.buildFilterFromString(part)).filter(f => f != undefined) as FilterQuery[];

        if (queries.length > 0) {
            return { OrExpression: queries };
        } else if (queries.length == 1) {
            return { Query: queries[0] };
        } else {
            return undefined;
        }
    }

    public static buildFilterFromString(filterStr: string): FilterQuery | undefined {
        filterStr = filterStr.trim();

        if (filterStr.startsWith(".")) {
            const cleanFilter = filterStr.replace(/^\./, "");
            const parsedPropertyFilter = this.parsePropertyFilterQuery(cleanFilter);
            if (parsedPropertyFilter) {
                return parsedPropertyFilter;
            }
        } else if (filterStr.startsWith("-")) {
            const tag = filterStr.replace(/^-/, "").trim();
            return this.tag(tag, true);
        }

        return this.tag(filterStr, false);
    }

    private static parsePropertyFilterQuery(expression: string): FilterQuery | undefined {
        let propertyName = "";
        let compareValue = "";
        let rawComparator = "";
        let comparatorStarted = false;
        let valueStarted = false;

        for (const char of expression) {
            if (!valueStarted) {
                switch (char) {
                    case " ":
                        break;
                    case "=":
                    case "!":
                    case ">":
                    case "<":
                        rawComparator += char;
                        comparatorStarted = true;
                        break;
                    default:
                        valueStarted = comparatorStarted;
                        if (valueStarted) {
                            compareValue += char;
                        } else {
                            propertyName += char;
                        }
                }
            } else {
                compareValue += char;
            }
        }

        return this.parseQueryFromParts(propertyName, rawComparator, compareValue);
    }

    private static parseQueryFromParts(
        propertyName: string,
        rawComparator: string,
        compareValue: string
    ): FilterQuery | undefined {
        const property = this.parsePropertyName(propertyName);
        const comparator = this.parseComparator(rawComparator);

        if (property && comparator) {
            let value;
            switch (property) {
                case "Status":
                    value = parseStatus(compareValue);
                    if (comparator === "Equal" && value != undefined) {
                        return this.status(value);
                    }
                    break;
                case "FileSize":
                    value = this.parsePropertyValue(compareValue, parseNumber);
                    if (value != undefined) {
                        return this.fileSize(value[0], comparator, value[1]);
                    }
                    break;
                case "ImportedTime":
                    value = this.parsePropertyValue(compareValue, parseDate);
                    if (value != undefined) {
                        return this.importedTime(value[0], comparator, value[1]);
                    }

                    break;
                case "ChangedTime":
                    value = this.parsePropertyValue(compareValue, parseDate);
                    if (value != undefined) {
                        return this.changedTime(value[0], comparator, value[1]);
                    }
                    break;
                case "CreatedTime":
                    value = this.parsePropertyValue(compareValue, parseDate);
                    if (value != undefined) {
                        return this.createdTime(value[0], comparator, value[1]);
                    }
                    break;
                case "TagCount":
                    value = this.parsePropertyValue(compareValue, parseNumber);
                    if (value != undefined) {
                        return this.tagCount(value[0], comparator, value[1]);
                    }
                    break;
                case "Cd":
                    if (compareValue) {
                        return this.contentDescriptor(compareValue);
                    }
                    break;
                case "Id":
                    value = parseNumber(compareValue);

                    if (value != undefined) {
                        return this.fileId(value);
                    }

                    break;
            }
        }

        return undefined;
    }

    private static parseComparator(comparatorStr: string): Comparator | undefined {
        switch (comparatorStr) {
            case "=":
            case "==":
                return "Equal";
            case "<":
                return "Less";
            case ">":
                return "Greater";
            default:
                return;
        }
    }

    private static parsePropertyName(nameStr: string): PropertyType | undefined {
        switch (nameStr.toLowerCase().replace(/-_/g, "")) {
            case "status":
                return "Status";
            case "filesize":
                return "FileSize";
            case "importedat":
            case "importeddate":
            case "importedtime":
                return "ImportedTime";
            case "changedat":
            case "changeddate":
            case "changedtime":
                return "ChangedTime";
            case "createdat":
            case "createddate":
            case "createdtime":
                return "CreatedTime";
            case "tagcount":
                return "TagCount";
            case "cd":
            case "contentdescriptor":
                return "Cd";
            case "id":
            case "fileid":
                return "Id";
            default:
                return;
        }
    }

    private static parsePropertyValue<T>(
        valueStr: string,
        parseFn: (valueStr: string) => T | undefined
    ): T[] | undefined {
        const [firstValue, secondValue] = valueStr.split(" ");
        if (secondValue != undefined) {
            const firstValueParsed = parseFn(firstValue);
            const secondValueParsed = parseFn(secondValue);

            if (firstValueParsed && secondValueParsed) {
                return [firstValueParsed, secondValueParsed];
            }
        } else {
            const value = parseFn(firstValue);
            return value != undefined ? [value] : undefined;
        }

        return;
    }
}

function filterQuery(propertyQuery: PropertyQuery): FilterQuery {
    return { Property: propertyQuery };
}

function valuesToCompareEnum<T>(min_value: T, comparator: Comparator, max_value?: T): ValueComparator<T> {
    switch (comparator) {
        case "Less":
            return { Less: min_value };
        case "Equal":
            return { Equal: min_value };
        case "Greater":
            return { Greater: min_value };
        case "Between":
            return { Between: [min_value, max_value!] };
    }
}

function parseNumber(value: string): number | undefined {
    const num = Number(value);
    return isNaN(num) ? undefined : num;
}

function parseDate(value: string): Date | undefined {
    const date = Date.parse(value);

    if (isNaN(date)) {
        return undefined;
    }
    return new Date(date);
}

function parseStatus(value: string): FileStatus | undefined {
    switch (value.toLowerCase()) {
        case "imported":
            return "Imported";
        case "archived":
            return "Archived";
        case "deleted":
            return "Deleted";
        default:
            return undefined;
    }
}
