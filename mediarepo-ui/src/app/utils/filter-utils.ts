import {FilterExpression, FilterQuery, PropertyQuery, TagQuery, ValueComparator} from "../../api/api-types/files";

export function filterExpressionToString(expression: FilterExpression) {
    let stringExpression = "";

    if ("OrExpression" in expression) {
        for (const query of expression.OrExpression) {
            stringExpression += filterQueryToString(query) + " OR ";
        }
        stringExpression = stringExpression.replace(/ OR $/, "");
    } else {
        stringExpression += filterQueryToString(expression.Query);
    }

    return stringExpression;
}

function filterQueryToString(query: FilterQuery): string {
    if ("Tag" in query) {
        return tagQueryToString(query.Tag);
    } else {
        return buildExpression(...propertyQueryToStringParts(query.Property));
    }
}

function tagQueryToString(tagQuery: TagQuery): string {
    return `${tagQuery.negate ? "-" : ""}${tagQuery.tag}`;
}

export function propertyQueryToStringParts(propertyQuery: PropertyQuery): [string, string, string] {
    if ("Status" in propertyQuery) {
        return ["Status", "=", propertyQuery.Status];
    } else if ("FileSize" in propertyQuery) {
        return [
            "FileSize",
            getComparator(propertyQuery.FileSize),
            getValue(propertyQuery.FileSize).toString()
        ];
    } else if ("ImportedTime" in propertyQuery) {
        return [
            "ImportedTime",
            getComparator(propertyQuery.ImportedTime),
            getValue(propertyQuery.ImportedTime)
        ];
    } else if ("ChangedTime" in propertyQuery) {
        return [
            "ChangedTime",
            getComparator(propertyQuery.ChangedTime),
            getValue(propertyQuery.ChangedTime)
        ];
    } else if ("CreatedTime" in propertyQuery) {
        return [
            "CreatedTime",
            getComparator(propertyQuery.CreatedTime),
            getValue(propertyQuery.CreatedTime)
        ];
    } else if ("TagCount" in propertyQuery) {
        return [
            "TagCount",
            getComparator(propertyQuery.TagCount),
            getValue(propertyQuery.TagCount).toString()
        ];
    } else if ("Cd" in propertyQuery) {
        return ["ContentDescriptor", "=", propertyQuery.Cd];
    } else if ("Id" in propertyQuery) {
        return ["FileId", "=", propertyQuery.Id.toString()];
    } else {
        return ["Invalid Expression", "", ""];
    }
}

function getComparator(value: ValueComparator<any>): "=" | "<" | ">" | "between" {
    if ("Greater" in value) {
        return ">";
    } else if ("Equal" in value) {
        return "=";
    } else if ("Less" in value) {
        return "<";
    } else {
        return "between";
    }
}

function getValue<T>(value: ValueComparator<T>): T {
    const singleValueKeys: ("Greater" | "Equal" | "Less")[] = ["Greater", "Equal", "Less"];

    for (const key of singleValueKeys) {
        if (key in value) {
            //@ts-ignore
            return value[key];
        }
    }
    if ("Between" in value) {
        return value.Between[0];
    } else {
        return "" as unknown as T;  // unreachable
    }
}

function buildExpression(property: string, comparator: string, value: string): string {
    return `.${property} ${comparator} ${value}`;
}
