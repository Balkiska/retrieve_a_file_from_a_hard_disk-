import os

# Specify the path to your file
file_path = 'yourfile.txt'

# Check if the file exists before trying to read it
if os.path.exists(file_path):
    # Open the file in read mode
    with open(file_path, 'r') as file:
        # Read the content of the file
        content = file.read()
        # Print the content
        print(content)
else:
    # Print an error message if the file does not exist
    print("The file does not exist !")
