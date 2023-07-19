using WES.Entity.Dto.{{entities}};
using WES.Entity.Entity;
using WES.Entity.Dto;

namespace {{namespace}}
{
    public interface I{{entity}}Repository : IBaseRepository<{{entity}}>
    {
        Task<(int total,List<{{entity}}> {{entities|camel}})> Get{{entities}}Async(Query{{entity}}Dto queryDto);
    }
}
