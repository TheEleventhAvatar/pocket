# For macOS, we'll create a simple PNG-based approach
# macOS can work directly with PNG files, so let's create a properly sized PNG

Add-Type -AssemblyName System.Drawing

$pngPath = "src-tauri/icons/pocket.png"
$icnsPath = "src-tauri/icons/pocket.icns"

try {
    $image = [System.Drawing.Image]::FromFile((Resolve-Path $pngPath).Path)
    
    # Create 512x512 PNG for macOS (standard size)
    $macSize = 512
    $macBitmap = New-Object System.Drawing.Bitmap($macSize, $macSize)
    $graphics = [System.Drawing.Graphics]::FromImage($macBitmap)
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $graphics.DrawImage($image, 0, 0, $macSize, $macSize)
    
    # Save as PNG (macOS can use PNG directly)
    $macPngPath = "src-tauri/icons/pocket_512.png"
    $macBitmap.Save($macPngPath, [System.Drawing.Imaging.ImageFormat]::Png)
    
    # Cleanup
    $graphics.Dispose()
    $macBitmap.Dispose()
    $image.Dispose()
    
    Write-Host "Created 512x512 PNG for macOS: $macPngPath"
} catch {
    Write-Host "Error: $_"
}
