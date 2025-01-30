using System;
using System.Threading.Tasks;
using Grpc.Net.Client;
using User; // Namespace from the generated code

class Program
{
    static async Task Main()
    {
        using var channel = GrpcChannel.ForAddress("http://localhost:50051", new GrpcChannelOptions { HttpHandler = new SocketsHttpHandler { EnableMultipleHttp2Connections = true } });
        var client = new UsersCtrl.UsersCtrlClient(channel);

        var userRequest = new User.User
        {
            Name = "John Doe",
            Id = 123,
            Email = "john@example.com"
        };

        var response = await client.PostUserAsync(userRequest);

        Console.WriteLine($"Received: {response.Name}, {response.Id}, {response.Email}");
    }
}
