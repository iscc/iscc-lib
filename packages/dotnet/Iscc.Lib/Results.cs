// Structured result types for ISCC code generation functions.
// Each gen_*_v0 function returns a dedicated result type carrying the ISCC code string
// plus any additional fields matching the Rust core's result struct definitions.

namespace Iscc.Lib;

/// <summary>Result of GenMetaCodeV0.</summary>
public sealed record MetaCodeResult(string Iscc);

/// <summary>Result of GenTextCodeV0.</summary>
public sealed record TextCodeResult(string Iscc);

/// <summary>Result of GenImageCodeV0.</summary>
public sealed record ImageCodeResult(string Iscc);

/// <summary>Result of GenAudioCodeV0.</summary>
public sealed record AudioCodeResult(string Iscc);

/// <summary>Result of GenVideoCodeV0.</summary>
public sealed record VideoCodeResult(string Iscc);

/// <summary>Result of GenMixedCodeV0.</summary>
public sealed record MixedCodeResult(string Iscc);

/// <summary>Result of GenDataCodeV0.</summary>
public sealed record DataCodeResult(string Iscc);

/// <summary>Result of GenInstanceCodeV0.</summary>
public sealed record InstanceCodeResult(string Iscc);

/// <summary>Result of GenIsccCodeV0.</summary>
public sealed record IsccCodeResult(string Iscc);

/// <summary>Result of GenSumCodeV0 — composite ISCC-CODE with file metadata.</summary>
public sealed record SumCodeResult(
    string Iscc,
    string Datahash,
    ulong Filesize,
    string[]? Units);

/// <summary>Result of IsccDecode — decoded ISCC header fields and raw digest bytes.</summary>
public sealed record DecodeResult(
    byte Maintype, byte Subtype, byte Version, byte Length, byte[] Digest);
