import { ListResultDto, PagedResultDto } from "@abp/ng.core";
import axios from "axios";
{% set lower_entity = entity | snake -%}
// 添加
export const add{{ entity }} = ({{ lower_entity }}: {{ entity }}CreateDto) =>
  axios.post < {{ entity }}Dto > (`{{url_prefix}}`, { ...{{ lower_entity }} });

// 获取列表
export const get{{ entity }}s = (params: Get{{ entity }}Input) =>
  axios.get < PagedResultDto < {{ entity }}Dto >> (`{{url_prefix}}`, { params });

// 删除
export const delete{{ entity }} = (id: string) =>
  axios.delete(`{{url_prefix}}/${id}`);

// 获取指定项
export const get{{ entity }}ById = (id: string) =>
  axios.get < {{ entity }}Dto > (`{{url_prefix}}/${id}`, {});

// 更新
export const update{{ entity }} = (id: string, {{ lower_entity }}: {{ entity }}UpdateDto) =>
  axios.put < {{ entity }}Dto > (`{{url_prefix}}/${id}`, { ...{{ lower_entity }} });
