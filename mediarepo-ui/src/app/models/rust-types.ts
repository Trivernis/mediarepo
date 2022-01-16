export type RustEnum<VariantData> = {
    [key: string]: VariantData
};

export function createRustEnum<T, VariantData>(variant: string, data: VariantData): T {
    let enumInstance: RustEnum<VariantData> = {};
    enumInstance[variant] = data;

    return enumInstance as unknown as T;
}
