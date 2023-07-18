using WES.Entity.Dto.{{entities}};
using WES.Repository.Contract;
using WES.Entity.Dto;

namespace {{namespace}}
{
    public interface I{{entity}}Service : IScopedDependency
    {
        Task<bool> Add{{entity}}Async(Create{{entity}}Dto {{entity | snake}});
        Task<bool> Delete{{entity}}Async(int id);
        Task<PagingResultDto<{{entity}}Dto>> Get{{entity}}Async(Query{{entity}}Dto query{{entity}}Dto);
        Task<bool> Update{{entity}}Async(int id, Update{{entity}}Dto {{entity | snake}});
    }
}