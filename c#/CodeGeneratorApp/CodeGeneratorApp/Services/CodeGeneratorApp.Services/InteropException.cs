using System;

namespace CodeGeneratorApp.Services
{
    public class InteropException<T> : Exception
    {
        public T Error { get; private set; }

        public InteropException(T error) : base($"Something went wrong: {error}")
        {
            Error = error;
        }
    }
}
