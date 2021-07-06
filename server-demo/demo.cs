using System;
using AMWD.Modbus.Serial.Server;
using AMWD.Modbus.Serial.Protocol;
using System.Threading.Tasks;
using AMWD.Modbus.Serial;
using System.Threading;
using Microsoft.Extensions.Logging.Console;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using System.IO.Ports;
using Serilog;
using AMWD.Modbus.Common.Util;

namespace ModbusServerTest
{
    class Program
    {

        static async Task Main(string[] args)
        {
            var services = new ServiceCollection();
            ConfigureServices(services);
            var provider = services.BuildServiceProvider();

            var logger = provider.GetService<ILogger<Program>>();
            var server = new ModbusServer("COM4",  logger, Handler)
            {
                BaudRate = BaudRate.Baud9600,
                StopBits = StopBits.One,
                DataBits = 8,
                Parity = Parity.None,
                Handshake = Handshake.None,                
            };
            await server.Initialization;
            Console.WriteLine($"Server is running {server.IsRunning}");

            /*
            var port = new SerialPort("COM7", 115200, Parity.None, 8, StopBits.One);
            port.DataReceived += Port_DataReceived;
            port.Open();
            */

            Thread.Sleep(Timeout.Infinite);
            provider.Dispose();
        }

        static void ConfigureServices(ServiceCollection services)
        {
            services.AddLogging(logger =>
            {
                logger.ClearProviders();
                logger.AddConsole();
            });
        }

        private static void Port_DataReceived(object sender, SerialDataReceivedEventArgs e)
        {
            Console.WriteLine("Received data");
        }

        private static readonly Random rnd = new();
        public static Response Handler(Request request) {
            var response = new Response(request);
            if(request.Function == AMWD.Modbus.Common.FunctionCode.ReadCoils)
            {
                response.Data = new DataBuffer();
                int tempbyte = 0;
                for (int i = 0; i < request.Count; i++)
                {
                    //var value = rnd.Next(0, 2) == 1 ? 1 : 0;
                    var value = 1;
                    tempbyte |= (value << i);
                }
                response.Data.AddByte((byte)tempbyte);
            }
            Console.WriteLine(response.ToString());
            foreach (var item in response.Serialize())
                Console.Write($"{item} ");
            Console.WriteLine();
            return response;
        }
    }
}
