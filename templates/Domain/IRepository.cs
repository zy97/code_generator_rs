using JetBrains.Annotations;
using System;
using System.Collections.Generic;
using System.Threading;
using System.Threading.Tasks;
using Volo.Abp.Domain.Repositories;

namespace {{namespace}}.{{entities}}
{
    /// <summary>
    /// 这些方法似乎没必要，因为标准仓库有IQueryable，可以直接使用而不是定义自定发方法，处于学习的目的，只是为了演示才使用
    /// <para>但真是情况是定义具体仓库的具体实现,某些更高级的操作在具体的仓库实现方便，比如EfCore仓库</para>
    /// </summary>
    public interface I{{entity}}Repository : IBasicRepository<{{entity}}, {{generic_type}}>
    {
        //为每个接口添加一个CancellationToken的可选参数,如果只是返回单个对象，添加一个默认值为true的includeDetails参数
        Task<{{entity}}> FindByName([NotNull] string name, bool includeDetails = true, CancellationToken cancellationToken = default);
        //如果返回的是一个集合，那么includeDetails参数应该默认为false
        Task<List<{{entity}}>> GetAll([NotNull] string name, bool includeDetails = false, CancellationToken cancellationToken = default);
        //除非对性能有很高的要求，一般不用返回映射类（只是返回实体某部分属性）
    }
}