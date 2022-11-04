{% set dto = entity ~ "Dto" -%}
{% set camelName = entity|camel -%}
{% set entities = entity|plural -%}

import { makeAutoObservable } from "mobx";
import { {{camelName}}Api } from "../apis";
class {{entity}}Store {
  constructor() {
    makeAutoObservable(this);
  }
  get{{entities}} = async (data: { current: number; pageSize: number }, form: any) => {
    try {
      const result = await {{camelName}}Api.get{{entities}}({skipCount: data.pageSize * (data.current - 1),maxResultCount: data.pageSize,...form,});
      return {total: result.data.totalCount,list: result.data.items,};
    } catch (error) {
      return { total: 0, list: [] };
    }
  };
  async delete{{entity}}(id: string) {
    try {
      await {{camelName}}Api.delete{{entity}}(id);
      return true;
    } catch (error) {
      return false;
    }
  }
  async add{{entity}}({{camelName}}: {{entity}}CreateDto) {
    try {
      const data = await {{camelName}}Api.add{{entity}}({{camelName}});
      return data.data;
    } catch (error) {
      return;
    }
  }
  async get{{entity}}ById(id: string) {
    try {
      const {{camelName}} = await {{camelName}}Api.get{{entity}}ById(id);
      return {{camelName}}.data;
    } catch (error) {
      return;
    }
  }
  async update{{entity}}(id: string, {{camelName}}: {{entity}}UpdateDto) {
    try {
      const result = await {{camelName}}Api.update{{entity}}(id, {{camelName}});
      return result.data;
    } catch (error) {
      return;
    }
  }
}

export default new {{entity}}Store();
