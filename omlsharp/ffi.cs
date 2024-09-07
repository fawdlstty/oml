using System.Reflection;
using System.Runtime.InteropServices;

namespace oml;

public static class FFI
{
    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_expr_from_str([MarshalAs(UnmanagedType.LPStr)] string psrc, out IntPtr ppexpr, out IntPtr pperr);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_expr_set_none(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_expr_set_bool(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, int value);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_expr_set_int(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, long value);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_expr_set_float(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, double value);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_expr_set_string(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, [MarshalAs(UnmanagedType.LPStr)] string pvalue);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_expr_evalute(IntPtr pexpr, [MarshalAs(UnmanagedType.LPStr)] string ppath, out IntPtr ppval, out IntPtr pperr);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_is_none(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_is_bool(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_as_bool(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_is_int(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern long oml_value_as_int(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_is_float(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern double oml_value_as_float(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_is_str(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr oml_value_as_str(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_is_array(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_get_array_length(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_is_map(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_get_map_length(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr oml_value_get_keys(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_set_none(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_value_set_bool(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath, int value);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_value_set_int(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath, long value);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_value_set_float(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath, double value);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern int oml_value_set_string(IntPtr pval, [MarshalAs(UnmanagedType.LPStr)] string ppath, [MarshalAs(UnmanagedType.LPStr)] string pvalue);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_release_expr(IntPtr pexpr);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_release_value(IntPtr pval);

    [DllImport("oml.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void oml_release_str(IntPtr pstr);

    static FFI()
    {
        IntPtr DllImportResolver(string libraryName, Assembly assembly, DllImportSearchPath? searchPath)
        {
            if (libraryName == "oml.dll")
            {
                if (Environment.Is64BitProcess)
                {
                    if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
                    {
                        return NativeLibrary.Load("libs/oml.x86_64_win.dll", assembly, searchPath);
                    }
                    else if (RuntimeInformation.IsOSPlatform(OSPlatform.Linux))
                    {
                        return NativeLibrary.Load("libs/liboml.x86_64_linux.so", assembly, searchPath);
                    }
                }
                throw new Exception("unsupported platform");
            }
            return IntPtr.Zero;
        }
        NativeLibrary.SetDllImportResolver(Assembly.GetExecutingAssembly(), DllImportResolver);
    }
}