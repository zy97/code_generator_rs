using CodeGeneratorApp.Services.Interfaces;

namespace CodeGeneratorApp.Services
{
    public class MessageService : IMessageService
    {
        public string GetMessage()
        {
            return "Hello from the Message Service";
        }
    }
}
