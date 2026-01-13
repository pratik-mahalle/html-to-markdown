using System.Runtime.InteropServices;
using System.Text;

namespace HtmlToMarkdown.Visitor;

/// <summary>
/// Internal bridge for marshalling between C# visitor pattern and C FFI.
/// Callback methods are generated in the partial class VisitorBridgeGenerated.cs.
/// </summary>
internal partial class VisitorBridge : IDisposable
{
    private static class NativeMethods
    {
        public const string LibraryName = "html_to_markdown_ffi";

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr html_to_markdown_visitor_create(IntPtr visitor);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        public static extern void html_to_markdown_visitor_free(IntPtr visitor);

        [DllImport(LibraryName, CallingConvention = CallingConvention.Cdecl)]
        public static extern IntPtr html_to_markdown_last_error();
    }

    private readonly IVisitor _visitor;
    private readonly GCHandle _gcHandle;
    private IntPtr _nativeVisitorHandle = IntPtr.Zero;

    private readonly Dictionary<string, Delegate> _delegateCache = new();

    public VisitorBridge(IVisitor visitor)
    {
        _visitor = visitor ?? throw new ArgumentNullException(nameof(visitor));
        _gcHandle = GCHandle.Alloc(this);
    }

    public IntPtr CreateNativeVisitor()
    {
        var visitTextCb = (NativeVisitorStructures.VisitTextCallback)VisitTextCallback;
        var visitElementStartCb = (NativeVisitorStructures.VisitElementStartCallback)VisitElementStartCallback;
        var visitElementEndCb = (NativeVisitorStructures.VisitElementEndCallback)VisitElementEndCallback;
        var visitLinkCb = (NativeVisitorStructures.VisitLinkCallback)VisitLinkCallback;
        var visitImageCb = (NativeVisitorStructures.VisitImageCallback)VisitImageCallback;
        var visitHeadingCb = (NativeVisitorStructures.VisitHeadingCallback)VisitHeadingCallback;
        var visitCodeBlockCb = (NativeVisitorStructures.VisitCodeBlockCallback)VisitCodeBlockCallback;
        var visitCodeInlineCb = (NativeVisitorStructures.VisitCodeInlineCallback)VisitCodeInlineCallback;
        var visitListItemCb = (NativeVisitorStructures.VisitListItemCallback)VisitListItemCallback;
        var visitListStartCb = (NativeVisitorStructures.VisitListStartCallback)VisitListStartCallback;
        var visitListEndCb = (NativeVisitorStructures.VisitListEndCallback)VisitListEndCallback;
        var visitTableStartCb = (NativeVisitorStructures.VisitTableStartCallback)VisitTableStartCallback;
        var visitTableRowCb = (NativeVisitorStructures.VisitTableRowCallback)VisitTableRowCallback;
        var visitTableEndCb = (NativeVisitorStructures.VisitTableEndCallback)VisitTableEndCallback;
        var visitBlockquoteCb = (NativeVisitorStructures.VisitBlockquoteCallback)VisitBlockquoteCallback;
        var visitStrongCb = (NativeVisitorStructures.VisitStrongCallback)VisitStrongCallback;
        var visitEmphasisCb = (NativeVisitorStructures.VisitEmphasisCallback)VisitEmphasisCallback;
        var visitStrikethroughCb = (NativeVisitorStructures.VisitStrikethroughCallback)VisitStrikethroughCallback;
        var visitUnderlineCb = (NativeVisitorStructures.VisitUnderlineCallback)VisitUnderlineCallback;
        var visitSubscriptCb = (NativeVisitorStructures.VisitSubscriptCallback)VisitSubscriptCallback;
        var visitSuperscriptCb = (NativeVisitorStructures.VisitSuperscriptCallback)VisitSuperscriptCallback;
        var visitMarkCb = (NativeVisitorStructures.VisitMarkCallback)VisitMarkCallback;
        var visitLineBreakCb = (NativeVisitorStructures.VisitLineBreakCallback)VisitLineBreakCallback;
        var visitHorizontalRuleCb = (NativeVisitorStructures.VisitHorizontalRuleCallback)VisitHorizontalRuleCallback;
        var visitCustomElementCb = (NativeVisitorStructures.VisitCustomElementCallback)VisitCustomElementCallback;
        var visitDefinitionListStartCb = (NativeVisitorStructures.VisitDefinitionListStartCallback)VisitDefinitionListStartCallback;
        var visitDefinitionTermCb = (NativeVisitorStructures.VisitDefinitionTermCallback)VisitDefinitionTermCallback;
        var visitDefinitionDescriptionCb = (NativeVisitorStructures.VisitDefinitionDescriptionCallback)VisitDefinitionDescriptionCallback;
        var visitDefinitionListEndCb = (NativeVisitorStructures.VisitDefinitionListEndCallback)VisitDefinitionListEndCallback;
        var visitFormCb = (NativeVisitorStructures.VisitFormCallback)VisitFormCallback;
        var visitInputCb = (NativeVisitorStructures.VisitInputCallback)VisitInputCallback;
        var visitButtonCb = (NativeVisitorStructures.VisitButtonCallback)VisitButtonCallback;
        var visitAudioCb = (NativeVisitorStructures.VisitAudioCallback)VisitAudioCallback;
        var visitVideoCb = (NativeVisitorStructures.VisitVideoCallback)VisitVideoCallback;
        var visitIFrameCb = (NativeVisitorStructures.VisitIFrameCallback)VisitIFrameCallback;
        var visitDetailsCb = (NativeVisitorStructures.VisitDetailsCallback)VisitDetailsCallback;
        var visitSummaryCb = (NativeVisitorStructures.VisitSummaryCallback)VisitSummaryCallback;
        var visitFigureStartCb = (NativeVisitorStructures.VisitFigureStartCallback)VisitFigureStartCallback;
        var visitFigCaptionCb = (NativeVisitorStructures.VisitFigCaptionCallback)VisitFigCaptionCallback;
        var visitFigureEndCb = (NativeVisitorStructures.VisitFigureEndCallback)VisitFigureEndCallback;

        _delegateCache["VisitText"] = visitTextCb;
        _delegateCache["VisitElementStart"] = visitElementStartCb;
        _delegateCache["VisitElementEnd"] = visitElementEndCb;
        _delegateCache["VisitLink"] = visitLinkCb;
        _delegateCache["VisitImage"] = visitImageCb;
        _delegateCache["VisitHeading"] = visitHeadingCb;
        _delegateCache["VisitCodeBlock"] = visitCodeBlockCb;
        _delegateCache["VisitCodeInline"] = visitCodeInlineCb;
        _delegateCache["VisitListItem"] = visitListItemCb;
        _delegateCache["VisitListStart"] = visitListStartCb;
        _delegateCache["VisitListEnd"] = visitListEndCb;
        _delegateCache["VisitTableStart"] = visitTableStartCb;
        _delegateCache["VisitTableRow"] = visitTableRowCb;
        _delegateCache["VisitTableEnd"] = visitTableEndCb;
        _delegateCache["VisitBlockquote"] = visitBlockquoteCb;
        _delegateCache["VisitStrong"] = visitStrongCb;
        _delegateCache["VisitEmphasis"] = visitEmphasisCb;
        _delegateCache["VisitStrikethrough"] = visitStrikethroughCb;
        _delegateCache["VisitUnderline"] = visitUnderlineCb;
        _delegateCache["VisitSubscript"] = visitSubscriptCb;
        _delegateCache["VisitSuperscript"] = visitSuperscriptCb;
        _delegateCache["VisitMark"] = visitMarkCb;
        _delegateCache["VisitLineBreak"] = visitLineBreakCb;
        _delegateCache["VisitHorizontalRule"] = visitHorizontalRuleCb;
        _delegateCache["VisitCustomElement"] = visitCustomElementCb;
        _delegateCache["VisitDefinitionListStart"] = visitDefinitionListStartCb;
        _delegateCache["VisitDefinitionTerm"] = visitDefinitionTermCb;
        _delegateCache["VisitDefinitionDescription"] = visitDefinitionDescriptionCb;
        _delegateCache["VisitDefinitionListEnd"] = visitDefinitionListEndCb;
        _delegateCache["VisitForm"] = visitFormCb;
        _delegateCache["VisitInput"] = visitInputCb;
        _delegateCache["VisitButton"] = visitButtonCb;
        _delegateCache["VisitAudio"] = visitAudioCb;
        _delegateCache["VisitVideo"] = visitVideoCb;
        _delegateCache["VisitIFrame"] = visitIFrameCb;
        _delegateCache["VisitDetails"] = visitDetailsCb;
        _delegateCache["VisitSummary"] = visitSummaryCb;
        _delegateCache["VisitFigureStart"] = visitFigureStartCb;
        _delegateCache["VisitFigCaption"] = visitFigCaptionCb;
        _delegateCache["VisitFigureEnd"] = visitFigureEndCb;

        var nativeVisitor = new NativeVisitorStructures.NativeVisitor
        {
            UserData = GCHandle.ToIntPtr(_gcHandle),
            VisitText = Marshal.GetFunctionPointerForDelegate(visitTextCb),
            VisitElementStart = Marshal.GetFunctionPointerForDelegate(visitElementStartCb),
            VisitElementEnd = Marshal.GetFunctionPointerForDelegate(visitElementEndCb),
            VisitLink = Marshal.GetFunctionPointerForDelegate(visitLinkCb),
            VisitImage = Marshal.GetFunctionPointerForDelegate(visitImageCb),
            VisitHeading = Marshal.GetFunctionPointerForDelegate(visitHeadingCb),
            VisitCodeBlock = Marshal.GetFunctionPointerForDelegate(visitCodeBlockCb),
            VisitCodeInline = Marshal.GetFunctionPointerForDelegate(visitCodeInlineCb),
            VisitListItem = Marshal.GetFunctionPointerForDelegate(visitListItemCb),
            VisitListStart = Marshal.GetFunctionPointerForDelegate(visitListStartCb),
            VisitListEnd = Marshal.GetFunctionPointerForDelegate(visitListEndCb),
            VisitTableStart = Marshal.GetFunctionPointerForDelegate(visitTableStartCb),
            VisitTableRow = Marshal.GetFunctionPointerForDelegate(visitTableRowCb),
            VisitTableEnd = Marshal.GetFunctionPointerForDelegate(visitTableEndCb),
            VisitBlockquote = Marshal.GetFunctionPointerForDelegate(visitBlockquoteCb),
            VisitStrong = Marshal.GetFunctionPointerForDelegate(visitStrongCb),
            VisitEmphasis = Marshal.GetFunctionPointerForDelegate(visitEmphasisCb),
            VisitStrikethrough = Marshal.GetFunctionPointerForDelegate(visitStrikethroughCb),
            VisitUnderline = Marshal.GetFunctionPointerForDelegate(visitUnderlineCb),
            VisitSubscript = Marshal.GetFunctionPointerForDelegate(visitSubscriptCb),
            VisitSuperscript = Marshal.GetFunctionPointerForDelegate(visitSuperscriptCb),
            VisitMark = Marshal.GetFunctionPointerForDelegate(visitMarkCb),
            VisitLineBreak = Marshal.GetFunctionPointerForDelegate(visitLineBreakCb),
            VisitHorizontalRule = Marshal.GetFunctionPointerForDelegate(visitHorizontalRuleCb),
            VisitCustomElement = Marshal.GetFunctionPointerForDelegate(visitCustomElementCb),
            VisitDefinitionListStart = Marshal.GetFunctionPointerForDelegate(visitDefinitionListStartCb),
            VisitDefinitionTerm = Marshal.GetFunctionPointerForDelegate(visitDefinitionTermCb),
            VisitDefinitionDescription = Marshal.GetFunctionPointerForDelegate(visitDefinitionDescriptionCb),
            VisitDefinitionListEnd = Marshal.GetFunctionPointerForDelegate(visitDefinitionListEndCb),
            VisitForm = Marshal.GetFunctionPointerForDelegate(visitFormCb),
            VisitInput = Marshal.GetFunctionPointerForDelegate(visitInputCb),
            VisitButton = Marshal.GetFunctionPointerForDelegate(visitButtonCb),
            VisitAudio = Marshal.GetFunctionPointerForDelegate(visitAudioCb),
            VisitVideo = Marshal.GetFunctionPointerForDelegate(visitVideoCb),
            VisitIFrame = Marshal.GetFunctionPointerForDelegate(visitIFrameCb),
            VisitDetails = Marshal.GetFunctionPointerForDelegate(visitDetailsCb),
            VisitSummary = Marshal.GetFunctionPointerForDelegate(visitSummaryCb),
            VisitFigureStart = Marshal.GetFunctionPointerForDelegate(visitFigureStartCb),
            VisitFigCaption = Marshal.GetFunctionPointerForDelegate(visitFigCaptionCb),
            VisitFigureEnd = Marshal.GetFunctionPointerForDelegate(visitFigureEndCb),
        };

        var nativePtr = Marshal.AllocHGlobal(Marshal.SizeOf<NativeVisitorStructures.NativeVisitor>());
        Marshal.StructureToPtr(nativeVisitor, nativePtr, false);

        try
        {
            _nativeVisitorHandle = NativeMethods.html_to_markdown_visitor_create(nativePtr);
            if (_nativeVisitorHandle == IntPtr.Zero)
            {
                throw new HtmlToMarkdownException("Failed to create native visitor");
            }

            return _nativeVisitorHandle;
        }
        finally
        {
            Marshal.FreeHGlobal(nativePtr);
        }
    }

