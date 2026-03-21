// level.dat
pub mod player;
use crate::{
    nbt::{Nbt, NbtValue},
    NbtError,
};

pub struct LevelData {
    pub nbt: Nbt,
}

impl LevelData {
    pub fn get_level_name(&self) -> Option<&str> {
        match self.nbt.try_get("Data.LevelName") {
            Some(NbtValue::String(s)) => Some(s),
            _ => None,
        }
    }

    pub fn set_level_name(&mut self, name: String) -> Result<(), NbtError> {
        let key = "Data.LevelName";
        let v = self
            .nbt
            .try_get_mut(key)
            .ok_or_else(|| NbtError::UnknownCompound(key.to_string()))?;
        *v = NbtValue::String(name);
        Ok(())
    }
    pub fn get_seed(&self) -> Option<i64> {
        match self.nbt.try_get("Data.WorldGenSettings.seed") {
            Some(NbtValue::Long(v)) => Some(*v),
            _ => None,
        }
    }
    pub fn get_spawn(&self) -> Option<(i32, i32, i32)> {
        let x = match self.nbt.try_get("Data.SpawnX") {
            Some(NbtValue::Int(v)) => *v,
            _ => return None,
        };
        let y = match self.nbt.try_get("Data.SpawnY") {
            Some(NbtValue::Int(v)) => *v,
            _ => return None,
        };
        let z = match self.nbt.try_get("Data.SpawnZ") {
            Some(NbtValue::Int(v)) => *v,
            _ => return None,
        };
        Some((x, y, z))
    }

    pub fn set_spawn(&mut self, x: i32, y: i32, z: i32) -> Result<(), NbtError> {
        let key_x = "Data.SpawnX";
        let key_y = "Data.SpawnY";
        let key_z = "Data.SpawnZ";

        *self.nbt.try_get_mut(key_x)
            .ok_or_else(|| NbtError::UnknownCompound(key_x.to_string()))? =
            NbtValue::Int(x);
        *self.nbt.try_get_mut(key_y)
            .ok_or_else(|| NbtError::UnknownCompound(key_y.to_string()))? =
            NbtValue::Int(y);
        *self.nbt.try_get_mut(key_z)
            .ok_or_else(|| NbtError::UnknownCompound(key_z.to_string()))? =
            NbtValue::Int(z);
        Ok(())
    }
    // 玩家
    pub fn player(&self) -> Option<&NbtValue> {
        self.nbt.try_get("Data.Player")
    }

    pub fn player_mut(&mut self) -> Option<&mut NbtValue> {
        self.nbt.try_get_mut("Data.Player")
    }

    // 游戏规则
    pub fn set_gamerule(&mut self, rule: &str, value: bool) -> Result<(), NbtError> {
        let key = "Data.GameRules";
        let rules = self
            .nbt
            .try_get_mut(key)
            .ok_or_else(|| NbtError::UnknownCompound(key.to_string()))?;

        let map = match rules {
            NbtValue::Compound(map) => map,
            _ => return Err(NbtError::UnknownCompound(key.to_string())),
        };

        map.insert(
            rule.to_string(),
            NbtValue::String(if value { "true".to_string() } else { "false".to_string() }),
        );

        Ok(())
    }

}
