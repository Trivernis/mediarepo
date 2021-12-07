export type RustEnum<VariantData> = {
    [key: string]: VariantData
};

export function createRustEnum<VariantData>(variant: string, data: VariantData): RustEnum<VariantData> {
    let enumInstance: RustEnum<VariantData> = {};
    enumInstance[variant] = data;

    return enumInstance;
}
