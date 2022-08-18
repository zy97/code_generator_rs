{% set dto = entity ~ "Dto" -%}
{% set snakeName = entity|snake -%}
{% set entities = entity|plural -%}

import { makeAutoObservable } from "mobx";
import { {{snakeName}}Api } from "../apis";
import { Add{{entity}}Dto } from "../data/models/{{entity}}";
class {{entity}}Store {
  constructor() {
    makeAutoObservable(this);
  }
  get{{entities}} = async (data: { current: number; pageSize: number }, form: any) => {
    try {
      const result = await {{snakeName}}Api.get{{entities}}({
        skipCount: data.pageSize * (data.current - 1),
        maxResultCount: data.pageSize,
        ...form,
      });
      return {
        total: result.data.totalCount,
        list: result.data.items,
      };
    } catch (error) {
      return { total: 0, list: [] };
    }
  };
  async delete{{entity}}(id: string) {
    try {
      await {{snakeName}}Api.delete{{entity}}(id);
      return true;
    } catch (error) {
      return false;
    }
  }
  async add{{entity}}({{snakeName}}: Add{{entity}}Dto) {
    try {
      const data = await {{snakeName}}Api.add{{entity}}({{snakeName}});
      return data.data;
    } catch (error) {
      return;
    }
  }
  async get{{entity}}ById(id: string) {
    try {
      const {{snakeName}} = await {{snakeName}}Api.get{{entity}}ById(id);
      return {{snakeName}}.data;
    } catch (error) {
      return;
    }
  }
  async update{{entity}}(id: string, {{snakeName}}: Add{{entity}}Dto) {
    try {
      const result = await {{snakeName}}Api.update{{entity}}(id, {{snakeName}});
      return result.data;
    } catch (error) {
      return;
    }
  }
}

export default new {{entity}}Store();
