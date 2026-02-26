Add-Type -AssemblyName System.Drawing

# Load the PNG image
$pngPath = "src-tauri/icons/pocket.png"
$icoPath = "src-tauri/icons/icon.ico"

try {
    $image = [System.Drawing.Image]::FromFile((Resolve-Path $pngPath).Path)
    
    # Create multiple sizes for proper ICO format (16x16, 32x32, 48x48, 256x256)
    $sizes = @(16, 32, 48, 256)
    $bitmaps = @()
    
    foreach ($size in $sizes) {
        $bitmap = New-Object System.Drawing.Bitmap($size, $size)
        $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
        $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
        $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
        $graphics.DrawImage($image, 0, 0, $size, $size)
        $graphics.Dispose()
        $bitmaps += $bitmap
    }
    
    # Create icon from the 32x32 bitmap (primary)
    $icon = [System.Drawing.Icon]::FromHandle($bitmaps[1].GetHicon())
    
    # Save as ICO file
    $fileStream = New-Object System.IO.FileStream($icoPath, [System.IO.FileMode]::Create)
    $icon.Save($fileStream)
    $fileStream.Close()
    
    # Cleanup
    foreach ($bitmap in $bitmaps) {
        $bitmap.Dispose()
    }
    $icon.Dispose()
    $image.Dispose()
    
    Write-Host "Successfully created proper ICO file: $icoPath"
} catch {
    Write-Host "Error: $_"
}
