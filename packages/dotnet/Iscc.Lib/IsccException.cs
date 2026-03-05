// Exception type for ISCC library errors.

namespace Iscc.Lib;

/// <summary>Exception thrown when an ISCC library operation fails.</summary>
public class IsccException : Exception
{
    /// <summary>Create a new IsccException with the specified error message.</summary>
    public IsccException(string message) : base(message) { }
}
