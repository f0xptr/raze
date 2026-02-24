# commands.rs Documentation

This document provides an overview of `src/cli/commands.rs`, which defines the available subcommands that the Raze CLI application supports.

## Overview

This module is dedicated to defining the various subcommands supported by the Raze CLI application. It leverages `clap`'s `Subcommand` derive attribute to enumerate and describe the primary operations that users can execute, such as `pack` for compression and `unpack` for decompression.

Each subcommand variant is designed to include its specific arguments and a clear description of its intended purpose. This approach significantly enhances the CLI's usability and self-documentation capabilities, making it intuitive for users to understand and operate.

## Enums

### `enum Commands`

Enumerates the core operations that the Raze CLI can perform.

This `enum` explicitly defines the distinct functionalities that are accessible via the command line interface. Each variant within this enum represents a specific subcommand that users are able to invoke, along with the particular set of parameters required for its successful execution.

#### Variants

*   `Pack`

    Compresses a file or directory into a `.rz` archive.

    The `pack` subcommand requires a source path (which can be either a file or a directory) and an output path for the newly created `.rz` archive. It employs Zstandard compression and Tar archiving to generate efficient archives.

    *   **Fields:**
        *   `source`: The path to the source file or directory that is intended for compression. This can be an absolute or relative path to the content to be archived.
        *   `output`: The name or path for the output `.rz` archive file. If the `.rz` extension is omitted, it will be automatically appended for consistency.

*   `Unpack`

    Decompresses a `.rz` archive into the current or a specified directory.

    The `unpack` subcommand necessitates the provision of a path to an existing `.rz` archive and, optionally, a destination directory where its contents will be extracted.

    *   **Fields:**
        *   `archive`: The path to the `.rz` archive file that needs to be decompressed. This must be a valid path pointing to an existing Raze archive.
        *   `destination`: The target directory where the archive's contents will be extracted. If this parameter is not specified, the archive will be extracted into the current working directory by default.
