{% set snakeName = entity|snake -%}
using System;
using Volo.Abp.Application.Services;
using Volo.Abp.Domain.Repositories;
using System.Linq;
using System.Collections.Generic;
using System.Threading.Tasks;
namespace {{namespace}}.{{folder}}
{
{%- if custom %}
    public class {{entity}}AppService : I{{entity}}AppService
    {
        private readonly I{{entity}}Repository {{snakeName}}Repository;
        public {{entity}}AppService(ITestRepository {{snakeName}}Repository)
        {
            this.{{snakeName}}Repository = {{snakeName}}Repository;
        }
        public Task<{{entity}}Dto> GetAsync(Guid id)
        {
            throw new NotImplementedException();
        }
        public Task<List<{{entity}}Dto>> GetListAsync(PagedAndSortedAndFilteredResultRequestDto queryDto)
        {
            throw new NotImplementedException();
        }
        public Task<{{entity}}Dto> CreateAsync(Create{{entity}}Dto questionDto)
        {
            throw new NotImplementedException();
        }
        public Task<{{entity}}Dto> UpdateAsync(Guid id, Update{{entity}}Dto update{{entity}}Dto)
        {
            throw new NotImplementedException();
        }
        public Task DeleteAsync(Guid id)
        {
            throw new NotImplementedException();
        }
    }
{%- else %}
    public class {{entity}}AppService : CrudAppService<{{entity}}, {{entity}}Dto, {{id}}, PagedAndSortedAndFilteredResultRequestDto, CreateOrUpdate{{entity}}Dto>, I{{entity}}AppService
    {
         public {{entity}}AppService(I{{entity}}Repository repository, {{entity}}Manager {{entity|snake}}Manage) : base(repository)
         {
         }
         //如果不需要过滤删除这个重载，属性判断根据自己的情况酌情调整
         protected override async Task<IQueryable<{{entity}}>> CreateFilteredQueryAsync(PagedAndSortedAndFilteredResultRequestDto input)
         {
            var queryable = await this.ReadOnlyRepository.GetQueryableAsync().ConfigureAwait(false);
            {%- for name,type in properties %}
            {%- if type == "string"%}
            queryable = queryable.WhereIf(!string.IsNullOrWhiteSpace(input.{{name}}), i => i.{{name}}.Contains(input.{{name}}));
            {%- else %}
            throw new NotImplementedException("属性{{property.0}}为{{property.1}}类型需自己实现过滤");
            {%- endif %}
            {%- endfor%}
            return queryable;
         }
    }   
{%- endif %}
}
