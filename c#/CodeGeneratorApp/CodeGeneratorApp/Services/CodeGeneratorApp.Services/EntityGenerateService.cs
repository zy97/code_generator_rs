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
            entityGenerator = EntityGenerator.New(entityPath);
        }
    }
}
