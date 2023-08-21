{% let class_names = class_name|pluralize %}
{%- let class_name_camel = class_name|camel %}
{%- let class_names_camel = class_names|camel -%}
using WES.Entity.Model.{{class_names}};
using WES.Entity.Entity;
using WES.Repository.IRepository;
using SqlSugar;
using WES.Repository.SqlSugar.Internal;
using WES.Repository.SqlSugar.Extensions;
namespace {{namespace}}
{
    public class {{class_name}}Repository : Repository<{{class_name}}>, I{{class_name}}Repository
    {
        private readonly ISqlSugarClient db;
        public {{class_name}}Repository(ISqlSugarClient db) : base(db)
        {
            this.db = db;
        }

        public Task<SqlSugarPagedList<{{class_name}}>> Get{{class_names}}Async(Query{{class_name}}Dto {{class_name_camel}})
        {
           return this.Context.Queryable<{{class_name}}>()
            {%- for property in properties %}
                {%- if property.property_type == "string" %}
              .WhereIF(!string.IsNullOrEmpty({{class_name_camel}}.{{property.property_name}}), i => i.{{property.property_name}}.Contains({{class_name_camel}}.{{property.property_name}}))
                {%- else if property.property_type.contains("DateTime")%}
              .WhereIF({{class_name_camel}}.{{property.property_name}} != null, i => SqlFunc.Between(i.{{property.property_name}}, {{class_name_camel}}.{{property.property_name}}, {{class_name_camel}}.{{property.property_name}}.Value.GetLastDateTime()))
                {%- endif %}
            {%- endfor %}
                .ToPagedListAsync({{class_name_camel}}.PageIndex, {{class_name_camel}}.PageSize);
        }
    }
}