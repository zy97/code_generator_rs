using System;
using Volo.Abp.Application.Services;
using System.Threading.Tasks;
using System.Collections.Generic;
using Volo.Abp.Application.Dtos;
namespace {{namespace}}.{{folder}}
{
{%- if custom %}
    public interface I{{entity}}AppService : IApplicationService
    {
        Task<{{entity}}Dto> GetAsync(Guid id);
        Task<PagedResultDto<{{entity}}Dto>> GetListAsync(PagedAndSortedAndFilteredResultRequestDto queryDto);
        Task<{{entity}}Dto> CreateAsync(Create{{entity}}Dto questionDto);
        Task<{{entity}}Dto> UpdateAsync(Guid id, Update{{entity}}Dto update{{entity}}Dto);
        Task DeleteAsync(Guid id);
    }
{%- else %}
    public interface I{{entity}}AppService : ICrudAppService<{{entity}}Dto, {{id}}, PagedAndSortedAndFilteredResultRequestDto, Create{{entity}}Dto, Update{{entity}}Dto>
    {
    }   
{%- endif %}
}