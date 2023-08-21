using System.ComponentModel;
using WES.Entity.Model;

namespace {{namespace}}
{
    public class Query{{class_name}}Dto : PagingRequestDto
    {
      {%- for property in properties %}
          {%- if property.comment.len() != 0 %}
          {{property.comment-}}
          {% endif -%}

          {%- if property.property_type == "string" -%}
          public {{property.property_type}} {{property.property_name}} { get; set; } = string.Empty;
          {%- else -%}
          public {{property.property_type}} {{property.property_name}} { get; set; }
          {%- endif -%}
      {%- endfor %}
    }

    public class ExportQuery{{class_name}}Dto 
    {
      {%- for property in properties %}
          {%- if property.comment.len() != 0 %}
          {{property.comment-}}
          {% endif -%}

          {%- if property.property_type == "string" -%}
          public {{property.property_type}} {{property.property_name}} { get; set; } = string.Empty;
          {%- else -%}
          public {{property.property_type}} {{property.property_name}} { get; set; }
          {%- endif -%}
      {%- endfor %}
    }
}
