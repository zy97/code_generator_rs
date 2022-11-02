using Volo.Abp;
{# {% set exception_without_suffix = exception_name ~ "Exception" -%} #}
{% set exception_without_suffix = exception_name | trim_end_matches(pat="Exception") -%}
namespace {{namespace}}.{{entities}}
{
    public class {{exception_name}} : BusinessException
    {
        public {{exception_name}}(string name) : base({{project_name}}DomainErrorCodes.{{exception_without_suffix}})
        {
            WithData("name", name);
        }
         ////异常常量定义一般在Domain.Shared中
        //public const string {{exception_without_suffix}} = "projectName:xxxxxx";
        ////展示文本一般定义在Domain.Shared中的资源Location资源中
        // "projectName:xxxxxx": "这是异常展示信息{name},异常构造函数有多少个参数，这里就可以显示指定参数信息"
    }
}