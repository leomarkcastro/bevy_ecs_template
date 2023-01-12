import json
from PIL import Image
import os

# Create a new image with a given size


def create_spritesheet(width, height):
    return Image.new('RGBA', (width, height), (255, 255, 255, 0))

# Add an image to the spritesheet at a given position


def add_sprite(spritesheet, sprite, x, y):
    spritesheet.paste(sprite, (x, y))

# Get all files in a directory that end with a certain extension


def get_files(directory, extension):
    try:
        return [os.path.join(directory, f) for f in os.listdir(directory) if f.endswith(extension)]
    except:
        return []


def process_spritesheet(directory, output_name):

    # Get all PNG files in the directory
    png_files = get_files(directory, '.png')

    # Get total spritesheet length
    total_sprites = len(png_files)

    # Get the largest width and height of all sprites
    max_width = 0
    max_height = 0
    for png_file in png_files:
        sprite = Image.open(png_file)
        width, height = sprite.size
        max_width = max(max_width, width)
        max_height = max(max_height, height)

    # Create the spritesheet
    spritesheet = create_spritesheet(max_width*total_sprites, max_height)

    # Add each PNG to the spritesheet
    for i, png_file in enumerate(png_files):
        sprite = Image.open(png_file)
        x = i
        y = 0
        add_sprite(spritesheet, sprite, x*max_width, y*max_height)

    # Save the spritesheet
    spritesheet.save(output_name)
    return {
        "max_width": max_width,
        "max_height": max_height,
        "total_sprites": total_sprites,
        "aspect_ratio": max_width/max_height,
    }


# Process all folders in the current directory
dat = {}
for directory in os.listdir('.'):
    print('Processing ' + directory)
    if os.path.isdir(directory):
        dat[directory] = process_spritesheet(directory, directory + '.png')

# Save the data as json
with open('data.json', 'w') as outfile:
    json.dump(dat, outfile)
