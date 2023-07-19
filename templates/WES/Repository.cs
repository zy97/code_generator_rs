using WES.Entity.Dto;
using WES.Entity.Dto.{{entities}};
using WES.Entity.Entity;
using WES.Repository.IRepository;
using AutoMapper;
using SqlSugar;

namespace {{namespace}}
{
    public class {{entity}}Repository : Repository<{{entity}}>, I{{entity}}Repository
    {
        private readonly ISqlSugarClient db;
        public {{entity}}Repository(ISqlSugarClient db) : base(db)
        {
            this.db = db;
        }

        public async Task<(int total, List<{{entity}}> {{entities|camel}})> Get{{entities}}Async(Query{{entity}}Dto {{entity|camel}})
        {
            RefAsync<int> total = 0;
            var {{entities|camel}} = await this.Context.Queryable<{{entity}}>()
            //.WhereIF(!string.IsNullOrEmpty({{entity|camel}}.CustomerName), u => u.CustomerName.Contains({{entity|camel}}.CustomerName))
            //.WhereIF(!string.IsNullOrEmpty({{entity|camel}}.Remark), u => u.Remark.Contains({{entity|camel}}.Remark))
            {%- for name,type in properties %}
                {%- if type == "string" %}
              .WhereIF(!string.IsNullOrEmpty({{entity|camel}}.{{name}}), i => i.{{name}}.Contains({{entity|camel}}.{{name}}))
                {%- endif %}
            {%- endfor %}
                .ToPageListAsync({{entity|camel}}.PageIndex, {{entity|camel}}.PageSize, total);
            return (total, {{entities|camel}});
        }
    }
}