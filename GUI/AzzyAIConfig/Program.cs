// Program.cs
//
// Programmed by Machiavellian of iRO Chaos
//
// Description:
// This file contains code for the main entry point of the program, as well
// as methods for outputting messages to the console.

using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using System.Windows.Forms;


namespace AzzyAIConfig
{
    static class Program
    {
        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        static void Main()
        {
            // A crash is the moment a report is most useful: the dialog offers the diagnostics
            // bundle and the GitHub issue instead of a wall of text the player has to copy.
            Application.ThreadException += delegate(object sender, System.Threading.ThreadExceptionEventArgs e)
            {
                ShowCrash(e.Exception);
            };
            AppDomain.CurrentDomain.UnhandledException += delegate(object sender, UnhandledExceptionEventArgs e)
            {
                ShowCrash(e.ExceptionObject as Exception);
            };
            try
            {
                Environment.CurrentDirectory = AppDomain.CurrentDomain.BaseDirectory;
                Application.EnableVisualStyles();
                Application.SetCompatibleTextRenderingDefault(false);
                Application.Run(new MainForm());
            }
            catch (Exception e)
            {
                ShowCrash(e);
            }
        }

        static void ShowCrash(Exception e)
        {
            string text = e == null ? "Unknown error." : e.ToString();
            try
            {
                using (DiagnosticsForm f = new DiagnosticsForm(text)) f.ShowDialog();
            }
            catch (Exception)
            {
                MessageBox.Show(text, "AzzyAI error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            }
        }

        public static void WriteLine(string value)
        {
            // Check if the console output stream is not null
            if (Console.Out != null)
            {
                // Get the console output stream
                Stream sout = Console.OpenStandardOutput();

                // Check if the stream can write and read
                if (sout.CanWrite && sout.CanRead)
                {
                    // Write the value to the console
                    Console.WriteLine(value);
                }
            }
        }

        public static void WriteLine(string format, object arg0)
        {
            // Check if the console output stream is not null
            if (Console.Out != null)
            {
                // Get the console output stream
                Stream sout = Console.OpenStandardOutput();

                // Check if the stream can write and read
                if (sout.CanWrite && sout.CanRead)
                {
                    // Write the formatted text with the argument to the console
                    Console.WriteLine(format, arg0);
                }
            }
        }

        public static void WriteLine(string format, object arg0, object arg1)
        {
            // Check if the console output stream is not null
            if (Console.Out != null)
            {
                // Get the console output stream
                Stream sout = Console.OpenStandardOutput();

                // Check if the stream can write and read
                if (sout.CanWrite && sout.CanRead)
                {
                    // Write the formatted text with the arguments to the console
                    Console.WriteLine(format, arg0, arg1);
                }
            }
        }

        public static void WriteLine(string format, object arg0, object arg1, object arg2)
        {
            // Check if the console output stream is not null
            if (Console.Out != null)
            {
                // Get the console output stream
                Stream sout = Console.OpenStandardOutput();

                // Check if the stream can write and read
                if (sout.CanWrite && sout.CanRead)
                {
                    // Write the formatted text with the arguments to the console
                    Console.WriteLine(format, arg0, arg1, arg2);
                }
            }
        }

        public static void WriteLine(string format, object arg0, object arg1, object arg2, object arg3)
        {
            // Check if the console output stream is not null
            if (Console.Out != null)
            {
                // Get the console output stream
                Stream sout = Console.OpenStandardOutput();

                // Check if the stream can write and read
                if (sout.CanWrite && sout.CanRead)
                {
                    // Write the formatted text with the arguments to the console
                    Console.WriteLine(format, arg0, arg1, arg2, arg3);
                }
            }
        }

        public static void WriteLine(string format, params object[] arg)
        {
            // Check if the console output stream is not null
            if (Console.Out != null)
            {
                // Get the console output stream
                Stream sout = Console.OpenStandardOutput();

                // Check if the stream can write and read
                if (sout.CanWrite && sout.CanRead)
                {
                    // Write the formatted text with the arguments to the console
                    Console.WriteLine(format, arg);
                }
            }
        }
    }
}