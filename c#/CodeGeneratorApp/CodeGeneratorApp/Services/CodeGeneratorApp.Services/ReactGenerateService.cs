using CodeGeneratorApp.Services.Interfaces;

namespace CodeGeneratorApp.Services
{
    public class ReactGenerateService : IReactGenerateService
    {
        private ReactGenerator reactGenerator;

        public ReactGenerateService()
        {

        }
        public void SetEntity(string entityPath)
        {
            if (reactGenerator != null)
                reactGenerator.Dispose();
            reactGenerator = ReactGenerator.New(entityPath);
        }
        public void Format()
        {
            if (reactGenerator != null)
                reactGenerator.FormatAll();
        }
        public void CreateApi(string prefix)
        {
            if (reactGenerator != null)
                reactGenerator.CreateApi(prefix);
        }
        public void CreateStore()
        {
            if (reactGenerator != null)
                reactGenerator.CreateStore();
        }
        public void CreatePage()
        {
            if (reactGenerator != null)
                reactGenerator.CreatePage();
        }
        public void Dispose()
        {
            if (reactGenerator != null)
                reactGenerator.Dispose();
        }
    }
}
