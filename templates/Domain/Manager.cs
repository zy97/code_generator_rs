{% set snakeName = entity|snake -%}
{% set repository = snakeName ~ "Repository" -%}
using JetBrains.Annotations;
using System.Threading.Tasks;
using Volo.Abp;
using Volo.Abp.Domain.Services;

namespace {{namespace}}.{{entities}}
{
    /// <summary>
    /// 不要引入域服务，除非真的需要与执行一些核心业务规则，比如现在确保目录名是唯一的
    /// </summary>
    public class {{entity}}Manager : DomainService
    {
        private readonly I{{entity}}Repository {{repository}};

        public {{entity}}Manager(I{{entity}}Repository {{repository}})
        {
            this.{{repository}} = {{repository}};
        }
        // public async Task<{{entity}}> CreateAsync([NotNull] string name, [NotNull] string displayName)
        // {
        //     Check.NotNull(name, nameof(name));
        //     Check.NotNull(displayName, nameof(displayName));
        //     var existing{{entity}} = await {{repository}}.FindByNameAsync(name);
        //     if (existing{{entity}} is not null)
        //     {
        //         throw new {{entity}}AlreadyExistingException(name);
        //     }
        //     return new {{entity}}(GuidGenerator.Create(), name, displayName);
        // }
    }
}
