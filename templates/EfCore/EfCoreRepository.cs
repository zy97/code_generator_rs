
using {{namespace}}.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;
using System;
using JetBrains.Annotations;
using System.Threading;
using System.Collections.Generic;
using System.Threading.Tasks;
using Volo.Abp.Domain.Repositories.EntityFrameworkCore;
using Volo.Abp.EntityFrameworkCore;

namespace {{namespace}}.{{entities}}
{
    public class EfCore{{entity}}Repository : EfCoreRepository<{{dbcontext}}, {{entity}}, {{generic_type}}>, I{{entity}}Repository
    {
        public EfCore{{entity}}Repository(IDbContextProvider<{{dbcontext}}> dbContextProvider) : base(dbContextProvider)
        {
        }
        //为每个接口添加一个CancellationToken的可选参数,如果只是返回单个对象，添加一个默认值为true的includeDetails参数
        public Task<{{entity}}> FindByName([NotNull] string name, bool includeDetails = true, CancellationToken cancellationToken = default)
        {
            throw new NotImplementedException();
        }
        //如果返回的是一个集合，那么includeDetails参数应该默认为false
        public Task<List<{{entity}}>> GetAll([NotNull] string name, bool includeDetails = false, CancellationToken cancellationToken = default)
        {
            throw new NotImplementedException();
        }
        //除非对性能有很高的要求，一般不用返回映射类（只是返回实体某部分属性）
        public override async Task<IQueryable<{{entity}}>> WithDetailsAsync()
        {
            return (await GetQueryableAsync()).IncludeDetails();
        }
    }
    // 如果聚合根有子集合，那么创建一个扩展方法获取详情
    public static class {{entity}}
    {
        public static IQueryable<{{entity}}> IncludeDetails(this IQueryable<{{entity}}> queryable, bool include = true)
        {
            if (!include)
                return queryable;
            return queryable.Include(i => i.Aaa).Include(i => i.Bbb);

        }
    }
}