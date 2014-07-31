// Copyright 2013 The gl-rs developers. For a full listing of the authors,
// refer to the AUTHORS file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub fn to_return_suffix(ty: &str) -> String {
    match ty {
        "c_void" | "VOID" | "GLvoid" => "".to_string(),
        ty_str => format!(" -> {}", ty_str.replace("*mut ", "*const ")),
    }
}

/// Converts a C style type definition to the Rust equivalent
pub fn to_rust_ty(ty: &str) -> &'static str {
    match ty {
        // gl.xml types
        "GLDEBUGPROC"               => "GLDEBUGPROC",
        "GLDEBUGPROCAMD"            => "GLDEBUGPROCAMD",
        "GLDEBUGPROCARB"            => "GLDEBUGPROCARB",
        "GLDEBUGPROCKHR"            => "GLDEBUGPROCKHR",
        "GLbitfield"                => "GLbitfield",
        "GLboolean"                 => "GLboolean",
        "GLbyte"                    => "GLbyte",
        "GLclampd"                  => "GLclampd",
        "GLclampf"                  => "GLclampf",
        "GLclampx"                  => "GLclampx",
        "GLdouble"                  => "GLdouble",
        "GLeglImageOES"             => "GLeglImageOES",
        "GLenum"                    => "GLenum",
        "GLfixed"                   => "GLfixed",
        "GLfloat"                   => "GLfloat",
        "GLhalfNV"                  => "GLhalfNV",
        "GLhandleARB"               => "GLhandleARB",
        "GLint"                     => "GLint",
        "GLint64EXT"                => "GLint64EXT",
        "GLintptr"                  => "GLintptr",
        "GLintptrARB"               => "GLintptrARB",
        "GLshort"                   => "GLshort",
        "GLsizei"                   => "GLsizei",
        "GLsizeiptr"                => "GLsizeiptr",
        "GLsizeiptrARB"             => "GLsizeiptrARB",
        "GLsync"                    => "GLsync",
        "GLubyte"                   => "GLubyte",
        "GLuint"                    => "GLuint",
        "GLuint64"                  => "GLuint64",
        "GLuint64EXT"               => "GLuint64EXT",
        "GLushort"                  => "GLushort",
        "GLvdpauSurfaceNV"          => "GLvdpauSurfaceNV",
        "void "                     => "::libc::c_void",
        "GLboolean *"               => "*mut GLboolean",
        "GLchar *"                  => "*mut GLchar",
        "GLcharARB *"               => "*mut GLcharARB",
        "GLdouble *"                => "*mut GLdouble",
        "GLenum *"                  => "*mut GLenum",
        "GLfixed *"                 => "*mut GLfixed",
        "GLfloat *"                 => "*mut GLfloat",
        "GLhandleARB *"             => "*mut GLhandleARB",
        "GLint *"                   => "*mut GLint",
        "GLint64 *"                 => "*mut GLint64",
        "GLint64EXT *"              => "*mut GLint64EXT",
        "GLsizei *"                 => "*mut GLsizei",
        "GLubyte *"                 => "*mut GLubyte",
        "GLuint *"                  => "*mut GLuint",
        "GLuint64 *"                => "*mut GLuint64",
        "GLuint64EXT *"             => "*mut GLuint64EXT",
        "GLushort *"                => "*mut GLushort",
        "GLvoid *"                  => "*mut GLvoid",
        "GLvoid **"                 => "*const *mut GLvoid",
        "void *"                    => "*mut ::libc::c_void",
        "void **"                   => "*const *mut ::libc::c_void",
        "const GLboolean *"         => "*const GLboolean",
        "const GLbyte *"            => "*const GLbyte",
        "const GLchar *"            => "*const GLchar",
        "const GLcharARB *"         => "*const GLcharARB",
        "const GLclampf *"          => "*const GLclampf",
        "const GLdouble *"          => "*const GLdouble",
        "const GLenum *"            => "*const GLenum",
        "const GLfixed *"           => "*const GLfixed",
        "const GLfloat *"           => "*const GLfloat",
        "const GLhalfNV *"          => "*const GLhalfNV",
        "const GLint *"             => "*const GLint",
        "const GLint64EXT *"        => "*const GLint64EXT",
        "const GLintptr *"          => "*const GLintptr",
        "const GLshort *"           => "*const GLshort",
        "const GLsizei *"           => "*const GLsizei",
        "const GLsizeiptr *"        => "*const GLsizeiptr",
        "const GLubyte *"           => "*const GLubyte",
        "const GLuint *"            => "*const GLuint",
        "const GLuint64 *"          => "*const GLuint64",
        "const GLuint64EXT *"       => "*const GLuint64EXT",
        "const GLushort *"          => "*const GLushort",
        "const GLvdpauSurfaceNV *"  => "*const GLvdpauSurfaceNV",
        "const GLvoid *"            => "*const GLvoid",
        "const void *"              => "*const ::libc::c_void",
        "const void **"             => "*const *const ::libc::c_void",
        "const void *const*"        => "*const *const ::libc::c_void",
        "const GLboolean **"        => "*const *const GLboolean",
        "const GLchar **"           => "*const *const GLchar",
        "const GLcharARB **"        => "*const *const GLcharARB",
        "const GLvoid **"           => "*const *const GLvoid",
        "const GLchar *const*"      => "*const *const GLchar",
        "const GLvoid *const*"      => "*const *const GLvoid",
        "struct _cl_context *"      => "*const _cl_context",
        "struct _cl_event *"        => "*const _cl_event",

        // glx.xml types
        "Bool"                      => "Bool",
        "Colormap"                  => "Colormap",
        "DMbuffer"                  => "DMbuffer",
        "Font"                      => "Font",
        "GLXContext"                => "GLXContext",
        "GLXContextID"              => "GLXContextID",
        "GLXDrawable"               => "GLXDrawable",
        "GLXFBConfig"               => "GLXFBConfig",
        "GLXFBConfigSGIX"           => "GLXFBConfigSGIX",
        "GLXPbuffer"                => "GLXPbuffer",
        "GLXPbufferSGIX"            => "GLXPbufferSGIX",
        "GLXPixmap"                 => "GLXPixmap",
        "GLXVideoCaptureDeviceNV"   => "GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV"          => "GLXVideoDeviceNV",
        "GLXVideoSourceSGIX"        => "GLXVideoSourceSGIX",
        "GLXWindow"                 => "GLXWindow",
        // "GLboolean"                 => "GLboolean",
        // "GLenum"                    => "GLenum",
        // "GLint"                     => "GLint",
        // "GLsizei"                   => "GLsizei",
        // "GLuint"                    => "GLuint",
        "Pixmap"                    => "Pixmap",
        "Status"                    => "Status",
        "VLNode"                    => "VLNode",
        "VLPath"                    => "VLPath",
        "VLServer"                  => "VLServer",
        "Window"                    => "Window",
        "__GLXextFuncPtr"           => "__GLXextFuncPtr",
        "const GLXContext"          => "const GLXContext",
        "float "                    => "::libc::c_float",
        "int "                      => "::libc::c_int",
        "int64_t"                   => "i64",
        "unsigned int "             => "::libc::c_uint",
        "unsigned long "            => "::libc::c_ulong",
        // "void "                     => "::libc::c_void",
        "DMparams *"                => "*mut DMparams",
        "Display *"                 => "*mut Display",
        "GLXFBConfig *"             => "*mut GLXFBConfig",
        "GLXFBConfigSGIX *"         => "*mut GLXFBConfigSGIX",
        "GLXHyperpipeConfigSGIX *"  => "*mut GLXHyperpipeConfigSGIX",
        "GLXHyperpipeNetworkSGIX *" => "*mut GLXHyperpipeNetworkSGIX",
        "GLXVideoCaptureDeviceNV *" => "*mut GLXVideoCaptureDeviceNV",
        "GLXVideoDeviceNV *"        => "*mut GLXVideoDeviceNV",
        // "GLuint *"                  => "*mut GLuint",
        "XVisualInfo *"             => "*mut XVisualInfo",
        // "const GLubyte *"           => "*GLubyte",
        "const char *"              => "*const ::libc::c_char",
        "const int *"               => "*const ::libc::c_int",
        // "const void *"              => "*const ::libc::c_void",
        "int *"                     => "*mut ::libc::c_int",
        "int32_t *"                 => "*mut i32",
        "int64_t *"                 => "*mut i64",
        "long *"                    => "*mut ::libc::c_long",
        "unsigned int *"            => "*mut ::libc::c_uint",
        "unsigned long *"           => "*mut ::libc::c_ulong",
        // "void *"                    => "*mut ::libc::c_void",

        // wgl.xml types
        "BOOL"                      => "BOOL",
        "DWORD"                     => "DWORD",
        "FLOAT"                     => "FLOAT",
        // "GLbitfield"                => "GLbitfield",
        // "GLboolean"                 => "GLboolean",
        // "GLenum"                    => "GLenum",
        // "GLfloat"                   => "GLfloat",
        // "GLint"                     => "GLint",
        // "GLsizei"                   => "GLsizei",
        // "GLuint"                    => "GLuint",
        // "GLushort"                  => "GLushort",
        "HANDLE"                    => "HANDLE",
        "HDC"                       => "HDC",
        "HENHMETAFILE"              => "HENHMETAFILE",
        "HGLRC"                     => "HGLRC",
        "HGPUNV"                    => "HGPUNV",
        "HPBUFFERARB"               => "HPBUFFERARB",
        "HPBUFFEREXT"               => "HPBUFFEREXT",
        "HPVIDEODEV"                => "HPVIDEODEV",
        "HVIDEOINPUTDEVICENV"       => "HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV"      => "HVIDEOOUTPUTDEVICENV",
        "INT"                       => "INT",
        "INT64"                     => "INT64",
        "LPCSTR"                    => "LPCSTR",
        "LPGLYPHMETRICSFLOAT"       => "LPGLYPHMETRICSFLOAT",
        "LPVOID"                    => "LPVOID",
        "PGPU_DEVICE"               => "PGPU_DEVICE",
        "PROC"                      => "PROC",
        "UINT"                      => "UINT",
        "VOID"                      => "VOID",
        // "int "                      => "::libc::c_int",
        // "unsigned int "             => "::libc::c_uint",
        // "void "                     => "::libc::c_void",
        "BOOL *"                    => "*mut BOOL",
        "DWORD *"                   => "*mut DWORD",
        "FLOAT *"                   => "*mut FLOAT",
        // "GLuint *"                  => "*mut GLuint",
        "HANDLE *"                  => "*mut HANDLE",
        "HGPUNV *"                  => "*mut HGPUNV",
        "HPVIDEODEV *"              => "*mut HPVIDEODEV",
        "HVIDEOINPUTDEVICENV *"     => "*mut HVIDEOINPUTDEVICENV",
        "HVIDEOOUTPUTDEVICENV *"    => "*mut HVIDEOOUTPUTDEVICENV",
        "INT32 *"                   => "*mut INT32",
        "INT64 *"                   => "*mut INT64",
        "UINT *"                    => "*mut UINT",
        "USHORT *"                  => "*mut USHORT",
        "const COLORREF *"          => "*const COLORREF",
        "const DWORD *"             => "*const DWORD",
        "const FLOAT *"             => "*const FLOAT",
        // "const GLushort *"          => "*const GLushort",
        "const HANDLE *"            => "*const HANDLE",
        "const HGPUNV *"            => "*const HGPUNV",
        "const LAYERPLANEDESCRIPTOR *"  => "*const LAYERPLANEDESCRIPTOR",
        "const LPVOID *"            => "*const LPVOID",
        "const PIXELFORMATDESCRIPTOR *" => "*const PIXELFORMATDESCRIPTOR",
        "const USHORT *"            => "*const USHORT",
        // "const char *"              => "*const ::libc::c_char",
        // "const int *"               => "*const ::libc::c_int",
        "float *"                   => "*mut ::libc::c_float",
        // "int *"                     => "*mut ::libc::c_int",
        // "unsigned long *"           => "*mut ::libc::c_ulong",
        // "void *"                    => "*mut ::libc::c_void",

        // failure
        _ => fail!("Type conversion not implemented for `{}`", ty),
    }
}

