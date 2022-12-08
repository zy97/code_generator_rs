using JetBrains.Annotations;
using System;
using Volo.Abp;
using Volo.Abp.Domain.Entities.Auditing;

namespace {{namespace}}
{
    //如果不想使用DDD，可以删除构造函数以及内部的所有函数，业务逻辑可能就转移到仓库中去了
    public class {{entityName}} : FullAuditedAggregateRoot<{{type}}>
    {
        public string Aaa { get; private set; }
        public string Bbb { get; private set; } 
        private {{entityName}}()
        {

        }
        internal {{entityName}}(Guid id, [NotNull] string aaa, [NotNull] string bbb) : base(id)
        {
            Set(aaa, bbb);
        }
        private void Set([NotNull] string aaa, [NotNull] string bbb)
        {
            Aaa = Check.NotNullOrWhiteSpace(aaa, nameof(aaa), {{entityName}}Const.{{entityName}}ConstA);
            Bbb = Check.NotNullOrWhiteSpace(bbb, nameof(bbb), {{entityName}}Const.{{entityName}}ConstB);
        }
        internal {{entityName}} Change([NotNull] string aaa, [NotNull] string bbb)
        {
            Set(aaa, bbb);
            return this;
        }
    }

    //一般常量定义在domain.shared项目中
    public static class {{entityName}}Const
    {
        public const int {{entityName}}ConstA = 24;
        public const int {{entityName}}ConstB = 100;
    }
}
