using AutoMapper;
using WES.Entity.Entity;
using WES.Entity.Dto.{{entities}};
using WES.Entity.Model;
using WES.Repository.IRepository;
using WES.Services.IServices;

namespace WES.Services.Services
{
    public class {{entity}}Service : I{{entity}}Service
    {
        private readonly I{{entity}}Repository {{entity|camel}}Repository;
        private readonly IMapper mapper;

        public {{entity}}Service(I{{entity}}Repository {{entity|camel}}Repository, IMapper mapper)
        {
            this.{{entity|camel}}Repository = {{entity|camel}}Repository;
            this.mapper = mapper;
        }
        // 添加
        public async Task<bool> Add{{entity}}Async(Create{{entity}}Dto {{entity|camel}})
        {
            var existing{{entity}} = await Get{{entity}}ByNameAsync({{entity|camel}}.{{entity}}Name);
            if (existing{{entity}} != null)
            {
                throw new Exception("已存在");
            }
            else
            {
                var res = mapper.Map<{{entity}}>({{entity|camel}});
                return await this.{{entity|camel}}Repository.InsertAsync(res);
            }
        }
        //以名字获取角色
        public async Task<{{entity}}> Get{{entity}}ByNameAsync(string {{entity|camel}}Name)
        {
            var {{entity|camel}} = await this.{{entity|camel}}Repository.GetFirstAsync(role => role.{{entity}}Name == {{entity|camel}}Name);
            return {{entity|camel}};
        }
        // 更新
        public async Task<bool> Update{{entity}}Async( Update{{entity}}Dto {{entity|camel}})
        {
            var existing{{entity}} = await Get{{entity}}ById({{entity|camel}}.Id);
            if (existing{{entity}} == null)
            {
                throw new Exception("客户不存在");
            }
            else
            {
                var res = await Get{{entity}}ByNameAsync({{entity|camel}}.{{entity}}Name);
                if (res != null && res.Id != {{entity|camel}}.Id)
                {
                    throw new Exception("客户已存在");
                }
                mapper.Map({{entity|camel}}, existing{{entity}});
                return await this.{{entity|camel}}Repository.UpdateAsync(existing{{entity}});
            }
        }
        // 获取
        public async Task<{{entity}}> Get{{entity}}ById(int id)
        {
            var {{entities|camel}} = await this.{{entity|camel}}Repository.GetByIdAsync(id);
            return {{entities|camel}};
        }

        //删除
        public async Task<bool> Delete{{entity}}Async(int id)
        {
            return await this.{{entity|camel}}Repository.DeleteByIdAsync(id);
        }

        //获取
        public async Task<PagingResultDto<{{entity}}Dto>> Get{{entities}}Async(Query{{entity}}Dto query{{entity}}Dto)
        {
            var (total, {{entities|camel}}) = await this.{{entity|camel}}Repository.Get{{entities}}Async(query{{entity}}Dto);
            return new PagingResultDto<{{entity}}Dto> { TotalCount = total, Items = mapper.Map<List<{{entity}}Dto>>({{entities|camel}}) };
        }
    }
}