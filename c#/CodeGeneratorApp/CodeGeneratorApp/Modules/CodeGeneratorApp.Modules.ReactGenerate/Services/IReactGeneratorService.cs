using System;

namespace CodeGeneratorApp.Modules.ReactGenerate.Services
{
    public interface IReactGeneratorService : IDisposable
    {
        void CreateApi(string prefix);
        void CreatePage();
        void CreateStore();
        void Format();
        void SetEntity(string entityPath);
    }
}
