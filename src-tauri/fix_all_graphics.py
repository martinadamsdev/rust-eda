#!/usr/bin/env python3
import re
import os

def fix_component_symbols(content):
    """Fix all ComponentSymbol instances to include graphics field"""
    lines = content.split('\n')
    result = []
    i = 0
    
    while i < len(lines):
        line = lines[i]
        
        # Check if this line starts a ComponentSymbol struct
        if 'ComponentSymbol {' in line:
            # Collect all lines of this struct
            struct_lines = [line]
            brace_count = line.count('{') - line.count('}')
            j = i + 1
            
            while j < len(lines) and brace_count > 0:
                struct_lines.append(lines[j])
                brace_count += lines[j].count('{') - lines[j].count('}')
                j += 1
            
            # Join the struct lines
            struct_text = '\n'.join(struct_lines)
            
            # Check if graphics field already exists
            if 'graphics:' not in struct_text:
                # Extract width and height values
                width_match = re.search(r'width:\s*([\d.]+)', struct_text)
                height_match = re.search(r'height:\s*([\d.]+)', struct_text)
                
                width = width_match.group(1) if width_match else '100.0'
                height = height_match.group(1) if height_match else '60.0'
                
                # Find the last field before the closing brace
                # Insert graphics field before the final closing brace
                last_brace_idx = struct_text.rfind('}')
                
                # Check if there's a trailing comma on the last field
                before_brace = struct_text[:last_brace_idx].rstrip()
                if not before_brace.endswith(','):
                    before_brace += ','
                
                graphics_field = f"""
        graphics: Some(SymbolGraphics {{
            bounds: GraphicsBounds {{
                width: {width},
                height: {height},
            }},
        }}),"""
                
                # Reconstruct the struct
                fixed_struct = before_brace + graphics_field + '\n    ' + struct_text[last_brace_idx:]
                result.append(fixed_struct)
            else:
                result.append(struct_text)
            
            # Skip the lines we've already processed
            i = j
        else:
            result.append(line)
            i += 1
    
    return '\n'.join(result)

def add_imports(content):
    """Add necessary imports if not present"""
    imports_to_add = []
    
    if 'SymbolGraphics' not in content:
        # Check if we need to add the import
        if 'use crate::models::component::{' in content:
            # Add to existing import
            content = re.sub(
                r'use crate::models::component::\{([^}]+)\};',
                lambda m: f"use crate::models::component::{{{m.group(1)}, SymbolGraphics, GraphicsBounds}};",
                content
            )
        else:
            imports_to_add.append('use crate::models::component::{SymbolGraphics, GraphicsBounds};')
    
    if imports_to_add:
        # Add imports at the beginning after other use statements
        lines = content.split('\n')
        for i, line in enumerate(lines):
            if line.startswith('use '):
                continue
            elif line == '' and i > 0 and lines[i-1].startswith('use '):
                # Insert here
                lines.insert(i, '\n'.join(imports_to_add))
                break
        else:
            # No use statements found, add at the beginning
            lines.insert(0, '\n'.join(imports_to_add))
        
        content = '\n'.join(lines)
    
    return content

def process_file(filepath):
    """Process a single Rust file"""
    print(f"Processing {filepath}...")
    
    with open(filepath, 'r') as f:
        content = f.read()
    
    # Add imports
    content = add_imports(content)
    
    # Fix ComponentSymbol structs
    content = fix_component_symbols(content)
    
    with open(filepath, 'w') as f:
        f.write(content)
    
    print(f"  Fixed {filepath}")

# Process all files
files_to_fix = [
    'src/services/extended_components.rs',
    'src/services/sensors_power.rs',
    'src/services/integrated_circuits.rs',
    'src/services/logic_gates.rs',
]

for file in files_to_fix:
    if os.path.exists(file):
        process_file(file)
    else:
        print(f"File not found: {file}")

print("\nAll files processed!")