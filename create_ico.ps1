Add-Type -AssemblyName System.Drawing

# Load the PNG image
$pngPath = "src-tauri/icons/pocket.png"
$icoPath = "src-tauri/icons/icon.ico"

try {
    $image = [System.Drawing.Image]::FromFile((Resolve-Path $pngPath).Path)
    
    # Create a new bitmap for the icon (32x32 is standard)
    $iconSize = 32
    $iconBitmap = New-Object System.Drawing.Bitmap($iconSize, $iconSize)
    
    # Create graphics object
    $graphics = [System.Drawing.Graphics]::FromImage($iconBitmap)
    
    # Set high quality rendering
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    
    # Draw the original image scaled to icon size
    $graphics.DrawImage($image, 0, 0, $iconSize, $iconSize)
    
    # Convert to icon format
    $icon = [System.Drawing.Icon]::FromHandle($iconBitmap.GetHicon())
    
    # Save as ICO file
    $fileStream = New-Object System.IO.FileStream($icoPath, [System.IO.FileMode]::Create)
    $icon.Save($fileStream)
    $fileStream.Close()
    
    # Cleanup
    $graphics.Dispose()
    $iconBitmap.Dispose()
    $image.Dispose()
    
    Write-Host "Successfully created ICO file: $icoPath"
} catch {
    Write-Host "Error: $_"
}
