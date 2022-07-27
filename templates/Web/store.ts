import { makeAutoObservable } from "mobx";
import { {{entity|snake}}Api } from "../apis";
import { Add{{entity}}Dto } from "../data/models/{{entity}}";
class {{entity}}Store {
  constructor() {
    makeAutoObservable(this);
  }
  getTags = async (data: any, form: any) => {
    try {
      const result = await {{entity|snake}}Api.get{{entities}}({
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
      await {{entity|snake}}Api.delete{{entity}}(id);
      return true;
    } catch (error) {
      return false;
    }
  }
  async add{{entity}}({{entity|snake}}: Add{{entity}}Dto) {
    try {
      const data = await {{entity|snake}}Api.add{{entity}}({{entity|snake}});
      return data.data;
    } catch (error) {
      return false;
    }
  }
  async get{{entity}}ById(id: string) {
    try {
      const {{entity|snake}} = await {{entity|snake}}Api.get{{entity}}ById(id);
      return {{entity|snake}}.data;
    } catch (error) {
      console.log(error);
    }
  }
  async update{{entity}}(id: string, {{entity|snake}}: Add{{entity}}Dto) {
    try {
      const result = await {{entity|snake}}Api.update{{entity}}(id, {{entity|snake}});
      return result.data;
    } catch (error) {
      console.log(error);
    }
  }
}

export default new {{entity}}Store();