pub type Src = &'static [&'static str];

pub static GL_ALIASES: Src = &[
    "// Common types from OpenGL 1.1",
    "pub type GLenum = ::libc::c_uint;",
    "pub type GLboolean = ::libc::c_uchar;",
    "pub type GLbitfield = ::libc::c_uint;",
    "pub type GLvoid = ::libc::c_void;",
    "pub type GLbyte = ::libc::c_char;",
    "pub type GLshort = ::libc::c_short;",
    "pub type GLint = ::libc::c_int;",
    "pub type GLclampx = ::libc::c_int;",
    "pub type GLubyte = ::libc::c_uchar;",
    "pub type GLushort = ::libc::c_ushort;",
    "pub type GLuint = ::libc::c_uint;",
    "pub type GLsizei = ::libc::c_int;",
    "pub type GLfloat = ::libc::c_float;",
    "pub type GLclampf = ::libc::c_float;",
    "pub type GLdouble = ::libc::c_double;",
    "pub type GLclampd = ::libc::c_double;",
    "pub type GLeglImageOES = *const ::libc::c_void;",
    "pub type GLchar = ::libc::c_char;",
    "pub type GLcharARB = ::libc::c_char;",
    "",
    "#[cfg(target_os = \"macos\")]",
    "pub type GLhandleARB = *const ::libc::c_void;",
    "#[cfg(not(target_os = \"macos\"))]",
    "pub type GLhandleARB = ::libc::c_uint;",
    "",
    "pub type GLhalfARB = ::libc::c_ushort;",
    "pub type GLhalf = ::libc::c_ushort;",
    "",
    "// Must be 32 bits",
    "pub type GLfixed = GLint;",
    "",
    "pub type GLintptr = ::libc::ptrdiff_t;",
    "pub type GLsizeiptr = ::libc::ptrdiff_t;",
    "pub type GLint64 = i64;",
    "pub type GLuint64 = u64;",
    "pub type GLintptrARB = ::libc::ptrdiff_t;",
    "pub type GLsizeiptrARB = ::libc::ptrdiff_t;",
    "pub type GLint64EXT = i64;",
    "pub type GLuint64EXT = u64;",
    "",
    "pub struct __GLsync;",
    "pub type GLsync = *const __GLsync;",
    "",
    "// compatible with OpenCL cl_context",
    "pub struct _cl_context;",
    "pub struct _cl_event;",
    "",
    "pub type GLDEBUGPROC = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut ::libc::c_void);",
    "pub type GLDEBUGPROCARB = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut ::libc::c_void);",
    "pub type GLDEBUGPROCKHR = extern \"system\" fn(source: GLenum, gltype: GLenum, id: GLuint, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut ::libc::c_void);",
    "",
    // "// GLES 1 types",
    // "pub type GLclampx = i32;",
    // "",
    // "// GLES 1/2 types (tagged for GLES 1)",
    // "pub type GLbyte = i8;",
    // "pub type GLubyte = u8;",
    // "pub type GLfloat = GLfloat;",
    // "pub type GLclampf = GLfloat;",
    // "pub type GLfixed = i32;",
    // "pub type GLint64 = i64;",
    // "pub type GLuint64 = u64;",
    // "pub type GLintptr = intptr_t;",
    // "pub type GLsizeiptr = ssize_t;",
    // "",
    // "// GLES 1/2 types (tagged for GLES 2 - attribute syntax is limited)",
    // "pub type GLbyte = i8;",
    // "pub type GLubyte = u8;",
    // "pub type GLfloat = GLfloat;",
    // "pub type GLclampf = GLfloat;",
    // "pub type GLfixed = i32;",
    // "pub type GLint64 = i64;",
    // "pub type GLuint64 = u64;",
    // "pub type GLint64EXT = i64;",
    // "pub type GLuint64EXT = u64;",
    // "pub type GLintptr = intptr_t;",
    // "pub type GLsizeiptr = ssize_t;",
    // "",
    // "// GLES 2 types (none currently)",
    // "",
    "// Vendor extension types",
    "pub type GLDEBUGPROCAMD = extern \"system\" fn(id: GLuint, category: GLenum, severity: GLenum, length: GLsizei, message: *const GLchar, userParam: *mut ::libc::c_void);",
    "pub type GLhalfNV = ::libc::c_ushort;",
    "pub type GLvdpauSurfaceNV = GLintptr;",
];

