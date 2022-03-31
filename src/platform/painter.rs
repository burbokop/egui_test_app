#![allow(unsafe_code)]

use std::collections::HashMap;

use egui::{
    emath::Rect,
    epaint::{Color32, Mesh, Vertex},
};
use epaint::ClippedShape;

use super::inkview;


pub struct Painter {
    max_texture_side: usize,

    srgb_support: bool,
    /// The filter used for subsequent textures.
    texture_filter: TextureFilter,
    //post_process: Option<PostProcess>,

    //textures: HashMap<egui::TextureId, glow::Texture>,

    
    /// Stores outdated OpenGL textures that are yet to be deleted
    //textures_to_destroy: Vec<glow::Texture>,

    /// Used to make sure we are destroyed correctly.
    destroyed: bool,
}

#[derive(Copy, Clone)]
pub enum TextureFilter {
    Linear,
    Nearest,
}

impl Default for TextureFilter {
    fn default() -> Self {
        TextureFilter::Linear
    }
}

impl TextureFilter {
    pub(crate) fn glow_code(&self) -> u32 {
        0
        //match self {
        //    TextureFilter::Linear => glow::LINEAR,
        //    TextureFilter::Nearest => glow::NEAREST,
        //}
    }
}

impl Painter {
    /// Create painter.
    ///
    /// Set `pp_fb_extent` to the framebuffer size to enable `sRGB` support on OpenGL ES and WebGL.
    ///
    /// Set `shader_prefix` if you want to turn on shader workaround e.g. `"#define APPLY_BRIGHTENING_GAMMA\n"`
    /// (see <https://github.com/emilk/egui/issues/794>).
    ///
    /// # Errors
    /// will return `Err` below cases
    /// * failed to compile shader
    /// * failed to create postprocess on webgl with `sRGB` support
    /// * failed to create buffer
    pub fn new() -> Result<Painter, String> {

        Ok(Self { destroyed: false, max_texture_side: 1000, srgb_support: true, texture_filter: TextureFilter::Linear })

        /*
        check_for_gl_error(gl, "before Painter::new");

        let max_texture_side = unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) } as usize;

        let support_vao = crate::misc_util::supports_vao(gl);
        let shader_version = ShaderVersion::get(gl);
        let is_webgl_1 = shader_version == ShaderVersion::Es100;
        let header = shader_version.version();
        tracing::debug!("Shader header: {:?}.", header);
        let srgb_support = gl.supported_extensions().contains("EXT_sRGB");

        let (post_process, srgb_support_define) = match (shader_version, srgb_support) {
            // WebGL2 support sRGB default
            (ShaderVersion::Es300, _) | (ShaderVersion::Es100, true) => unsafe {
                // Add sRGB support marker for fragment shader
                if let Some([width, height]) = pp_fb_extent {
                    tracing::debug!("WebGL with sRGB enabled. Turning on post processing for linear framebuffer blending.");
                    // install post process to correct sRGB color:
                    (
                        Some(PostProcess::new(
                            gl,
                            shader_prefix,
                            support_vao,
                            is_webgl_1,
                            width,
                            height,
                        )?),
                        "#define SRGB_SUPPORTED",
                    )
                } else {
                    tracing::debug!("WebGL or OpenGL ES detected but PostProcess disabled because dimension is None");
                    (None, "")
                }
            },

            // WebGL1 without sRGB support disable postprocess and use fallback shader
            (ShaderVersion::Es100, false) => (None, ""),

            // OpenGL 2.1 or above always support sRGB so add sRGB support marker
            _ => (None, "#define SRGB_SUPPORTED"),
        };
 */
/*
            let vert = compile_shader(
                gl,
                glow::VERTEX_SHADER,
                &format!(
                    "{}\n{}\n{}\n{}",
                    header,
                    shader_prefix,
                    shader_version.is_new_shader_interface(),
                    VERT_SRC
                ),
            )?;
            let frag = compile_shader(
                gl,
                glow::FRAGMENT_SHADER,
                &format!(
                    "{}\n{}\n{}\n{}\n{}",
                    header,
                    shader_prefix,
                    srgb_support_define,
                    shader_version.is_new_shader_interface(),
                    FRAG_SRC
                ),
            )?;
            let program = link_program(gl, [vert, frag].iter())?;
            gl.detach_shader(program, vert);
            gl.detach_shader(program, frag);
            gl.delete_shader(vert);
            gl.delete_shader(frag);
            let u_screen_size = gl.get_uniform_location(program, "u_screen_size").unwrap();
            let u_sampler = gl.get_uniform_location(program, "u_sampler").unwrap();
            let vertex_buffer = gl.create_buffer()?;
            let element_array_buffer = gl.create_buffer()?;
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer));
            let a_pos_loc = gl.get_attrib_location(program, "a_pos").unwrap();
            let a_tc_loc = gl.get_attrib_location(program, "a_tc").unwrap();
            let a_srgba_loc = gl.get_attrib_location(program, "a_srgba").unwrap();
            let mut vertex_array = if support_vao {
                crate::misc_util::VAO::native(gl)
            } else {
                crate::misc_util::VAO::emulated()
            };
            vertex_array.bind_vertex_array(gl);
            vertex_array.bind_buffer(gl, &vertex_buffer);
            let stride = std::mem::size_of::<Vertex>() as i32;
            let position_buffer_info = vao_emulate::BufferInfo {
                location: a_pos_loc,
                vector_size: 2,
                data_type: glow::FLOAT,
                normalized: false,
                stride,
                offset: offset_of!(Vertex, pos) as i32,
            };
            let tex_coord_buffer_info = vao_emulate::BufferInfo {
                location: a_tc_loc,
                vector_size: 2,
                data_type: glow::FLOAT,
                normalized: false,
                stride,
                offset: offset_of!(Vertex, uv) as i32,
            };
            let color_buffer_info = vao_emulate::BufferInfo {
                location: a_srgba_loc,
                vector_size: 4,
                data_type: glow::UNSIGNED_BYTE,
                normalized: false,
                stride,
                offset: offset_of!(Vertex, color) as i32,
            };
            vertex_array.add_new_attribute(gl, position_buffer_info);
            vertex_array.add_new_attribute(gl, tex_coord_buffer_info);
            vertex_array.add_new_attribute(gl, color_buffer_info);
            check_for_gl_error(gl, "after Painter::new");

            Ok(Painter {
                max_texture_side,
                program,
                u_screen_size,
                u_sampler,
                is_webgl_1,
                is_embedded: matches!(shader_version, ShaderVersion::Es100 | ShaderVersion::Es300),
                vertex_array,
                srgb_support,
                texture_filter: Default::default(),
                post_process,
                vertex_buffer,
                element_array_buffer,
                textures: Default::default(),
                #[cfg(feature = "epi")]
                next_native_tex_id: 1 << 32,
                textures_to_destroy: Vec::new(),
                destroyed: false,
            })
        }
        */
    }

