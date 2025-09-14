#!/usr/bin/env python3
import re
import sys

def fix_component_symbol(content):
    """Add graphics field to ComponentSymbol structs"""
    
    # Pattern to find ComponentSymbol structs
    pattern = r'(ComponentSymbol\s*\{[^}]*?)(\s*\})'
    
    def add_graphics_field(match):
        struct_content = match.group(1)
        closing_brace = match.group(2)
        
        # Check if graphics field already exists
        if 'graphics:' in struct_content:
            return match.group(0)
        
        # Extract width and height from the struct if available
        width_match = re.search(r'width:\s*([\d.]+)', struct_content)
        height_match = re.search(r'height:\s*([\d.]+)', struct_content)
        
        width = width_match.group(1) if width_match else '100.0'
        height = height_match.group(1) if height_match else '60.0'
        
        # Add graphics field with proper formatting
        graphics_field = f'''
        graphics: Some(SymbolGraphics {{
            bounds: GraphicsBounds {{
                width: {width},
                height: {height},
            }},
        }}),'''
        
        # Remove trailing comma from last field if exists
        struct_content = re.sub(r',(\s*$)', '', struct_content)
        
        return struct_content + ',' + graphics_field + closing_brace
    
    # Apply the fix
    fixed_content = re.sub(pattern, add_graphics_field, content, flags=re.DOTALL)
    
    return fixed_content

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: python fix_graphics.py <rust_file>")
        sys.exit(1)
    
    file_path = sys.argv[1]
    
    with open(file_path, 'r') as f:
        content = f.read()
    
    fixed_content = fix_component_symbol(content)
    
    with open(file_path, 'w') as f:
        f.write(fixed_content)
    
    print(f"Fixed {file_path}")