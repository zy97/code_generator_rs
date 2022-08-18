using System;
using Volo.Abp.Application.Services;

namespace {{namespace}}.{{folder}}
{
{%- if custom %}
    public interface I{{entity}}Service : IApplicationService
    {
    }
{%- else %}
    public interface I{{entity}}Service : ICrudAppService<{{entity}}Dto, {{id}}, PagedAndSortedAndFilteredResultRequestDto, CreateOrUpdate{{entity}}Dto>
    {
    }   
{%- endif %}
}