using System;

namespace CodeGeneratorApp.Modules.EntityGenerate.Services
{
    public interface IEntityGeneratorService : IDisposable
    {
        void CreateCreateOrUpdateDto(string dir);
        void CreateDto(string dir);
        void CreateEfRepository(string dir);
        void CreateEntity(string nameSpace, string idType, string name, string dir);
        void CreateException(string exceptionName, string dir);
        void CreateIService(bool isCustom, string dir);
        void CreateManager(string dir);
        void CreatePagedAndSortedAndFilterResultDto(string dir);
        void CreateRepositoryInterface(string dir);
        void CreateService(bool isCustom, string dir);
        void FormatAll();
        void InsertEfcoreEntityConfig(string filePath);
        void InsertMapper(string filePath);
        void SetEntity(string entityPath);
    }
}
