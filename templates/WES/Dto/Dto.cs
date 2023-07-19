using System.ComponentModel;
using WES.Entity.Model;

namespace {{namespace}}.{{folder }}
{
    public class {{ entity}}Dto : BaseDto
    {
      {%- for name,type in properties %}
          public {{type}} {{name}} { get; set; }
      {%- endfor %}
    }
}
