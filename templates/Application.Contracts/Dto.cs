using System;
using Volo.Abp.Application.Dtos;

namespace {{namespace}}.{{folder}}
{
    [Serializable]
    public class {{entity}}Dto : ExtensibleFullAuditedEntityDto<{{id}}>
    {
      {%- for name,type in properties %}
      public {{type}} {{name}} { get; set; }
      {%- endfor %}
    }
}