using System;
namespace {{namespace}}.{{folder}}
{
    public class CreateOrUpdate{{entity}}Dto
    {
      {%- for name,type in properties %}
      public {{type}} {{name}} { get; set; }
      {%- endfor %}
    }
}