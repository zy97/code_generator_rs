{% let class_names = class_name|pluralize %}
{%- let class_name_camel = class_name|camel %}
{%- let class_names_camel = class_names|camel -%}
using WES.Entity.Entity;
using WES.Entity.Model.{{class_names}};
using WES.Entity.Model;
using WES.Repository.IRepository;
using WES.Services.IServices;
using Mapster;

namespace WES.Services.Services
{
    public class {{class_name}}Service : I{{class_name}}Service
    {
        private readonly I{{class_name}}Repository {{class_name_camel}}Repository;

        public {{class_name}}Service(I{{class_name}}Repository {{class_name_camel}}Repository)
        {
            this.{{class_name_camel}}Repository = {{class_name_camel}}Repository;
        }
        // 添加
        public async Task<bool> Add{{class_name}}Async(Create{{class_name}}Dto {{class_name_camel}})
        {
            var existing{{class_name}} = await Get{{class_name}}ByXXXAsync();
            if (existing{{class_name}} != null)
            {
                throw new Exception("已存在");
            }
            else
            {
                var res = {{class_name_camel}}.Adapt<{{class_name}}>();
                return await this.{{class_name_camel}}Repository.InsertAsync(res);
            }
        }
       
        public async Task<{{class_name}}> Get{{class_name}}ByXXXAsync()
        {
            var {{class_name_camel}} = await this.{{class_name_camel}}Repository.GetListAsync();
            return {{class_name_camel}}[0];
        }
        // 更新
        public async Task<bool> Update{{class_name}}Async( Update{{class_name}}Dto {{class_name_camel}})
        {
            var existing{{class_name}} = await Get{{class_name}}ById({{class_name_camel}}.Id);
            if (existing{{class_name}} == null)
            {
                throw new Exception("客户不存在");
            }
            else
            {
                var res = await Get{{class_name}}ByXXXAsync();
                if (res != null && res.Id != {{class_name_camel}}.Id)
                {
                    throw new Exception("客户已存在");
                }
                {{class_name_camel}}.Adapt(existing{{class_name}});
                return await this.{{class_name_camel}}Repository.UpdateAsync(existing{{class_name}});
            }
        }
        // 获取
        public async Task<{{class_name}}> Get{{class_name}}ById(int id)
        {
            var {{class_names_camel}} = await this.{{class_name_camel}}Repository.GetByIdAsync(id);
            return {{class_names_camel}};
        }

        //删除
        public async Task<bool> Delete{{class_name}}Async(int id)
        {
            return await this.{{class_name_camel}}Repository.DeleteByIdAsync(id);
        }

        //获取
        public async Task<PagingResultDto<{{class_name}}Dto>> Get{{class_names}}Async(Query{{class_name}}Dto query{{class_name}}Dto)
        {
            var res = await this.{{class_name_camel}}Repository.Get{{class_names}}Async(query{{class_name}}Dto);
            var items = new PagingResultDto<{{class_name}}Dto> { TotalCount = res.pagination.Total, Items = res.list.Adapt<List<{{class_name}}Dto>>() };
            return items;
        }
    }
}