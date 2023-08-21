{% let class_names = class_name|pluralize %}
{%- let class_name_camel = class_name|camel -%}
using WES.Entity.Model.{{class_names}};
using WES.Repository.Contract;
using WES.Entity.Model;

namespace {{namespace}}
{
    public interface I{{class_name}}Service : IScopedDependency
    {
        Task<bool> Add{{class_name}}Async(Create{{class_name}}Dto {{class_name_camel}});
        Task<bool> Delete{{class_name}}Async(int id);
        Task<PagingResultDto<{{class_name}}Dto>> Get{{class_names}}Async(Query{{class_name}}Dto query{{class_name}}Dto);
        Task<bool> Update{{class_name}}Async(Update{{class_name}}Dto {{class_name_camel}});
    }
}