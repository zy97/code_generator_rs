using System;
using Volo.Abp.Application.Services;
using Volo.Abp.Domain.Repositories;
using System.Linq;
using System.Threading.Tasks;
namespace {{namespace}}.{{folder}}
{
{% if custom %}
    public class {{entity}}Service : I{{entity}}Service
    {
        public {{entity}}Service()
        {

        }
    }
{% else %}
    public class {{entity}}Service : CrudAppService<{{entity}}, {{entity}}Dto, {{id}}, PagedAndSortedAndFilteredResultRequestDto, CreateOrUpdate{{entity}}Dto>, I{{entity}}Service
    {
         public {{entity}}Service(IRepository<{{entity}}, {{id}}> repository) : base(repository)
         {
         }
         //如果不需要过滤删除这个重载，属性判断根据自己的情况酌情调整
         protected override async Task<IQueryable<{{entity}}>> CreateFilteredQueryAsync(PagedAndSortedAndFilteredResultRequestDto input)
         {
            var queryable = await this.ReadOnlyRepository.GetQueryableAsync().ConfigureAwait(false);{% for property in properties %}{% if property.1 == "string"%}
            queryable = queryable.WhereIf(!string.IsNullOrWhiteSpace(input.{{property.0}}), i => i.{{property.0}}.Contains(input.{{property.0}}));{% else %}
            throw new NotImplementedException("属性{{property.0}}为{{property.1}}类型需自己实现过滤");{% endif %}{% endfor%}
            return queryable;
         }
    }   
{% endif %}
}