    public IntPtr GetNativeVisitorHandle() => _nativeVisitorHandle;

    private NodeContext UnmarshalNodeContext(IntPtr ctx)
    {
        var nativeCtx = Marshal.PtrToStructure<NativeVisitorStructures.NativeNodeContext>(ctx);

        var attributes = new List<Attribute>();
        if (nativeCtx.Attributes != IntPtr.Zero)
        {
            var attrPtr = nativeCtx.Attributes;
            var attrSize = Marshal.SizeOf<NativeVisitorStructures.NativeAttribute>();

            while (true)
            {
                var attr = Marshal.PtrToStructure<NativeVisitorStructures.NativeAttribute>(attrPtr);
                if (attr.Key == IntPtr.Zero || attr.Value == IntPtr.Zero)
                {
                    break;
                }

                var key = PtrToStringUtf8(attr.Key);
                var value = PtrToStringUtf8(attr.Value);
                attributes.Add(new Attribute(key ?? "", value ?? ""));

                attrPtr = IntPtr.Add(attrPtr, attrSize);
            }
        }

        var nodeType = (NodeType)nativeCtx.NodeType;
        var tagName = PtrToStringUtf8(nativeCtx.TagName) ?? "";
        var parentTag = nativeCtx.ParentTag != IntPtr.Zero ? PtrToStringUtf8(nativeCtx.ParentTag) : null;

        return new NodeContext(
            nodeType,
            tagName,
            attributes.AsReadOnly(),
            (int)nativeCtx.Depth,
            (int)nativeCtx.IndexInParent,
            parentTag,
            nativeCtx.IsInline);
    }

