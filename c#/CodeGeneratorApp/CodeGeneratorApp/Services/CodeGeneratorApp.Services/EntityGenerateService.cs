using CodeGeneratorApp.Services.Interfaces;

namespace CodeGeneratorApp.Services
{
    public class EntityGenerateService : IEntityGenerateService
    {
        private EntityGenerator entityGenerator;

        public EntityGenerateService()
        {
        }

        public void Dispose()
        {
            if (entityGenerator != null)
                entityGenerator.Dispose();
        }

        public void SetEntity(string entityPath)
        {
            if (entityGenerator != null)
                entityGenerator.Dispose();
            entityGenerator = EntityGenerator.New(entityPath);
        }
        public void CreateDto()
        {
            entityGenerator?.CreateDto();
        }
        public void CreateCreateOrUpdateDto()
        {
            entityGenerator?.CreateCreateorupdatedto();
        }
        public void CreatePagedAndSortedAndFilterResultDto()
        {
            entityGenerator?.CreatePagedandsortedandfilterresultdto();
        }
        public void CreateManager()
        {
            entityGenerator?.CreateManager();
        }
        public void CreateEfRepository()
        {
            entityGenerator?.CreateEfRepository();
        }
        public void CreateRepositoryInterface()
        {
            entityGenerator?.CreateRepositoryInterface();
        }
        public void CreateIService(bool isCustom)
        {
            entityGenerator?.CreateIservice(isCustom);
        }
        public void CreateService(bool isCustom)
        {
            entityGenerator?.CreateService(isCustom);
        }
        public void CreateException(string exceptionName, string exceptionCode, string exceptionDisplayName)
        {
            entityGenerator?.CreateException(exceptionName, exceptionCode, exceptionDisplayName);
        }
        public void InsertMapper()
        {
            entityGenerator?.InsertMapper();
        }
        public void InsertEfcoreEntityConfig()
        {
            entityGenerator?.InsertEfcoreEntityConfig();
        }
        public void FormatAll()
        {
            entityGenerator?.FormatAll();
        }
    }
}
