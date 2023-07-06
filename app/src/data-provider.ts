import { DataProvider } from "@refinedev/core";
import { stringify } from "query-string";
import { invoke } from '@tauri-apps/api'
export const tauriDataProvider = (apiUrl: string): DataProvider => ({
    getList: async ({ resource, pagination, meta }) => {
        const url = `${resource}_get_list`;
        console.log(url, meta, pagination)
        const page = pagination ?? { current: 1, pageSize: 10 };
        // const data = await invoke(`${url}?${stringify(query)}`, { name: 'World' })
        const [total, data] = await invoke(url, { ...page })
        console.log(data)
        return {
            data,
            total,
        };
    },
    create: async ({ resource, variables }) => {
        const url = `${resource}_create`;
        console.log(url, variables);
        const data = await invoke(url, { ...variables })

        return {
            data,
        };
    },
    update: async ({ resource, id, variables }) => {
        const url = `${resource}_update`;
        console.log(url, variables);
        const data = await invoke(url, { id, ...variables })
        return {
            data,
        };
    },
    deleteOne: async ({ resource, id, variables }) => {
        const url = `${resource}_delete`;
        console.log("删除开始")

        const data = await invoke(url, { id: parseInt(id.toString()) })
        console.log("删除结束")
        return {
            data,
        };
    },
    getOne: async ({ resource, id }) => {
        const url = `${resource}_find`;
        console.log(url, id);
        const data = await invoke(url, { id: parseInt(id.toString()) })

        return {
            data,
        };
    },
    getMany: async ({ resource, ids, meta }) => {
        const url = `${resource}_get_many`;
        console.log(url, ids);
        const data = await invoke(url, { ids })
        console.log(data)
        return {
            data,
        };
    }
});