using WES.Entity.Dto.{{entities}};
using WES.Entity.Entity;
using WES.Entity.Dto;

namespace {{namespace}}
{
    public interface I{{entity}}Repository : IBaseRepository<{{entity}}>
    {
        Task<PagingResultDto<{{entity}}Dto>> Get{{entity}}Async(Query{{entity}}Dto query{{entity}}Dto);
    }
}
