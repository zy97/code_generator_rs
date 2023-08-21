{% let class_names = class_name|pluralize %}
{%- let class_name_camel = class_name|camel %}
{%- let class_names_camel = class_names|camel -%}
using Microsoft.AspNetCore.Mvc;
using WES.API.IServices;
using WES.Entity.Model;
using WES.Entity.Model.{{class_names}};
using WES.Services.IServices;

namespace {{namespace}}
{
    [Route("[controller]/[action]")]
    [ApiController]
    public class {{class_name}}Controller : ControllerBase
    {
        private readonly I{{class_name}}Service {{class_name_camel}}Service;
        private readonly IExcelOperatore excelOperatore;

        public {{class_name}}Controller(I{{class_name}}Service {{class_name_camel}}Service, IExcelOperatore excelOperatore)
        {
            this.{{class_name_camel}}Service = {{class_name_camel}}Service;
            this.excelOperatore = excelOperatore;
        }

        /// <summary>
        /// 添加
        /// </summary>
        /// <param name="{{class_name_camel}}"></param>
        /// <returns></returns>
        [HttpPost]
        public async Task<IActionResult> Add{{class_name}}Async(Create{{class_name}}Dto {{class_name_camel}})
        {
            var res = await {{class_name_camel}}Service.Add{{class_name}}Async({{class_name_camel}});
            return Ok(res);
        }

        /// <summary>
        ///  修改
        /// </summary>
        /// <param name="{{class_name_camel}}"></param>
        /// <returns></returns>
        [HttpPost]
        public async Task<IActionResult> Update{{class_name}}Async(Update{{class_name}}Dto {{class_name_camel}})
        {
            var res = await {{class_name_camel}}Service.Update{{class_name}}Async({{class_name_camel}});
            return Ok(res);
        }

        //删除
        [HttpPost]
        public async Task<IActionResult> Delete{{class_name}}Async(OnlyIdDto deleteDto)
        {
            var res = await {{class_name_camel}}Service.Delete{{class_name}}Async(deleteDto.Id);
            return Ok(res);
        }

        //查询
        [HttpGet]
        public async Task<PagingResultDto<{{class_name}}Dto>> Get{{class_names}}Async([FromQuery] Query{{class_name}}Dto query{{class_name}}Dto)
        {
            var res = await {{class_name_camel}}Service.Get{{class_names}}Async(query{{class_name}}Dto);
            return res;
        }
        //导出
        [HttpGet()]
        public async Task<IActionResult> Export{{class_names}}Async([FromQuery] Query{{class_name}}Dto query{{class_name}}Dto)
        {
            var items = await {{class_name_camel}}Service.Get{{class_names}}Async(query{{class_name}}Dto);
            var stream = excelOperatore.Save(items.Items, "数据表");
            return File(stream, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet", "{{class_name_camel}}.xlsx");
        }
    }
}
