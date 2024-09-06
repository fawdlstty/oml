﻿using RustyOptions;
using System.Runtime.InteropServices;
using System.Text;

namespace OmlSharp;

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

    public static string PtrToStringUTF8(IntPtr ptr)
    {
        if (ptr == IntPtr.Zero)
            return "";
        int length = 0;
        while (Marshal.ReadByte(ptr, length) != 0)
            length++;
        byte[] bytes = new byte[length];
        Marshal.Copy(ptr, bytes, 0, length);
        return Encoding.UTF8.GetString(bytes);
    }
}



public class OmlExpr : IDisposable
{
    public OmlExpr(IntPtr pexpr, string path = "") => (this.pexpr, this.path) = (pexpr, path);
    ~OmlExpr() => do_release();

    public OmlExpr this[int index] => new OmlExpr(pexpr, path.Length > 0 ? $"{path}.[{index}]" : $"[{index}]");
    public OmlExpr this[string index] => new OmlExpr(pexpr, path.Length > 0 ? $"{path}.{index}" : index);


    public static Result<OmlExpr, string> from_str(string src)
    {
        IntPtr pexpr = 0;
        IntPtr perr = 0;
        if (FFI.oml_expr_from_str(src, out pexpr, out perr) > 0)
        {
            return Result.Ok<OmlExpr, string>(new OmlExpr(pexpr));
        }
        else
        {
            string err = FFI.PtrToStringUTF8(perr);
            FFI.oml_release_str(perr);
            return Result.Err<OmlExpr, string>(err);
        }
    }


    public void set_none() { FFI.oml_expr_set_none(pexpr, path); }
    public void set_bool(bool val) { FFI.oml_expr_set_bool(pexpr, path, val ? 1 : 0); }
    public void set_int(long val) { FFI.oml_expr_set_int(pexpr, path, val); }
    public void set_float(double val) { FFI.oml_expr_set_float(pexpr, path, val); }
    public void set_string(string val) { FFI.oml_expr_set_string(pexpr, path, val); }

    public Result<OmlValue, string> evalute()
    {
        IntPtr pval = 0;
        IntPtr perr = 0;
        if (FFI.oml_expr_evalute(pexpr, path, out pval, out perr) > 0)
        {
            return Result.Ok<OmlValue, string>(new OmlValue(pval));
        }
        else
        {
            string err = FFI.PtrToStringUTF8(perr);
            FFI.oml_release_str(perr);
            return Result.Err<OmlValue, string>(err);
        }
    }

    public void Dispose()
    {
        do_release();
        GC.SuppressFinalize(this);
    }

    private void do_release()
    {
        if (pexpr == 0) return;
        FFI.oml_release_expr(pexpr);
        pexpr = 0;
    }

    private IntPtr pexpr = 0;
    private string path = "";
}

public class OmlValue : IDisposable
{
    public OmlValue(IntPtr pval, string path = "") => (this.pval, this.path) = (pval, path);
    ~OmlValue() => do_release();

    public OmlValue this[int index] => new OmlValue(pval, path.Length > 0 ? $"{path}.[{index}]" : $"[{index}]");
    public OmlValue this[string index] => new OmlValue(pval, path.Length > 0 ? $"{path}.{index}" : index);

    public bool is_none() { return FFI.oml_value_is_none(pval, path) > 0; }
    public bool is_bool() { return FFI.oml_value_is_bool(pval, path) > 0; }
    public bool is_int() { return FFI.oml_value_is_int(pval, path) > 0; }
    public bool is_float() { return FFI.oml_value_is_float(pval, path) > 0; }
    public bool is_str() { return FFI.oml_value_is_str(pval, path) > 0; }
    public bool is_array() { return FFI.oml_value_is_array(pval, path) > 0; }
    public bool is_map() { return FFI.oml_value_is_map(pval, path) > 0; }

    public void set_none() { FFI.oml_value_set_none(pval, path); }
    public void set_bool(bool val) { FFI.oml_value_set_bool(pval, path, val ? 1 : 0); }
    public void set_int(long val) { FFI.oml_value_set_int(pval, path, val); }
    public void set_float(double val) { FFI.oml_value_set_float(pval, path, val); }
    public void set_string(string val) { FFI.oml_value_set_string(pval, path, val); }

    public bool as_bool() { return FFI.oml_value_as_bool(pval, path) > 0; }
    public long as_int() { return FFI.oml_value_as_int(pval, path); }
    public double as_float() { return FFI.oml_value_as_float(pval, path); }
    public string as_str()
    {
        IntPtr pstr = FFI.oml_value_as_str(pval, path);
        string ret = FFI.PtrToStringUTF8(pstr);
        FFI.oml_release_str(pstr);
        return ret;
    }
    public int get_array_length() { return FFI.oml_value_get_array_length(pval, path); }
    public int get_map_length() { return FFI.oml_value_get_map_length(pval, path); }
    public List<string> get_map_keys()
    {
        IntPtr pstr = FFI.oml_value_get_keys(pval, path);
        string str = FFI.PtrToStringUTF8(pstr);
        FFI.oml_release_str(pstr);
        return str.Split('#', StringSplitOptions.RemoveEmptyEntries).ToList() ?? new List<string>();
    }

    public void Dispose()
    {
        do_release();
        GC.SuppressFinalize(this);
    }

    private void do_release()
    {
        if (pval == 0) return;
        FFI.oml_release_value(pval);
        pval = 0;
    }

    private IntPtr pval = 0;
    private string path = "";
}