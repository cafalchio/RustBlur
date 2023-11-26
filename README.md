# Image Blur using Rust

This Rust program applies a box blur kernel to an image using parallel processing (utilizing Rayon) and saves the blurred output.

### How to Run:

    git clone <repository_url>


2. **Navigate to the Project Directory:**

    cd blur

3. **Install Rust (if not installed):**

Make sure you have Rust installed on your system. If not, follow the instructions on [Rust's official website](https://www.rust-lang.org/tools/install) to install it.

4. **Build the Project:**

cargo build --release

5. **Run the Program:**

Execute the following command:

    ./target/release/blur <kernel_size> <input_image_path> <output_image_path>

Replace `<kernel_size>` with the desired size of the box blur kernel (e.g., 5, 7, 9, etc.). 
Provide the `<input_image_path>` as the path to the image file you want to blur, and `<output_image_path>` as the desired path for the resulting blurred image.

**Example:**

    ./target/release/image-blur 5 input.jpg output_blurred.jpg

6. **Check Output:**

Once the program finishes, the blurred image will be saved at the specified `<output_image_path>`.

### Notes:

- The program applies a box blur filter of the specified kernel size to the input image using parallel processing with Rayon.
- Ensure the input image exists at the provided path and that you have write permissions for the specified output path.
---

### Warning

The image border was not taken in consideration. so "half of kernel size pixels" in the border will be kept as original;