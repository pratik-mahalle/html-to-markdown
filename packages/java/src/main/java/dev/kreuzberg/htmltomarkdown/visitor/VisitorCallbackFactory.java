package dev.kreuzberg.htmltomarkdown.visitor;

import dev.kreuzberg.htmltomarkdown.util.StringUtils;
import java.lang.foreign.Arena;
import java.lang.foreign.FunctionDescriptor;
import java.lang.foreign.Linker;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandle;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.MethodType;
import java.util.List;

/**
 * Factory for creating native visitor callback stubs.
 *
 * <p>Creates Panama FFI upcall stubs that bridge Java Visitor methods to C callbacks.
 *
 * @since 2.17.0
 */
public final class VisitorCallbackFactory {

  /** Number of callback function pointers in the visitor struct (including user_data). */
  private static final int CALLBACK_STRUCT_FIELD_COUNT = 39;

  /** Size of a pointer on this platform. */
  private static final long POINTER_SIZE = ValueLayout.ADDRESS.byteSize();

  /** Native linker instance. */
  private static final Linker LINKER = Linker.nativeLinker();

  /** Lookup for method handles. */
  private static final MethodHandles.Lookup LOOKUP = MethodHandles.lookup();

  /** Function descriptor for callbacks with just context: fn(user_data, ctx) -> result. */
  private static final FunctionDescriptor CTX_ONLY_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG, // result (encoded)
          ValueLayout.ADDRESS, // user_data
          ValueLayout.ADDRESS // ctx
          );

  /** Function descriptor for callbacks with context + 1 string. */
  private static final FunctionDescriptor CTX_STR_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG, ValueLayout.ADDRESS, ValueLayout.ADDRESS, ValueLayout.ADDRESS);

  /** Function descriptor for callbacks with context + 2 strings. */
  private static final FunctionDescriptor CTX_2STR_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS);

  /** Function descriptor for callbacks with context + 3 strings. */
  private static final FunctionDescriptor CTX_3STR_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS);

  /** Function descriptor for callbacks with context + 4 strings. */
  private static final FunctionDescriptor CTX_4STR_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS);

  /** Function descriptor for heading callback: ctx, level (u32), text, id. */
  private static final FunctionDescriptor HEADING_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.JAVA_INT,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS);

  /** Function descriptor for list item: ctx, ordered (bool), marker, text. */
  private static final FunctionDescriptor LIST_ITEM_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.JAVA_BOOLEAN,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS);

  /** Function descriptor for list start/end with bool: ctx, ordered. */
  private static final FunctionDescriptor LIST_BOOL_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.JAVA_BOOLEAN);

  /** Function descriptor for list end: ctx, ordered, output. */
  private static final FunctionDescriptor LIST_END_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.JAVA_BOOLEAN,
          ValueLayout.ADDRESS);

  /** Function descriptor for table row: ctx, cells, cell_count, is_header. */
  private static final FunctionDescriptor TABLE_ROW_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.JAVA_LONG,
          ValueLayout.JAVA_BOOLEAN);

  /** Function descriptor for blockquote: ctx, content, depth. */
  private static final FunctionDescriptor BLOCKQUOTE_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.JAVA_LONG);

  /** Function descriptor for details: ctx, open (bool). */
  private static final FunctionDescriptor DETAILS_DESC =
      FunctionDescriptor.of(
          ValueLayout.JAVA_LONG,
          ValueLayout.ADDRESS,
          ValueLayout.ADDRESS,
          ValueLayout.JAVA_BOOLEAN);

  /** The visitor bridge instance. */
  private final VisitorBridge bridge;

  /** Arena for allocations. */
  private final Arena arena;

  /**
   * Create a new factory for generating callback stubs.
   *
   * @param visitor the Java visitor implementation
   * @param arena the arena for memory allocations
   */
  public VisitorCallbackFactory(final Visitor visitor, final Arena arena) {
    this.bridge = new VisitorBridge(visitor, arena);
    this.arena = arena;
  }

  /**
   * Create the native visitor callbacks struct.
   *
   * <p>Allocates a C struct containing function pointers for all visitor callbacks. The returned
   * memory segment is valid for the lifetime of the arena.
   *
   * @return memory segment containing the callbacks struct
   */
  public MemorySegment createCallbacksStruct() {
    // Allocate struct with 39 pointer-sized fields
    long structSize = CALLBACK_STRUCT_FIELD_COUNT * POINTER_SIZE;
    MemorySegment callbacks = arena.allocate(structSize);

    // user_data - store bridge reference (we use address 0 for simplicity)
    callbacks.set(ValueLayout.ADDRESS, 0, MemorySegment.NULL);

    try {
      // Index 1: visit_text
      callbacks.set(ValueLayout.ADDRESS, 1 * POINTER_SIZE, createVisitTextStub());
      // Index 2: visit_element_start
      callbacks.set(ValueLayout.ADDRESS, 2 * POINTER_SIZE, createVisitElementStartStub());
      // Index 3: visit_element_end
      callbacks.set(ValueLayout.ADDRESS, 3 * POINTER_SIZE, createVisitElementEndStub());
      // Index 4: visit_link
      callbacks.set(ValueLayout.ADDRESS, 4 * POINTER_SIZE, createVisitLinkStub());
      // Index 5: visit_image
      callbacks.set(ValueLayout.ADDRESS, 5 * POINTER_SIZE, createVisitImageStub());
      // Index 6: visit_heading
      callbacks.set(ValueLayout.ADDRESS, 6 * POINTER_SIZE, createVisitHeadingStub());
      // Index 7: visit_code_block
      callbacks.set(ValueLayout.ADDRESS, 7 * POINTER_SIZE, createVisitCodeBlockStub());
      // Index 8: visit_code_inline
      callbacks.set(ValueLayout.ADDRESS, 8 * POINTER_SIZE, createVisitCodeInlineStub());
      // Index 9: visit_list_item
      callbacks.set(ValueLayout.ADDRESS, 9 * POINTER_SIZE, createVisitListItemStub());
      // Index 10: visit_list_start
      callbacks.set(ValueLayout.ADDRESS, 10 * POINTER_SIZE, createVisitListStartStub());
      // Index 11: visit_list_end
      callbacks.set(ValueLayout.ADDRESS, 11 * POINTER_SIZE, createVisitListEndStub());
      // Index 12: visit_table_start
      callbacks.set(ValueLayout.ADDRESS, 12 * POINTER_SIZE, createVisitTableStartStub());
      // Index 13: visit_table_row
      callbacks.set(ValueLayout.ADDRESS, 13 * POINTER_SIZE, createVisitTableRowStub());
      // Index 14: visit_table_end
      callbacks.set(ValueLayout.ADDRESS, 14 * POINTER_SIZE, createVisitTableEndStub());
      // Index 15: visit_blockquote
      callbacks.set(ValueLayout.ADDRESS, 15 * POINTER_SIZE, createVisitBlockquoteStub());
      // Index 16: visit_strong
      callbacks.set(ValueLayout.ADDRESS, 16 * POINTER_SIZE, createVisitStrongStub());
      // Index 17: visit_emphasis
      callbacks.set(ValueLayout.ADDRESS, 17 * POINTER_SIZE, createVisitEmphasisStub());
      // Index 18: visit_strikethrough
      callbacks.set(ValueLayout.ADDRESS, 18 * POINTER_SIZE, createVisitStrikethroughStub());
      // Index 19: visit_underline
      callbacks.set(ValueLayout.ADDRESS, 19 * POINTER_SIZE, createVisitUnderlineStub());
      // Index 20: visit_subscript
      callbacks.set(ValueLayout.ADDRESS, 20 * POINTER_SIZE, createVisitSubscriptStub());
      // Index 21: visit_superscript
      callbacks.set(ValueLayout.ADDRESS, 21 * POINTER_SIZE, createVisitSuperscriptStub());
      // Index 22: visit_mark
      callbacks.set(ValueLayout.ADDRESS, 22 * POINTER_SIZE, createVisitMarkStub());
      // Index 23: visit_line_break
      callbacks.set(ValueLayout.ADDRESS, 23 * POINTER_SIZE, createVisitLineBreakStub());
      // Index 24: visit_horizontal_rule
      callbacks.set(ValueLayout.ADDRESS, 24 * POINTER_SIZE, createVisitHorizontalRuleStub());
      // Index 25: visit_custom_element
      callbacks.set(ValueLayout.ADDRESS, 25 * POINTER_SIZE, createVisitCustomElementStub());
      // Index 26: visit_definition_list_start
      callbacks.set(ValueLayout.ADDRESS, 26 * POINTER_SIZE, createVisitDefinitionListStartStub());
      // Index 27: visit_definition_term
      callbacks.set(ValueLayout.ADDRESS, 27 * POINTER_SIZE, createVisitDefinitionTermStub());
      // Index 28: visit_definition_description
      callbacks.set(ValueLayout.ADDRESS, 28 * POINTER_SIZE, createVisitDefinitionDescriptionStub());
      // Index 29: visit_definition_list_end
      callbacks.set(ValueLayout.ADDRESS, 29 * POINTER_SIZE, createVisitDefinitionListEndStub());
      // Index 30: visit_form
      callbacks.set(ValueLayout.ADDRESS, 30 * POINTER_SIZE, createVisitFormStub());
      // Index 31: visit_input
      callbacks.set(ValueLayout.ADDRESS, 31 * POINTER_SIZE, createVisitInputStub());
      // Index 32: visit_button
      callbacks.set(ValueLayout.ADDRESS, 32 * POINTER_SIZE, createVisitButtonStub());
      // Index 33: visit_audio
      callbacks.set(ValueLayout.ADDRESS, 33 * POINTER_SIZE, createVisitAudioStub());
      // Index 34: visit_video
      callbacks.set(ValueLayout.ADDRESS, 34 * POINTER_SIZE, createVisitVideoStub());
      // Index 35: visit_iframe
      callbacks.set(ValueLayout.ADDRESS, 35 * POINTER_SIZE, createVisitIframeStub());
      // Index 36: visit_details
      callbacks.set(ValueLayout.ADDRESS, 36 * POINTER_SIZE, createVisitDetailsStub());
      // Index 37: visit_summary
      callbacks.set(ValueLayout.ADDRESS, 37 * POINTER_SIZE, createVisitSummaryStub());
      // Index 38: visit_figure_start
      callbacks.set(ValueLayout.ADDRESS, 38 * POINTER_SIZE, createVisitFigureStartStub());
      // Note: figure_caption and figure_end would be 39 and 40, but struct only has 39 fields
      // The C struct may have a different layout - for now we'll handle what we can

    } catch (Exception e) {
      throw new RuntimeException("Failed to create visitor callback stubs", e);
    }

    return callbacks;
  }

  // Callback implementations that delegate to Java visitor

  private long visitText(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result = bridge.getVisitor().visitText(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitElementStart(MemorySegment userData, MemorySegment ctx) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    VisitResult result = bridge.getVisitor().visitElementStart(nodeCtx);
    return bridge.encodeResult(result);
  }

  private long visitElementEnd(MemorySegment userData, MemorySegment ctx, MemorySegment output) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String outputStr = StringUtils.fromCString(output);
    VisitResult result =
        bridge.getVisitor().visitElementEnd(nodeCtx, outputStr != null ? outputStr : "");
    return bridge.encodeResult(result);
  }

  private long visitLink(
      MemorySegment userData,
      MemorySegment ctx,
      MemorySegment href,
      MemorySegment text,
      MemorySegment title) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String hrefStr = StringUtils.fromCString(href);
    String textStr = StringUtils.fromCString(text);
    String titleStr = StringUtils.fromCString(title);
    VisitResult result =
        bridge
            .getVisitor()
            .visitLink(
                nodeCtx, hrefStr != null ? hrefStr : "", textStr != null ? textStr : "", titleStr);
    return bridge.encodeResult(result);
  }

  private long visitImage(
      MemorySegment userData,
      MemorySegment ctx,
      MemorySegment src,
      MemorySegment alt,
      MemorySegment title) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String srcStr = StringUtils.fromCString(src);
    String altStr = StringUtils.fromCString(alt);
    String titleStr = StringUtils.fromCString(title);
    VisitResult result =
        bridge
            .getVisitor()
            .visitImage(
                nodeCtx, srcStr != null ? srcStr : "", altStr != null ? altStr : "", titleStr);
    return bridge.encodeResult(result);
  }

  private long visitHeading(
      MemorySegment userData, MemorySegment ctx, int level, MemorySegment text, MemorySegment id) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    String idStr = StringUtils.fromCString(id);
    VisitResult result =
        bridge.getVisitor().visitHeading(nodeCtx, level, textStr != null ? textStr : "", idStr);
    return bridge.encodeResult(result);
  }

  private long visitCodeBlock(
      MemorySegment userData, MemorySegment ctx, MemorySegment lang, MemorySegment code) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String langStr = StringUtils.fromCString(lang);
    String codeStr = StringUtils.fromCString(code);
    VisitResult result =
        bridge.getVisitor().visitCodeBlock(nodeCtx, langStr, codeStr != null ? codeStr : "");
    return bridge.encodeResult(result);
  }

  private long visitCodeInline(MemorySegment userData, MemorySegment ctx, MemorySegment code) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String codeStr = StringUtils.fromCString(code);
    VisitResult result =
        bridge.getVisitor().visitCodeInline(nodeCtx, codeStr != null ? codeStr : "");
    return bridge.encodeResult(result);
  }

  private long visitListItem(
      MemorySegment userData,
      MemorySegment ctx,
      boolean ordered,
      MemorySegment marker,
      MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String markerStr = StringUtils.fromCString(marker);
    String textStr = StringUtils.fromCString(text);
    VisitResult result =
        bridge
            .getVisitor()
            .visitListItem(
                nodeCtx,
                ordered,
                markerStr != null ? markerStr : "",
                textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitListStart(MemorySegment userData, MemorySegment ctx, boolean ordered) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    VisitResult result = bridge.getVisitor().visitListStart(nodeCtx, ordered);
    return bridge.encodeResult(result);
  }

  private long visitListEnd(
      MemorySegment userData, MemorySegment ctx, boolean ordered, MemorySegment output) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String outputStr = StringUtils.fromCString(output);
    VisitResult result =
        bridge.getVisitor().visitListEnd(nodeCtx, ordered, outputStr != null ? outputStr : "");
    return bridge.encodeResult(result);
  }

  private long visitTableStart(MemorySegment userData, MemorySegment ctx) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    VisitResult result = bridge.getVisitor().visitTableStart(nodeCtx);
    return bridge.encodeResult(result);
  }

  private long visitTableRow(
      MemorySegment userData,
      MemorySegment ctx,
      MemorySegment cells,
      long cellCount,
      boolean isHeader) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    List<String> cellList = bridge.parseCells(cells, (int) cellCount);
    VisitResult result = bridge.getVisitor().visitTableRow(nodeCtx, cellList, isHeader);
    return bridge.encodeResult(result);
  }

  private long visitTableEnd(MemorySegment userData, MemorySegment ctx, MemorySegment output) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String outputStr = StringUtils.fromCString(output);
    VisitResult result =
        bridge.getVisitor().visitTableEnd(nodeCtx, outputStr != null ? outputStr : "");
    return bridge.encodeResult(result);
  }

  private long visitBlockquote(
      MemorySegment userData, MemorySegment ctx, MemorySegment content, long depth) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String contentStr = StringUtils.fromCString(content);
    String contentValue = contentStr != null ? contentStr : "";
    VisitResult result = bridge.getVisitor().visitBlockquote(nodeCtx, contentValue, (int) depth);
    return bridge.encodeResult(result);
  }

  private long visitStrong(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result = bridge.getVisitor().visitStrong(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitEmphasis(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result = bridge.getVisitor().visitEmphasis(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitStrikethrough(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result =
        bridge.getVisitor().visitStrikethrough(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitUnderline(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result =
        bridge.getVisitor().visitUnderline(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitSubscript(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result =
        bridge.getVisitor().visitSubscript(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitSuperscript(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result =
        bridge.getVisitor().visitSuperscript(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitMark(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result = bridge.getVisitor().visitMark(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitLineBreak(MemorySegment userData, MemorySegment ctx) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    VisitResult result = bridge.getVisitor().visitLineBreak(nodeCtx);
    return bridge.encodeResult(result);
  }

  private long visitHorizontalRule(MemorySegment userData, MemorySegment ctx) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    VisitResult result = bridge.getVisitor().visitHorizontalRule(nodeCtx);
    return bridge.encodeResult(result);
  }

  private long visitCustomElement(
      MemorySegment userData, MemorySegment ctx, MemorySegment tagName, MemorySegment html) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String tagNameStr = StringUtils.fromCString(tagName);
    String htmlStr = StringUtils.fromCString(html);
    VisitResult result =
        bridge
            .getVisitor()
            .visitCustomElement(
                nodeCtx, tagNameStr != null ? tagNameStr : "", htmlStr != null ? htmlStr : "");
    return bridge.encodeResult(result);
  }

  private long visitDefinitionListStart(MemorySegment userData, MemorySegment ctx) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    VisitResult result = bridge.getVisitor().visitDefinitionListStart(nodeCtx);
    return bridge.encodeResult(result);
  }

  private long visitDefinitionTerm(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result =
        bridge.getVisitor().visitDefinitionTerm(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitDefinitionDescription(
      MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result =
        bridge.getVisitor().visitDefinitionDescription(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitDefinitionListEnd(
      MemorySegment userData, MemorySegment ctx, MemorySegment output) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String outputStr = StringUtils.fromCString(output);
    VisitResult result =
        bridge.getVisitor().visitDefinitionListEnd(nodeCtx, outputStr != null ? outputStr : "");
    return bridge.encodeResult(result);
  }

  private long visitForm(
      MemorySegment userData, MemorySegment ctx, MemorySegment action, MemorySegment method) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String actionStr = StringUtils.fromCString(action);
    String methodStr = StringUtils.fromCString(method);
    VisitResult result = bridge.getVisitor().visitForm(nodeCtx, actionStr, methodStr);
    return bridge.encodeResult(result);
  }

  private long visitInput(
      MemorySegment userData,
      MemorySegment ctx,
      MemorySegment inputType,
      MemorySegment name,
      MemorySegment value) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String inputTypeStr = StringUtils.fromCString(inputType);
    String nameStr = StringUtils.fromCString(name);
    String valueStr = StringUtils.fromCString(value);
    VisitResult result =
        bridge
            .getVisitor()
            .visitInput(nodeCtx, inputTypeStr != null ? inputTypeStr : "", nameStr, valueStr);
    return bridge.encodeResult(result);
  }

  private long visitButton(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result = bridge.getVisitor().visitButton(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitAudio(MemorySegment userData, MemorySegment ctx, MemorySegment src) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String srcStr = StringUtils.fromCString(src);
    VisitResult result = bridge.getVisitor().visitAudio(nodeCtx, srcStr);
    return bridge.encodeResult(result);
  }

  private long visitVideo(MemorySegment userData, MemorySegment ctx, MemorySegment src) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String srcStr = StringUtils.fromCString(src);
    VisitResult result = bridge.getVisitor().visitVideo(nodeCtx, srcStr);
    return bridge.encodeResult(result);
  }

  private long visitIframe(MemorySegment userData, MemorySegment ctx, MemorySegment src) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String srcStr = StringUtils.fromCString(src);
    VisitResult result = bridge.getVisitor().visitIframe(nodeCtx, srcStr);
    return bridge.encodeResult(result);
  }

  private long visitDetails(MemorySegment userData, MemorySegment ctx, boolean open) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    VisitResult result = bridge.getVisitor().visitDetails(nodeCtx, open);
    return bridge.encodeResult(result);
  }

  private long visitSummary(MemorySegment userData, MemorySegment ctx, MemorySegment text) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    String textStr = StringUtils.fromCString(text);
    VisitResult result = bridge.getVisitor().visitSummary(nodeCtx, textStr != null ? textStr : "");
    return bridge.encodeResult(result);
  }

  private long visitFigureStart(MemorySegment userData, MemorySegment ctx) {
    NodeContext nodeCtx = bridge.parseNodeContext(ctx);
    VisitResult result = bridge.getVisitor().visitFigureStart(nodeCtx);
    return bridge.encodeResult(result);
  }

  // Upcall stub creation methods

  private MemorySegment createVisitTextStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitText",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitElementStartStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitElementStart",
            MethodType.methodType(long.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_ONLY_DESC, arena);
  }

  private MemorySegment createVisitElementEndStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitElementEnd",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitLinkStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitLink",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_4STR_DESC, arena);
  }

  private MemorySegment createVisitImageStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitImage",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_4STR_DESC, arena);
  }

  private MemorySegment createVisitHeadingStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitHeading",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                int.class,
                MemorySegment.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, HEADING_DESC, arena);
  }

  private MemorySegment createVisitCodeBlockStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitCodeBlock",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_2STR_DESC, arena);
  }

  private MemorySegment createVisitCodeInlineStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitCodeInline",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitListItemStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitListItem",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                boolean.class,
                MemorySegment.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, LIST_ITEM_DESC, arena);
  }

  private MemorySegment createVisitListStartStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitListStart",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, boolean.class));
    return LINKER.upcallStub(mh, LIST_BOOL_DESC, arena);
  }

  private MemorySegment createVisitListEndStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitListEnd",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                boolean.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, LIST_END_DESC, arena);
  }

  private MemorySegment createVisitTableStartStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitTableStart",
            MethodType.methodType(long.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_ONLY_DESC, arena);
  }

  private MemorySegment createVisitTableRowStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitTableRow",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                long.class,
                boolean.class));
    return LINKER.upcallStub(mh, TABLE_ROW_DESC, arena);
  }

  private MemorySegment createVisitTableEndStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitTableEnd",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitBlockquoteStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitBlockquote",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                long.class));
    return LINKER.upcallStub(mh, BLOCKQUOTE_DESC, arena);
  }

  private MemorySegment createVisitStrongStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitStrong",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitEmphasisStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitEmphasis",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitStrikethroughStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitStrikethrough",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitUnderlineStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitUnderline",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitSubscriptStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitSubscript",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitSuperscriptStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitSuperscript",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitMarkStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitMark",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitLineBreakStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitLineBreak",
            MethodType.methodType(long.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_ONLY_DESC, arena);
  }

  private MemorySegment createVisitHorizontalRuleStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitHorizontalRule",
            MethodType.methodType(long.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_ONLY_DESC, arena);
  }

  private MemorySegment createVisitCustomElementStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitCustomElement",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_2STR_DESC, arena);
  }

  private MemorySegment createVisitDefinitionListStartStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitDefinitionListStart",
            MethodType.methodType(long.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_ONLY_DESC, arena);
  }

  private MemorySegment createVisitDefinitionTermStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitDefinitionTerm",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitDefinitionDescriptionStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitDefinitionDescription",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitDefinitionListEndStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitDefinitionListEnd",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitFormStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitForm",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_2STR_DESC, arena);
  }

  private MemorySegment createVisitInputStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitInput",
            MethodType.methodType(
                long.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class,
                MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_3STR_DESC, arena);
  }

  private MemorySegment createVisitButtonStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitButton",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitAudioStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitAudio",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitVideoStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitVideo",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitIframeStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitIframe",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitDetailsStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitDetails",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, boolean.class));
    return LINKER.upcallStub(mh, DETAILS_DESC, arena);
  }

  private MemorySegment createVisitSummaryStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitSummary",
            MethodType.methodType(
                long.class, MemorySegment.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_STR_DESC, arena);
  }

  private MemorySegment createVisitFigureStartStub() throws Exception {
    MethodHandle mh =
        LOOKUP.bind(
            this,
            "visitFigureStart",
            MethodType.methodType(long.class, MemorySegment.class, MemorySegment.class));
    return LINKER.upcallStub(mh, CTX_ONLY_DESC, arena);
  }
}
