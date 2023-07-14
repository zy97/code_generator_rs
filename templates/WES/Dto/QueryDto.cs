using System.ComponentModel;

namespace {{namespace}}.{{folder }} 
{
    public class {{ entity}}Dto : PagingRequestDto
    {
      {%- for name,type in properties %}
      public {{type}} {{name}} { get; set; }
      {%- endfor %}
    }
}
