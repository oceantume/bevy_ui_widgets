
// todo: reimnplement extractor with a different marker component TextInput

use bevy_sprite::prelude::*;
use bevy_app::prelude::*;
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_render::{prelude::*, RenderWorld, RenderStage};
use bevy_text::{prelude::*, DefaultTextPipeline};
use bevy_transform::prelude::*;
use bevy_ui::{prelude::*, FocusPolicy, RenderUiSystem, ExtractedUiNode, ExtractedUiNodes};
use bevy_utils::prelude::*;
use bevy_window::{prelude::*, WindowId};

pub struct TextInputPlugin;

impl Plugin for TextInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_to_stage(
            RenderStage::Extract,
            extract_text_uinodes.after(RenderUiSystem::ExtractNode),
        );
    }
}


pub fn extract_text_uinodes(
    mut render_world: ResMut<RenderWorld>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    text_pipeline: Res<DefaultTextPipeline>,
    windows: Res<Windows>,
    uinode_query: Query<(
        Entity,
        &Node,
        &GlobalTransform,
        &Text,
        &Visibility,
        Option<&CalculatedClip>,
    )>,
) {
    let mut extracted_uinodes = render_world.resource_mut::<ExtractedUiNodes>();

    let scale_factor = windows.scale_factor(WindowId::primary()) as f32;

    for (entity, uinode, transform, text, visibility, clip) in uinode_query.iter() {
        if !visibility.is_visible {
            continue;
        }
        // Skip if size is set to zero (e.g. when a parent is set to `Display::None`)
        if uinode.size == Vec2::ZERO {
            continue;
        }
        if let Some(text_layout) = text_pipeline.get_glyphs(&entity) {
            let text_glyphs = &text_layout.glyphs;
            let alignment_offset = (uinode.size / -2.0).extend(0.0);

            for text_glyph in text_glyphs {
                let color = text.sections[text_glyph.section_index].style.color;
                let atlas = texture_atlases
                    .get(text_glyph.atlas_info.texture_atlas.clone_weak())
                    .unwrap();
                let texture = atlas.texture.clone_weak();
                let index = text_glyph.atlas_info.glyph_index as usize;
                let rect = atlas.textures[index];
                let atlas_size = Some(atlas.size);

                let transform =
                    Mat4::from_rotation_translation(transform.rotation, transform.translation)
                        * Mat4::from_scale(transform.scale / scale_factor)
                        * Mat4::from_translation(
                            alignment_offset * scale_factor + text_glyph.position.extend(0.),
                        );

                extracted_uinodes.uinodes.push(ExtractedUiNode {
                    transform,
                    color,
                    rect,
                    image: texture,
                    atlas_size,
                    clip: clip.map(|clip| clip.clip),
                    border_color: None,
                    border_width: None,
                    corner_radius: None,
                });
            }
        }
    }
}
