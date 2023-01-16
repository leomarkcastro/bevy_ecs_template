from PIL import Image
import os

target_folder = "resize_box/"

# Get all the images in the current directory
images = [f for f in os.listdir(target_folder) if f.endswith('.png')]

for image in images:
    # Open an image file
    print("Resizing image: ", image)
    with Image.open(f"{target_folder}{image}") as im:
        # Resize the image
        im_resized = im.resize((32, 32), resample=Image.NEAREST)
        # Save the resized image
        im_resized.save(f"{target_folder}{image}")

    print(" > Done resizing image: ", image)
