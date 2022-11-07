using System;
using Volo.Abp.Application.Services;
using System.Threading.Tasks;
using System.Collections.Generic;

namespace {{namespace}}.{{folder}}
{
{%- if custom %}
    public interface I{{entity}}AppService : IApplicationService
    {
        Task<{{entity}}Dto> GetAsync(Guid id);
        Task<List<{{entity}}Dto>> GetListAsync(PagedAndSortedAndFilteredResultRequestDto queryDto);
        Task<{{entity}}Dto> CreateAsync(Create{{entity}}Dto questionDto);
        Task<{{entity}}Dto> UpdateAsync(Guid id, Update{{entity}}Dto update{{entity}}Dto);
        Task DeleteAsync(Guid id);
    }
{%- else %}
    public interface I{{entity}}AppService : ICrudAppService<{{entity}}Dto, {{id}}, PagedAndSortedAndFilteredResultRequestDto, CreateOrUpdate{{entity}}Dto>
    {
    }   
{%- endif %}
}