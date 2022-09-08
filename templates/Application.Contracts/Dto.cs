using System;
using Volo.Abp.Application.Dtos;

namespace {{namespace}}.{{folder}}
{
    public class {{entity}}Dto : EntityDto<{{id}}>
    {
      {%- for name,type in properties %}
      public {{type}} {{name}} { get; set; }
      {%- endfor %}
    }
}