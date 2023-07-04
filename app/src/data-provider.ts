import { DataProvider } from "@refinedev/core";
import { stringify } from "query-string";
import { invoke } from '@tauri-apps/api'
export const tauriDataProvider = (apiUrl: string): DataProvider => ({
    getList: async ({ resource, pagination }) => {
        const url = `${apiUrl}/${resource}`;
        console.log(url)
        const { current = 1, pageSize = 10 } = pagination ?? {};
        const query: {
            _start?: number;
            _end?: number;
        } = {
            _start: (current - 1) * pageSize,
            _end: current * pageSize,
        };

        const data = await invoke(`${url}?${stringify(query)}`, { name: 'World' })
        const total = 100;
        return {
            data,
            total,
        };
    },
    create: async ({ resource, variables }) => {
        const url = `${apiUrl}/${resource}`;

        const data = await invoke(url, { name: 'World' })

        return {
            data,
        };
    },
    update: async ({ resource, id, variables }) => {
        const url = `${apiUrl}/${resource}/${id}`;

        const data = await invoke(url, { name: 'World' })

        return {
            data,
        };
    },
    deleteOne: async ({ resource, id, variables }) => {
        const url = `${apiUrl}/${resource}/${id}`;

        const data = await invoke(url, { name: 'World' })

        return {
            data,
        };
    },
    getOne: async ({ resource, id }) => {
        const url = `${apiUrl}/${resource}/${id}`;

        const data = await invoke(url, { name: 'World' })

        return {
            data,
        };
    },
});