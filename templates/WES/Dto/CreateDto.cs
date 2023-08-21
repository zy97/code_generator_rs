using System.ComponentModel;
using WES.Entity.Model;

namespace {{namespace}}
{
    public class Create{{class_name}}Dto : BaseDto
    {
      {%- for property in properties %}
          {%- if property.comment.len() != 0 %}
          {{property.comment-}}
          {% endif -%}
          public {{property.property_type}} {{property.property_name}} { get; set; }
      {%- endfor %}
    }
}
