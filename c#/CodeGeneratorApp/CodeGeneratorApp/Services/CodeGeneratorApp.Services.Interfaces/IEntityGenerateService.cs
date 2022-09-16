using System;

namespace CodeGeneratorApp.Services.Interfaces
{
    public interface IEntityGenerateService : IDisposable
    {
        void CreateCreateOrUpdateDto();
        void CreateDto();
        void CreateEfRepository();
        void CreateException(string exceptionName, string exceptionCode, string exceptionDisplayName);
        void CreateIService(bool isCustom);
        void CreateManager();
        void CreatePagedAndSortedAndFilterResultDto();
        void CreateRepositoryInterface();
        void CreateService(bool isCustom);
        void FormatAll();
        void InsertEfcoreEntityConfig();
        void InsertMapper();
        void SetEntity(string entityPath);
    }
}
