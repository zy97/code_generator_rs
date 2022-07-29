import axios from "axios";
import PagedList from "../data/models/PagedList";
import PageRequest from "../data/models/PageRequest";
import { Add{{entity}}Dto, {{entity}}Dto } from "../data/models/{{entity}}";

// 添加
export const add{{entity}} = ({{entity|snake}}: Add{{entity}}Dto) =>
  axios.post<{{entity}}Dto>(`{{url_prefix}}`, { ...{{entity|snake}} });

// 获取列表
export const get{{entity}}s = (params: PageRequest) =>
  axios.get<PagedList<{{entity}}Dto>>(`{{url_prefix}}`, { params });

// 删除
export const delete{{entity}} = (id: string) =>
  axios.delete(`{{url_prefix}}/${id}`);

// 获取指定项
export const get{{entity}}ById = (id: string) =>
  axios.get<{{entity}}Dto>(`{{url_prefix}}/${id}`, {});

// 更新
export const update{{entity}} = (id: string, {{entity|snake}}: Add{{entity}}Dto) =>
  axios.put<Add{{entity}}Dto>(`{{url_prefix}}/${id}`, { ...{{entity|snake}} });
