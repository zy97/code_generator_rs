using System;

namespace CodeGeneratorApp.Modules.EntityGenerate.Services
{
    public class EntityGeneratorService : IEntityGeneratorService
    {
        private EntityGenerator entityGenerator;

        internal EntityGeneratorService()
        {
        }

        public void Dispose()
        {
            if (entityGenerator != null)
            {
                entityGenerator.Dispose();
                GC.SuppressFinalize(this);
            }
        }
        public void CreateEntity(string nameSpace, string idType, string name, string dir)
        {
            EntityGenerator.CreateEntity(nameSpace, idType, name, dir);
        }
        public void SetEntity(string entityPath)
        {
            if (entityGenerator != null)
                entityGenerator.Dispose();
            entityGenerator = EntityGenerator.New(entityPath);
        }
        public void CreateDto(string dir)
        {
            entityGenerator?.CreateDto(dir);
        }
        public void CreateCreateOrUpdateDto(string dir)
        {
            entityGenerator?.CreateAddAndModifyDto(dir);
        }
        public void CreatePagedAndSortedAndFilterResultDto(string dir)
        {
            entityGenerator?.CreatePagedandsortedandfilterresultdto(dir);
        }
        public void CreateManager(string dir)
        {
            entityGenerator?.CreateManager(dir);
        }
        public void CreateEfRepository(string dir)
        {
            entityGenerator?.CreateEfRepository(dir);
        }
        public void CreateRepositoryInterface(string dir)
        {
            entityGenerator?.CreateRepositoryInterface(dir);
        }
        public void CreateIService(bool isCustom, string dir)
        {
            entityGenerator?.CreateIservice(isCustom, dir);
        }
        public void CreateService(bool isCustom, string dir)
        {
            entityGenerator?.CreateService(isCustom, dir);
        }
        public void CreateException(string exceptionName, string dir)
        {
            entityGenerator?.CreateException(exceptionName, dir);
        }
        public void InsertMapper(string filePath)
        {
            entityGenerator?.InsertMapper(filePath);
        }
        public void InsertEfcoreEntityConfig(string filePath)
        {
            entityGenerator?.InsertEfcoreEntityConfig(filePath);
        }
        public void FormatAll()
        {
            entityGenerator?.FormatAll();
        }
    }
}
