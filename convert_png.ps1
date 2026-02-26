Add-Type -AssemblyName System.Drawing

# Load the original PNG
$originalPath = "src-tauri/icons/pocket.png"
$outputPath = "src-tauri/icons/pocket_rgba.png"

try {
    $image = [System.Drawing.Image]::FromFile((Resolve-Path $originalPath).Path)
    
    # Create a new bitmap with 32-bit ARGB format
    $rgbaBitmap = New-Object System.Drawing.Bitmap($image.Width, $image.Height, [System.Drawing.Imaging.PixelFormat]::Format32bppArgb)
    
    # Create graphics object
    $graphics = [System.Drawing.Graphics]::FromImage($rgbaBitmap)
    
    # Draw the original image onto the new bitmap
    $graphics.DrawImage($image, 0, 0, $image.Width, $image.Height)
    
    # Save as PNG with alpha channel
    $rgbaBitmap.Save($outputPath, [System.Drawing.Imaging.ImageFormat]::Png)
    
    # Cleanup
    $graphics.Dispose()
    $rgbaBitmap.Dispose()
    $image.Dispose()
    
    Write-Host "Successfully created RGBA PNG: $outputPath"
} catch {
    Write-Host "Error: $_"
}