    pub fn max_texture_side(&self) -> usize {
        self.max_texture_side
    }

    /*
    unsafe fn prepare_painting(
        &mut self,
        [width_in_pixels, height_in_pixels]: [u32; 2],
        gl: &glow::Context,
        pixels_per_point: f32,
    ) -> (u32, u32) {
        gl.enable(glow::SCISSOR_TEST);
        // egui outputs mesh in both winding orders
        gl.disable(glow::CULL_FACE);

        gl.enable(glow::BLEND);
        gl.blend_equation(glow::FUNC_ADD);
        gl.blend_func_separate(
            // egui outputs colors with premultiplied alpha:
            glow::ONE,
            glow::ONE_MINUS_SRC_ALPHA,
            // Less important, but this is technically the correct alpha blend function
            // when you want to make use of the framebuffer alpha (for screenshots, compositing, etc).
            glow::ONE_MINUS_DST_ALPHA,
            glow::ONE,
        );

        let width_in_points = width_in_pixels as f32 / pixels_per_point;
        let height_in_points = height_in_pixels as f32 / pixels_per_point;

        gl.viewport(0, 0, width_in_pixels as i32, height_in_pixels as i32);
        gl.use_program(Some(self.program));

        gl.uniform_2_f32(Some(&self.u_screen_size), width_in_points, height_in_points);
        gl.uniform_1_i32(Some(&self.u_sampler), 0);
        gl.active_texture(glow::TEXTURE0);
        self.vertex_array.bind_vertex_array(gl);

        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.element_array_buffer));

        (width_in_pixels, height_in_pixels)
    } */

    pub fn emath_pos_to_iv_vec(pos: emath::Pos2) -> inkview::VecI32 {
        inkview::VecI32 { x: pos.x as i32, y: pos.y as i32 }
    }

    pub fn emath_rect_to_iv(rect: emath::Rect) -> inkview::Rect {
        inkview::Rect { pos: Self::emath_pos_to_iv_vec(rect.min), size: inkview::VecUSize { x: (rect.max.x - rect.min.x) as usize, y: (rect.max.y - rect.min.y) as usize } }
    }

    pub fn epaint_color_to_iv(color: epaint::Color32) -> inkview::Color32 {
        inkview::Color32::rgb(color.r(), color.g(), color.b())
    }

