using WES.Entity.Model;

namespace {{namespace}}.{{folder }} 
{
    public class {{ entity}}Dto : PagingRequestDto
    {
      {%- for name,type in properties %}
        {%- if type == "string" %}
            public {{type}} {{name}} { get; set; } = string.Empty;
        {%- else %}
            public {{type}} {{name}} { get; set; }
        {%- endif %}
      {%- endfor %}
    }
}
