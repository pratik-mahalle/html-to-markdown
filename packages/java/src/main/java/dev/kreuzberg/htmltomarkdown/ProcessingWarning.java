package dev.kreuzberg.htmltomarkdown;

import com.fasterxml.jackson.annotation.JsonProperty;

/**
 * A non-fatal processing warning produced during conversion.
 *
 * @param message human-readable description of the warning
 * @param kind the warning category (e.g., "malformed_html", "encoding_fallback")
 */
public record ProcessingWarning(
    @JsonProperty("message") String message, @JsonProperty("kind") String kind) { }
