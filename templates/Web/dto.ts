import { ExtensibleEntityDto, ExtensibleObject, PagedAndSortedResultRequestDto } from "@abp/ng.core";

export interface {{dto_name}}Dto extends ExtensibleEntityDto<string> {

}
export interface {{dto_name}}CreateOrUpdateDtoBase extends ExtensibleObject {
}
export interface {{dto_name}}CreateDto extends {{dto_name}}CreateOrUpdateDtoBase {

}
export interface {{dto_name}}UpdateDto extends {{dto_name}}CreateOrUpdateDtoBase {
    concurrencyStamp?: string;
}
export interface Get{{dto_name}}Input extends PagedAndSortedResultRequestDto {
    filter?: string;
}

