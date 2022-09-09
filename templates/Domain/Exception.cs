using Volo.Abp;
{% set exception_with_suffix = exception_name ~ "Exception" -%}
namespace {{namespace}}.{{entities}}
{
    public class {{exception_with_suffix}} : BusinessException
    {
        public {{exception_with_suffix}}(string name) : base({{error_codes}}.{{entity}}{{exception_name}})
        {
            WithData("name", name);
        }
    }
}