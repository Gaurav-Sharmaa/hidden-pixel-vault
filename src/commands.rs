use crate::Result;
use crate::atomic_file::AtomicFileHandler;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use std::str::FromStr;

pub fn print(path: &str) -> Result<()> {
    let handler = AtomicFileHandler::new(path)?;
    let buffer = handler.read_file()?;

    let png =
        Png::try_from(buffer.as_slice()).map_err(|e| format!("Failed to parse PNG: {}", e))?;

    let chunk_types: Vec<String> = png
        .chunks()
        .iter()
        .map(|c| c.chunk_type().to_string())
        .collect();

    println!(
        " 📋  Available chunks in '{}':",
        handler.target_path().display()
    );
    for chunk in chunk_types {
        println!("  • {}", chunk);
    }
    Ok(())
}

pub fn decode(path: &str, chunk_type: &str) -> Result<()> {
    let handler = AtomicFileHandler::new(path)?;
    let buffer = handler.read_file()?;

    let png =
        Png::try_from(buffer.as_slice()).map_err(|e| format!("Failed to parse PNG: {}", e))?;

    match png.chunk_by_type(chunk_type) {
        Some(target) => match target.data_as_string() {
            Ok(message) => {
                println!("🔓  Hidden message found:");
                println!("    File: {}", handler.target_path().display());
                println!("    Chunk: {}", chunk_type);
                println!("    Message: {}", message);
                Ok(())
            }
            Err(_) => {
                println!(
                    " ❌  Cannot decode message from chunk '{}': This chunk contains binary data, not text",
                    chunk_type
                );
                println!(
                    " 💡  Tip: This chunk may be a critical PNG chunk or contain non-text data"
                );
                Ok(())
            }
        },
        None => Err(format!(" Chunk type '{}' not found", chunk_type).into()),
    }
}

pub fn encode(path: &str, chunk_type: &str, message: &str) -> Result<()> {
    // Check for critical PNG chunks
    if ["IHDR", "PLTE", "IDAT", "IEND"].contains(&chunk_type) {
        return Err(format!(
            " ❌  Cannot use critical PNG chunk name '{}'. Please use a different chunk name.\n 💡 Tip: Make sure the 3rd character is uppercase (e.g., 'abCd', 'boOp', 'vaRu')",
            chunk_type
        ).into());
    }

    // Validate chunk type format (3rd character must be uppercase)
    if chunk_type.len() == 4 {
        let chars: Vec<char> = chunk_type.chars().collect();
        if !chars[2].is_uppercase() {
            return Err(format!(
                " ❌ Invalid chunk type '{}'. The 3rd character must be uppercase.\n💡  Example: '{}{}{}{}' should be '{}{}{}{}' ",
                chunk_type,
                chars[0], chars[1], chars[2], chars[3],
                chars[0], chars[1], chars[2].to_uppercase().next().unwrap(), chars[3]
            ).into());
        }
    }

    let handler = AtomicFileHandler::new(path)?;

    println!(
        "🔐  Encoding message into '{}'...",
        handler.target_path().display()
    );

    handler.atomic_modify(|content| {
        // Parse PNG
        let mut png =
            Png::try_from(content.as_slice()).map_err(|e| format!("Failed to parse PNG: {}", e))?;

        // Check for duplicate chunk
        if png.chunk_by_type(chunk_type).is_some() {
            return Err(format!(
                " ❌ Chunk '{}' already exists. Cannot add duplicate message.\n💡  Tip: Use a different chunk name to store another hidden message",
                chunk_type
            ).into());
        }

        // Remove IEND chunk
        let end = png
            .remove_chunk("IEND")
            .map_err(|e| format!("Failed to remove IEND chunk: {}", e))?;

        // Create and validate chunk type
        let chunk_type_obj =
            ChunkType::from_str(chunk_type).map_err(|e| format!("Invalid chunk type: {}", e))?;

        // Add new chunk with message
        png.append_chunk(Chunk::new(chunk_type_obj, message.as_bytes().to_vec()));

        // Re-add IEND chunk
        png.append_chunk(end);

        println!(" ✅ Message encoded successfully");
        Ok(png.as_bytes())
    })
}

pub fn remove(path: &str, chunk_type: &str) -> Result<()> {
    println!("🗑️  Removing the Hidden Message:");
    println!("   File: {}", path);
    println!("   Chunk: {}", chunk_type);

    // Check if it's a critical chunk before attempting modification
    if ["IHDR", "PLTE", "IDAT", "IEND"].contains(&chunk_type) {
        println!(
            "  Removed: ❌ Cannot remove critical PNG chunk '{}'",
            chunk_type
        );
        println!("💡 Tip: Use 'restore' command if you need to revert changes");
        return Ok(());
    }

    let handler = AtomicFileHandler::new(path)?;

    // Check if chunk exists before creating backup
    let buffer = handler.read_file()?;
    let png =
        Png::try_from(buffer.as_slice()).map_err(|e| format!("Failed to parse PNG: {}", e))?;

    if png.chunk_by_type(chunk_type).is_none() {
        println!("   Removed: ❌ Failed to remove chunk -> chunk not found");
        println!("💡 Tip: Use 'restore' command if you need to revert changes");
        return Ok(());
    }

    // Create backup silently and perform removal
    handler.atomic_modify_silent(|content| {
        let mut png =
            Png::try_from(content.as_slice()).map_err(|e| format!("Failed to parse PNG: {}", e))?;

        png.remove_chunk(chunk_type)
            .map_err(|e| format!("Failed to remove chunk: {}", e))?;

        println!("   Removed: ✅ Successfully");
        Ok(png.as_bytes())
    })
}

pub fn restore_original(path: &str) -> Result<()> {
    // Check if the provided path is a backup file
    if path.ends_with(".backup") {
        // User provided backup file path, restore to original
        let original_path = path.strip_suffix(".backup").unwrap();

        println!("🔄 Restoring original file from backup...");
        println!("  From: {}", path);

        if !std::path::Path::new(path).exists() {
            return Err(format!("Backup file '{}' not found", path).into());
        }

        std::fs::copy(path, original_path)
            .map_err(|e| format!("Failed to restore from backup: {}", e))?;

        println!("✅ Original file restored successfully");
        return Ok(());
    }

    // Original behavior - restore from handler's backup
    let handler = AtomicFileHandler::new(path)?;

    if !handler.has_backup() {
        return Err(format!(
            "No backup found for '{}'. File may already be in original state.",
            path
        )
        .into());
    }

    handler.restore_original()
}

pub fn cleanup_files(path: &str) -> Result<()> {
    let handler = AtomicFileHandler::new(path)?;
    handler.cleanup()
}

pub fn show_status(path: &str) -> Result<()> {
    let handler = AtomicFileHandler::new(path)?;

    println!("📊  File Status:");
    println!(
        "   Target file: {} {}",
        handler.target_path().display(),
        if handler.target_path().exists() {
            "✅"
        } else {
            "❌"
        }
    );
    println!(
        "   Backup file: {} {}",
        handler.backup_path().display(),
        if handler.has_backup() { "✅" } else { "❌" }
    );

    if handler.has_backup() {
        println!("💡 Use 'restore' command to revert to original");
    }

    Ok(())
}
