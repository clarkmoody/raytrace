# Raytrace

Working through [Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust.

## Final Render

![](output/random-scene-large.png)

## Progress Shots

Initial colors when writing to an image file programmatically.

![](output/colors.png)

Background gradient based on ray y-component.

![](output/gradient.png)

Hit sphere filled with solid color. The first raytracing.

![](output/hit-sphere.png)

Hit sphere colored by unit normal direction at point of intersection.

![](output/sphere-normals.png)

Scene contains list of multiple hittable objects.

![](output/hittables.png)

Antialiasing via randomized sub-pixel ray positions and multi-sampling.

![](output/antialiasing.png)

Initial ray bouncing from diffuse materials.

![](output/diffuse-material.png)

Gamma correction.

![](output/diffuse-gamma-corrected.png)

Handle "shadow acne" with self-intersection prevention.

![](output/shadow-acne.png)

Multiple materials: Lambertians and shiny metal.

![](output/shiny-metal.png)

Semi-reflective metal surfaces via fuzz factor.

![](output/fuzzy-metal.png)

Dielectric materials (glass) with Schlick approximation of reflection angle.

![](output/dielectric-schlick.png)

Wide-angle camera.

![](output/wide-angle-camera.png)

Camera positioning and field-of-view exploration.

![](output/camera-positioning-wide.png)

![](output/camera-positioning-zoom.png)

Camera depth of field.

![](output/depth-of-field.png)

Random scene generation.

![](output/random-scene.png)