pub static X_ALIASES: Src = &[
    "pub type XID = ::libc::c_ulong;",
    "pub type Bool = ::libc::c_int;         // Not sure if this is correct...",
    "pub struct Display;",
];

pub static GLX_ALIASES: Src = &[
    "pub type GLXFBConfigID = XID;",
    "pub type GLXFBConfig = *const ::libc::c_void;",
    "pub type GLXContextID = XID;",
    "pub type GLXContext = *const ::libc::c_void;",
    "pub type GLXPixmap = XID;",
    "pub type GLXDrawable = XID;",
    "pub type GLXWindow = XID;",
    "pub type GLXPbuffer = XID;",
    "pub type __GLXextFuncPtr = extern \"system\" fn();",
    "pub type GLXVideoCaptureDeviceNV = XID;",
    "pub type GLXVideoDeviceNV = ::libc::c_int;",
    "pub type GLXVideoSourceSGIX = XID;",
    "pub type GLXFBConfigIDSGIX = XID;",
    "pub type GLXFBConfigSGIX = *const ::libc::c_void;",
    "pub type GLXPbufferSGIX = XID;",
    "",
    "pub struct GLXPbufferClobberEvent {",
    "    event_type: ::libc::c_int,          // GLX_DAMAGED or GLX_SAVED",
    "    draw_type: ::libc::c_int,           // GLX_WINDOW or GLX_PBUFFER",
    "    serial: ::libc::c_ulong,            // # of last request processed by server",
    "    send_event: Bool,           // true if this came for SendEvent request",
    "    display: *const Display,          // display the event was read from",
    "    drawable: GLXDrawable,      // XID of Drawable",
    "    buffer_mask: ::libc::c_uint,        // mask indicating which buffers are affected",
    "    aux_buffer: ::libc::c_uint,         // which aux buffer was affected",
    "    x: ::libc::c_int,",
    "    y: ::libc::c_int,",
    "    width: ::libc::c_int,",
    "    height: ::libc::c_int,",
    "    count: ::libc::c_int,               // if nonzero, at least this many more",
    "}",
    "",
    "pub struct GLXBufferSwapComplete {",
    "    type: ::libc::c_int,",
    "    serial: ::libc::c_ulong,            // # of last request processed by server",
    "    send_event: Bool,           // true if this came from a SendEvent request",
    "    display: *const Display,          // Display the event was read from",
    "    drawable: GLXDrawable,      // drawable on which event was requested in event mask",
    "    event_type: ::libc::c_int,",
    "    ust: i64,",
    "    msc: i64,",
    "    sbc: i64,",
    "}",
    "",
    "// typedef union __GLXEvent {",
    "//     GLXPbufferClobberEvent glxpbufferclobber;",
    "//     GLXBufferSwapComplete glxbufferswapcomplete;",
    "//     long pad[24];",
    "// } GLXEvent;",
    "",
    "pub struct GLXBufferClobberEventSGIX {",
    "    type: ::libc::c_int,",
    "    serial: ::libc::c_ulong,            // # of last request processed by server",
    "    send_event: Bool,           // true if this came for SendEvent request",
    "    display: *const Display,          // display the event was read from",
    "    drawable: GLXDrawable,      // i.d. of Drawable",
    "    event_type: ::libc::c_int,          // GLX_DAMAGED_SGIX or GLX_SAVED_SGIX",
    "    draw_type: ::libc::c_int,           // GLX_WINDOW_SGIX or GLX_PBUFFER_SGIX",
    "    mask: ::libc::c_uint,               // mask indicating which buffers are affected",
    "    x: ::libc::c_int,",
    "    y: ::libc::c_int,",
    "    width: ::libc::c_int,",
    "    height: ::libc::c_int,",
    "    count: ::libc::c_int,               // if nonzero, at least this many more",
    "}",
    "",
    "pub struct GLXHyperpipeNetworkSGIX {",
    "    pipeName: [::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]",
    "    networkId: ::libc::c_int,",
    "}",
    "",
    "pub struct GLXHyperpipeConfigSGIX {",
    "    pipeName: [::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]",
    "    channel: ::libc::c_int,",
    "    participationType: ::libc::c_uint,",
    "    timeSlice: ::libc::c_int,",
    "}",
    "",
    "pub struct GLXPipeRect {",
    "    pipeName: [::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]",
    "    srcXOrigin: ::libc::c_int,",
    "    srcYOrigin: ::libc::c_int,",
    "    srcWidth: ::libc::c_int,",
    "    srcHeight: ::libc::c_int,",
    "    destXOrigin: ::libc::c_int,",
    "    destYOrigin: ::libc::c_int,",
    "    destWidth: ::libc::c_int,",
    "    destHeight: ::libc::c_int,",
    "}",
    "",
    "pub struct GLXPipeRectLimits {",
    "    pipeName: [::libc::c_char, ..80],   // Should be [GLX_HYPERPIPE_PIPE_NAME_LENGTH_SGIX]",
    "    XOrigin: ::libc::c_int,",
    "    YOrigin: ::libc::c_int,",
    "    maxHeight: ::libc::c_int,",
    "    maxWidth: ::libc::c_int,",
    "}",
];

