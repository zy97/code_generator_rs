using AutoMapper;
using WES.Entity.Dto;
using WES.Entity.Dto.{{entities}};
using WES.Entity.Entity;
using WES.Entity.Exceptions.{{entity}};
using WES.Repository.IRepository;
using WES.Services.IServices;

namespace WES.Services.Services
{
    public class {{entity}}Service : I{{entity}}Service
    {
        private readonly I{{entity}}Repository {{entity|snake}}Repository;
        private readonly IMapper mapper;

        public {{entity}}Service(I{{entity}}Repository {{entity|snake}}Repository, IMapper mapper)
        {
            this.{{entity|snake}}Repository = {{entity|snake}}Repository;
            this.mapper = mapper;
        }
        // 添加
        public async Task<bool> Add{{entity}}Async(Create{{entity}}Dto {{entity|snake}})
        {
            var existing{{entity}} = await Get{{entity}}ByNameAsync({{entity|snake}}.{{entity}}Name);
            if (existing{{entity}} != null)
            {
                throw new {{entity}}AlreadyExistsException("已存在");
            }
            else
            {
                var res = mapper.Map<{{entity}}>({{entity|snake}});
                return await this.{{entity|snake}}Repository.InsertAsync(res);
            }
        }
        //以名字获取角色
        public async Task<{{entity}}> Get{{entity}}ByNameAsync(string {{entity|snake}}Name)
        {
            var {{entity|snake}} = await this.{{entity|snake}}Repository.GetFirstAsync(role => role.{{entity}}Name == {{entity|snake}}Name);
            return {{entity|snake}};
        }
        // 更新
        public async Task<bool> Update{{entity}}Async(int id, Update{{entity}}Dto {{entity|snake}})
        {
            var existing{{entity}} = await Get{{entity}}ById(id);
            if (existing{{entity}} == null)
            {
                throw new {{entity}}NotFoundException("客户不存在");
            }
            else
            {
                var res = await Get{{entity}}ByNameAsync({{entity|snake}}.{{entity}}Name);
                if (res != null && res.Id != {{entity|snake}}.Id)
                {
                    throw new {{entity}}AlreadyExistsException("客户已存在");
                }
                mapper.Map({{entity}}, existing{{entity}});
                return await this.{{entity|snake}}Repository.UpdateAsync(existing{{entity}});
            }
        }
        // 获取
        public async Task<{{entity}}> Get{{entity}}ById(int id)
        {
            var {{entities|snake}} = await this.{{entity|snake}}Repository.GetByIdAsync(id);
            return {{entities|snake}};
        }

        //删除
        public async Task<bool> Delete{{entity}}Async(int id)
        {
            return await this.{{entity|snake}}Repository.DeleteByIdAsync(id);
        }

        //获取
        public Task<PagingResultDto<{{entity}}Dto>> Get{{entity}}Async(Query{{entity}}Dto query{{entity}}Dto)
        {
            return this.{{entity|snake}}Repository.Get{{entity}}Async(query{{entity}}Dto);
        }
    }
}