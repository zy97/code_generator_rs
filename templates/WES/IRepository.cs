{% let class_names = class_name|pluralize %}
{%- let class_name_camel = class_name|camel %}
{%- let class_names_camel = class_names|camel -%}
using WES.Entity.Model.{{class_names}};
using WES.Entity.Entity;
using WES.Repository.SqlSugar.Internal;

namespace {{namespace}}
{
    public interface I{{class_name}}Repository : IBaseRepository<{{class_name}}>
    {
        Task<SqlSugarPagedList<{{class_name}}>> Get{{class_names}}Async(Query{{class_name}}Dto queryDto);
    }
}
