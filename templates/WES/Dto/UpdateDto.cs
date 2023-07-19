using System.ComponentModel.DataAnnotations;

namespace {{namespace}}.{{folder }}
{
    public class {{entity}}Dto 
    {
       [Required]
        public int Id { get; set; }
      {%- for name,type in properties %}
      public {{type}} {{name}} { get; set; }
      {%- endfor %}
    }
}