pub static WIN_ALIASES: Src = &[
    "// From WinNT.h",
    "pub type CHAR = ::libc::c_char;",
    "pub type HANDLE = PVOID;",
    "pub type LONG = ::libc::c_long;",
    "pub type LPCSTR = *const ::libc::c_char;",
    "pub type VOID = ::libc::c_void;",
    "",
    "// From Windef.h",
    "pub type BOOL = ::libc::c_int;",
    "pub type BYTE = ::libc::c_uchar;",
    "pub type COLORREF = DWORD;",
    "pub type FLOAT = ::libc::c_float;",
    "pub type HDC = HANDLE;",
    "pub type HENHMETAFILE = HANDLE;",
    "pub type HGLRC = *const ::libc::c_void;",
    "pub type INT = ::libc::c_int;",
    "pub type LPVOID = *const ::libc::c_void;",
    "pub type PROC = extern \"system\" fn();     // Not sure about this one :/",
    "pub struct RECT {",
    "    left: LONG,",
    "    top: LONG,",
    "    right: LONG,",
    "    bottom: LONG,",
    "}",
    "pub type UINT = ::libc::c_uint;",
    "pub type USHORT = ::libc::c_ushort;",
    "pub type WORD = ::libc::c_ushort;",
    "",
    "// From BaseTsd.h",
    "pub type INT32 = i32;",
    "pub type INT64 = i64;",
    "",
    "// From IntSafe.h",
    "pub type DWORD = ::libc::c_ulong;",
    "",
    "// From Wingdi.h",
    "pub struct POINTFLOAT {",
    "    x: FLOAT,",
    "    y: FLOAT,",
    "} ",
    "pub struct GLYPHMETRICSFLOAT {",
    "    gmfBlackBoxX: FLOAT,",
    "    gmfBlackBoxY: FLOAT,",
    "    gmfptGlyphOrigin: POINTFLOAT,",
    "    gmfCellIncX: FLOAT,",
    "    gmfCellIncY: FLOAT,",
    "}",
    "pub type LPGLYPHMETRICSFLOAT = *const GLYPHMETRICSFLOAT;",
    "pub struct LAYERPLANEDESCRIPTOR {",
    "    nSize: WORD,",
    "    nVersion: WORD,",
    "    dwFlags: DWORD,",
    "    iPixelType: BYTE,",
    "    cColorBits: BYTE,",
    "    cRedBits: BYTE,",
    "    cRedShift: BYTE,",
    "    cGreenBits: BYTE,",
    "    cGreenShift: BYTE,",
    "    cBlueBits: BYTE,",
    "    cBlueShift: BYTE,",
    "    cAlphaBits: BYTE,",
    "    cAlphaShift: BYTE,",
    "    cAccumBits: BYTE,",
    "    cAccumRedBits: BYTE,",
    "    cAccumGreenBits: BYTE,",
    "    cAccumBlueBits: BYTE,",
    "    cAccumAlphaBits: BYTE,",
    "    cDepthBits: BYTE,",
    "    cStencilBits: BYTE,",
    "    cAuxBuffers: BYTE,",
    "    iLayerType: BYTE,",
    "    bReserved: BYTE,",
    "    crTransparent: COLORREF,",
    "}",
    "pub struct PIXELFORMATDESCRIPTOR {",
    "    nSize: WORD,",
    "    nVersion: WORD,",
    "    dwFlags: DWORD,",
    "    iPixelType: BYTE,",
    "    cColorBits: BYTE,",
    "    cRedBits: BYTE,",
    "    cRedShift: BYTE,",
    "    cGreenBits: BYTE,",
    "    cGreenShift: BYTE,",
    "    cBlueBits: BYTE,",
    "    cBlueShift: BYTE,",
    "    cAlphaBits: BYTE,",
    "    cAlphaShift: BYTE,",
    "    cAccumBits: BYTE,",
    "    cAccumRedBits: BYTE,",
    "    cAccumGreenBits: BYTE,",
    "    cAccumBlueBits: BYTE,",
    "    cAccumAlphaBits: BYTE,",
    "    cDepthBits: BYTE,",
    "    cStencilBits: BYTE,",
    "    cAuxBuffers: BYTE,",
    "    iLayerType: BYTE,",
    "    bReserved: BYTE,",
    "    dwLayerMask: DWORD,",
    "    dwVisibleMask: DWORD,",
    "    dwDamageMask: DWORD,",
    "}",
];

