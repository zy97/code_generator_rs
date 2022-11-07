using System;
using Volo.Abp.ObjectExtending;
namespace {{namespace}}.{{folder}}
{
    [Serializable]
    public class Create{{entity}}Dto : ExtensibleObject
    {
      {%- for name,type in properties %}
      public {{type}} {{name}} { get; set; }
      {%- endfor %}
    }
}