    pub fn paint_shape<'f>(&mut self, shape: ClippedShape, font: &inkview::Font<'f>) {
        match shape.1 {
            egui::Shape::Noop => todo!(),
            egui::Shape::Vec(_) => todo!(),
            egui::Shape::Circle(circle) => inkview::draw_circle(Painter::emath_pos_to_iv_vec(circle.center), circle.radius as i32, Self::epaint_color_to_iv(circle.fill)),
            egui::Shape::LineSegment { points, stroke } => todo!(),
            egui::Shape::Path(path) => todo!(),
            egui::Shape::Rect(rect) => inkview::fill_area(Self::emath_rect_to_iv(rect.rect), Self::epaint_color_to_iv(rect.fill)),
            egui::Shape::Text(text) => {

                let str = &text.galley.as_ref().job.as_ref().text;

                inkview::set_font(font, Self::epaint_color_to_iv(text.override_text_color.unwrap_or(Color32::from_rgb(255, 255, 255))));
                inkview::draw_string(Self::emath_pos_to_iv_vec(text.pos), str.as_str());
            },
            egui::Shape::Mesh(_) => todo!(),
            egui::Shape::QuadraticBezier(_) => todo!(),
            egui::Shape::CubicBezier(_) => todo!(),
        }
    }

    
    pub fn paint_and_update_textures<'f>(
        &mut self,
        clipped_shapes: Vec<epaint::ClippedShape>,
        textures_delta: &egui::TexturesDelta,
        canvas: &mut inkview::Canvas<'_>,
        font: &inkview::Font<'f>
    ) {

        for (id, image_delta) in &textures_delta.set {
            let p = &image_delta.pos;

            let s = match &image_delta.image {
                egui::ImageData::Color(image) => {

                    //depth = 32
                    println!("\tcolor image: {:?}", image.size);
                    [image.width(), image.height()]
                },
                egui::ImageData::Alpha(image) => {
                    println!("\talpha image: {:?}", image.size);


                    let unwraped_pos = image_delta.pos.unwrap_or([0, 0]);

                    let end_pos = [
                        canvas.width.min(image.width() + unwraped_pos[0]),
                        canvas.height.min(image.height() + unwraped_pos[1]),
                    ];

                    println!("before {}, {}, {}, {}",unwraped_pos[0], unwraped_pos[1], end_pos[0] - unwraped_pos[0], end_pos[1] - unwraped_pos[1]);
                    for y in unwraped_pos[1]..end_pos[1] {
                        for x in unwraped_pos[0]..end_pos[0] {
                        let uu = image.pixels[x + y * image.width()];
                            canvas.pixels[x + y * canvas.width] = uu;

                        }
                    }
                    println!("after {}, {}, {}, {}",unwraped_pos[0], unwraped_pos[1], end_pos[0] - unwraped_pos[0], end_pos[1] - unwraped_pos[1]);

                    inkview::dynamic_update(
                        inkview::DynamicUpdateType::Normal(inkview::update_type::Normal), 
                        unwraped_pos[0], 
                        unwraped_pos[1], 
                        end_pos[0] - unwraped_pos[0], 
                        end_pos[1] - unwraped_pos[1]
                    );

                    [image.width(), image.height()]
                },
            };
            

            println!("\timage_delta: {:?}, {:?}", p, s)
        }

        for s in clipped_shapes {
            //println!("\tshape: {:?}");
            self.paint_shape(s, font)
        }

        inkview::full_update(inkview::FullSoftUpdateType::Normal(inkview::update_type::Normal))

        //self.paint_meshes(gl, inner_size, pixels_per_point, clipped_meshes);

        //for &id in &textures_delta.free {
        //    self.free_texture(gl, id);
        //}
    }
}

impl Drop for Painter {
    fn drop(&mut self) {
        if !self.destroyed {
            panic!(
                "You forgot to call destroy() on the egui glow painter. Resources will leak!"
            );
        }
    }
}

#[cfg(feature = "epi")]
impl epi::NativeTexture for Painter {
    type Texture = glow::Texture;

    fn register_native_texture(&mut self, native: Self::Texture) -> egui::TextureId {
        self.assert_not_destroyed();
        let id = egui::TextureId::User(self.next_native_tex_id);
        self.next_native_tex_id += 1;
        self.textures.insert(id, native);
        id
    }

    fn replace_native_texture(&mut self, id: egui::TextureId, replacing: Self::Texture) {
        if let Some(old_tex) = self.textures.insert(id, replacing) {
            self.textures_to_destroy.push(old_tex);
        }
    }
}