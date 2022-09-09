
using {{namespace}}.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore;
using System;
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
    }
}