using Microsoft.AspNetCore.Mvc;
using WES.API.IServices;
using WES.Entity.Model;
using WES.Entity.Dto.{{entities}};
using WES.Services.IServices;

namespace WES.API.Controllers
{
    [Route("[controller]/[action]")]
    [ApiController]
    public class {{entity}}Controller : ControllerBase
    {
        private readonly I{{entity}}Service {{entity|snake}}Service;
        private readonly IExcelOperatore excelOperatore;

        public {{entity}}Controller(I{{entity}}Service {{entity|snake}}Service, IExcelOperatore excelOperatore)
        {
            this.{{entity|snake}}Service = {{entity|snake}}Service;
            this.excelOperatore = excelOperatore;
        }

        /// <summary>
        /// 添加
        /// </summary>
        /// <param name="{{entity|snake}}"></param>
        /// <returns></returns>
        [HttpPost]
        public async Task<IActionResult> Add{{entity}}Async(Create{{entity}}Dto {{entity|snake}})
        {
            var res = await {{entity|snake}}Service.Add{{entity}}Async({{entity|snake}});
            return Ok(res);
        }

        /// <summary>
        ///  修改
        /// </summary>
        /// <param name="{{entity|snake}}"></param>
        /// <returns></returns>
        [HttpPost]
        public async Task<IActionResult> Update{{entity}}Async(Update{{entity}}Dto {{entity|snake}})
        {
            var res = await {{entity|snake}}Service.Update{{entity}}Async({{entity|snake}});
            return Ok(res);
        }

        //删除
        [HttpPost]
        public async Task<IActionResult> Delete{{entity}}Async(DeleteDto deleteDto)
        {
            var res = await {{entity|snake}}Service.Delete{{entity}}Async(deleteDto.Id);
            return Ok(res);
        }

        //查询
        [HttpGet]
        public async Task<PagingResultDto<{{entity}}Dto>> Get{{entity}}Async([FromQuery] Query{{entity}}Dto query{{entity}}Dto)
        {
            var res = await {{entity|snake}}Service.Get{{entities}}Async(query{{entity}}Dto);
            return res;
        }
        //导出
        [HttpGet()]
        public async Task<IActionResult> Export{{entity}}Async([FromQuery] Query{{entity}}Dto query{{entity}}Dto)
        {
            var roles = await {{entity|snake}}Service.Get{{entities}}Async(query{{entity}}Dto);
            var dataBytes = excelOperatore.Save(roles.Items, "客户数据表");
            //application/vnd.openxmlformats-officedocument.spreadsheetml.sheet
            return File(dataBytes, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "{{entity|snake}}.xlsx");
        }
    }
}