pub static WGL_ALIASES: Src = &[
    "// From WinNT.h",
    "// #define DECLARE_HANDLE(name) struct name##__{int unused;}; typedef struct name##__ *name",
    "macro_rules! DECLARE_HANDLE(",
    "    ($name:ident) => (",
    "        pub type $name = *const ::libc::c_void;",
    "    )",
    ")",
    "",
    "pub struct _GPU_DEVICE {",
    "    cb: DWORD,",
    "    DeviceName: [CHAR, ..32],",
    "    DeviceString: [CHAR, ..128],",
    "    Flags: DWORD,",
    "    rcVirtualScreen: RECT,",
    "}",
    "DECLARE_HANDLE!(HPBUFFERARB)",
    "DECLARE_HANDLE!(HPBUFFEREXT)",
    "DECLARE_HANDLE!(HVIDEOOUTPUTDEVICENV)",
    "DECLARE_HANDLE!(HPVIDEODEV)",
    "DECLARE_HANDLE!(HPGPUNV)",
    "DECLARE_HANDLE!(HGPUNV)",
    "DECLARE_HANDLE!(HVIDEOINPUTDEVICENV)",
    "pub struct GPU_DEVICE(_GPU_DEVICE);",
    "pub struct PGPU_DEVICE(*const _GPU_DEVICE);",
];
