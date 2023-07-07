use crate::gen::entityc::Entityc;
use crate::gen::player::Player;

/// Interface for {@link mindustry.entities.comp.SyncComp}
pub trait Syncc : Entityc {
    fn is_sync_hidden(player: Player) -> bool;

    fn last_updated() -> i64;

    fn update_spacing() -> i64;

    fn after_sync();

    fn handle_sync_hidden();

    fn interpolate();

    fn last_updated_last_updated(last_updated: i64);

    fn read_sync(read: Reads);

    fn read_sync_manual(buffer: FloatBuffer);

    fn remove();

    fn snap_interpolation();

    fn snap_sync();

    fn update();

    fn update_spacing_update_spacing(update_spacing: i64);

    fn write_sync(write: Writes);

    fn write_sync_manual(buffer: FloatBuffer);
}
