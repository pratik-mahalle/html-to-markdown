# html-to-markdown-ffi CMake config-mode find module
#
# Defines the imported target:
#   html-to-markdown-ffi::html-to-markdown-ffi
#
# Usage:
#   find_package(html-to-markdown-ffi REQUIRED)
#   target_link_libraries(myapp PRIVATE html-to-markdown-ffi::html-to-markdown-ffi)

if(TARGET html-to-markdown-ffi::html-to-markdown-ffi)
  return()
endif()

get_filename_component(_HTM_FFI_CMAKE_DIR "${CMAKE_CURRENT_LIST_FILE}" PATH)
get_filename_component(_HTM_FFI_PREFIX "${_HTM_FFI_CMAKE_DIR}/.." ABSOLUTE)

# ── Step 1: Find the library and headers ──────────────────────────────

find_library(_HTM_FFI_LIBRARY
  NAMES html_to_markdown_ffi libhtml_to_markdown_ffi
  PATHS "${_HTM_FFI_PREFIX}/lib"
  NO_DEFAULT_PATH
)

if(NOT _HTM_FFI_LIBRARY)
  find_library(_HTM_FFI_LIBRARY
    NAMES html_to_markdown_ffi libhtml_to_markdown_ffi
  )
endif()

find_path(_HTM_FFI_INCLUDE_DIR
  NAMES html_to_markdown.h
  PATHS "${_HTM_FFI_PREFIX}/include"
  NO_DEFAULT_PATH
)

if(NOT _HTM_FFI_INCLUDE_DIR)
  find_path(_HTM_FFI_INCLUDE_DIR NAMES html_to_markdown.h)
endif()

# ── Step 2: Validate that required files were found ───────────────────

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(html-to-markdown-ffi
  REQUIRED_VARS _HTM_FFI_LIBRARY _HTM_FFI_INCLUDE_DIR
)

# ── Step 3: Create the imported target with correct library type ──────

if(html-to-markdown-ffi_FOUND)
  # Determine library type from the file extension
  set(_HTM_FFI_LIB_TYPE UNKNOWN)

  if(_HTM_FFI_LIBRARY MATCHES "\\.(dylib|so)$" OR _HTM_FFI_LIBRARY MATCHES "\\.so\\.")
    set(_HTM_FFI_LIB_TYPE SHARED)
  elseif(_HTM_FFI_LIBRARY MATCHES "\\.dll$")
    set(_HTM_FFI_LIB_TYPE SHARED)
  elseif(_HTM_FFI_LIBRARY MATCHES "\\.(a|lib)$")
    set(_HTM_FFI_LIB_TYPE STATIC)
  endif()

  add_library(html-to-markdown-ffi::html-to-markdown-ffi ${_HTM_FFI_LIB_TYPE} IMPORTED)

  # ── Step 4: Set target properties ─────────────────────────────────

  set_target_properties(html-to-markdown-ffi::html-to-markdown-ffi PROPERTIES
    IMPORTED_LOCATION "${_HTM_FFI_LIBRARY}"
    INTERFACE_INCLUDE_DIRECTORIES "${_HTM_FFI_INCLUDE_DIR}"
  )

  # On Windows with SHARED libraries, handle the DLL + import lib split
  if(WIN32 AND _HTM_FFI_LIB_TYPE STREQUAL "SHARED")
    # The found .dll.lib or .lib is the import library; find the actual DLL
    find_file(_HTM_FFI_DLL
      NAMES html_to_markdown_ffi.dll libhtml_to_markdown_ffi.dll
      PATHS "${_HTM_FFI_PREFIX}/bin" "${_HTM_FFI_PREFIX}/lib"
      NO_DEFAULT_PATH
    )
    if(_HTM_FFI_DLL)
      set_target_properties(html-to-markdown-ffi::html-to-markdown-ffi PROPERTIES
        IMPORTED_LOCATION "${_HTM_FFI_DLL}"
        IMPORTED_IMPLIB "${_HTM_FFI_LIBRARY}"
      )
    endif()
    unset(_HTM_FFI_DLL CACHE)
  endif()

  # Platform-specific link dependencies
  if(APPLE)
    set_property(TARGET html-to-markdown-ffi::html-to-markdown-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES "-framework CoreFoundation" "-framework Security" pthread)
  elseif(UNIX)
    set_property(TARGET html-to-markdown-ffi::html-to-markdown-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES pthread dl m)
  elseif(WIN32)
    set_property(TARGET html-to-markdown-ffi::html-to-markdown-ffi APPEND PROPERTY
      INTERFACE_LINK_LIBRARIES ws2_32 userenv bcrypt)
  endif()

  unset(_HTM_FFI_LIB_TYPE)
endif()

mark_as_advanced(_HTM_FFI_LIBRARY _HTM_FFI_INCLUDE_DIR)
unset(_HTM_FFI_CMAKE_DIR)
unset(_HTM_FFI_PREFIX)
