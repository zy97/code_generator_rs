using WES.Entity.Dto;
using WES.Entity.Dto.{{entities}};
using WES.Entity.Entity;
using WES.Repository.IRepository;
using AutoMapper;
using SqlSugar;

namespace {{namespace}}
{
    public class {{entity}}Repository : Repository<{{entity}}>, I{{entity}}Repository
    {
        private readonly IMapper mapper;

        public {{entity}}Repository(ISqlSugarClient db, IMapper mapper) : base(db)
        {
            this.mapper = mapper;
        }

        public async Task<PagingResultDto<{{entity}}Dto>> Get{{entity}}Async(Query{{entity}}Dto query{{entity}}Dto)
        {
            RefAsync<int> total = 0;
            var {{entities | snake}} = await this.Context.Queryable<{{entity}}>()
            //.WhereIF(!string.IsNullOrEmpty(customerQueryDto.CustomerName), u => u.CustomerName.Contains(customerQueryDto.CustomerName))
            //.WhereIF(!string.IsNullOrEmpty(customerQueryDto.Remark), u => u.Remark.Contains(customerQueryDto.Remark))
                .ToPageListAsync(query{{entity}}Dto.PageIndex, query{{entity}}Dto.PageSize, total);
            var {{entities | snake}}Dto = mapper.Map<List<{{entity}}Dto>>({{entities | snake}});
            var pageResult = new PagingResultDto<{{entity}}Dto>()
            {
                TotalCount = total,
                Items ={{entities | snake}}Dto,
            };
            return pageResult;
        }
    }
}