    private string? PtrToStringUtf8(IntPtr ptr)
    {
        if (ptr == IntPtr.Zero)
        {
            return null;
        }

        try
        {
            return Marshal.PtrToStringUTF8(ptr);
        }
        catch (AccessViolationException)
        {
            return string.Empty;
        }
    }

    private IntPtr StringToUtf8Ptr(string value)
    {
        if (string.IsNullOrEmpty(value))
        {
            return IntPtr.Zero;
        }

        var bytes = Encoding.UTF8.GetBytes(value);
        var ptr = Marshal.AllocHGlobal(bytes.Length + 1);
        Marshal.Copy(bytes, 0, ptr, bytes.Length);
        Marshal.WriteByte(ptr, bytes.Length, 0);
        return ptr;
    }

    private NativeVisitorStructures.NativeVisitResult MarshalVisitResult(VisitResult result)
    {
        var nativeResult = new NativeVisitorStructures.NativeVisitResult
        {
            ResultType = (NativeVisitorStructures.NativeVisitResultType)result.ResultType,
            CustomOutput = IntPtr.Zero,
            ErrorMessage = IntPtr.Zero
        };

        if (result is CustomResult customResult)
        {
            nativeResult.CustomOutput = StringToUtf8Ptr(customResult.CustomOutput);
        }
        else if (result is ErrorResult errorResult)
        {
            nativeResult.ErrorMessage = StringToUtf8Ptr(errorResult.ErrorMessage);
        }

        return nativeResult;
    }

    // Callback methods are generated in the partial class VisitorBridgeGenerated.cs

    public void Dispose()
    {
        if (_nativeVisitorHandle != IntPtr.Zero)
        {
            NativeMethods.html_to_markdown_visitor_free(_nativeVisitorHandle);
            _nativeVisitorHandle = IntPtr.Zero;
        }

        if (_gcHandle.IsAllocated)
        {
            _gcHandle.Free();
        }
    }
}
