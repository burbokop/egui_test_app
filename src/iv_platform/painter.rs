#![allow(unsafe_code)]


use egui::{
    emath::Rect,
    epaint::{Color32, Mesh, Vertex},
};
use epaint::ClippedShape;

use super::{inkview as iv, epi_backend::pixels_per_point32};
use super::convert::{from_iv, to_iv};

pub struct Painter {
    max_texture_side: usize,

    srgb_support: bool,
    /// The filter used for subsequent textures.
    texture_filter: TextureFilter,
    //post_process: Option<PostProcess>,
    pixels_per_point: f32,
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
    pub fn new(pixels_per_point: f32) -> Result<Painter, String> {

        Ok(Self { 
            destroyed: false, max_texture_side: 1000, 
            srgb_support: true, 
            texture_filter: TextureFilter::Linear, 
            pixels_per_point: pixels_per_point 
        })

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

    pub fn paint_shape<'f>(&mut self, shape: ClippedShape, font: &iv::Font<'f>) -> Option<iv::Rect> {
        match shape.1 {
            egui::Shape::Noop => todo!(),
            egui::Shape::Vec(_) => todo!(),
            egui::Shape::Circle(circle) => {
                Some(iv::draw_circle_quarter(
                    to_iv::emath_pos(circle.center, self.pixels_per_point), 
                    circle.radius as usize, 
                    iv::Style::from_bits_truncate(iv::Style::default().bits() | iv::Style::FILL_INSIDE.bits()), 
                    circle.stroke.width as u32, 
                    to_iv::epaint_color(circle.stroke.color), 
                    to_iv::epaint_color(circle.fill)
                ))
            },
            egui::Shape::LineSegment { points, stroke } => todo!(),
            egui::Shape::Path(path) => todo!(),
            egui::Shape::Rect(rect) => {
                Some(iv::draw_frame_certified_ex(
                    to_iv::emath_rect(rect.rect, self.pixels_per_point), 
                    1, // rect.stroke.width as i32, 
                    iv::Side::default(),
                    iv::Style::from_bits_truncate(iv::Style::default().bits() | iv::Style::FILL_INSIDE.bits()), 
                    0, 
                    to_iv::epaint_color(rect.stroke.color), 
                    to_iv::epaint_color(rect.fill)
                ))
            }
            egui::Shape::Text(text) => {
                let galley = text.galley.as_ref();
                let job = galley.job.as_ref();

                let translated_rect = galley.rect.translate(text.pos.to_vec2());
               
                if job.sections.len() > 0 {
                    let f =  &job.sections[0].format.font_id;

                    //println!("f: {:?}", font);

                    //println!("f.family: {}", f.family);

                    iv::set_font(font, to_iv::epaint_color(text.override_text_color.unwrap_or(Color32::from_rgb(255, 255, 255))));
                
                    Some(iv::draw_text_rect(to_iv::emath_rect(translated_rect, self.pixels_per_point), job.text.as_str(), 0).0)
                } else {
                    None
                }
               
                //iv::draw_string(Self::emath_pos_to_iv_vec(text.pos), job.text.as_str());
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
        canvas: &mut iv::Canvas<'_>,
        font: &iv::Font<'f>
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
                    [image.width(), image.height()]

/*
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
                     */
                },
            };

            
            

            println!("\timage_delta: {:?}, {:?}", p, s)
        }

        for s in clipped_shapes {
            //println!("\tshape: {:?}");
            if let Some(update_rect) = self.paint_shape(s, font) {
                iv::dynamic_update(iv::DynamicUpdateType::A2(iv::update_type::A2), update_rect)
            }
        }
        
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