// 仅限单人的 Data.Player

use crate::{
    nbt::{NbtValue},
    NbtError,
};

pub struct PlayerData<'a> {
    pub root: &'a mut NbtValue,
}

impl<'a> PlayerData<'a> {
    fn as_compound(&self) -> Result<&std::collections::HashMap<String, NbtValue>, NbtError> {
        match &*self.root {
            NbtValue::Compound(map) => Ok(map),
            _ => Err(NbtError::UnknownCompound("Data.Player".to_string())),
        }
    }

    fn as_compound_mut(&mut self) -> Result<&mut std::collections::HashMap<String, NbtValue>, NbtError> {
        match self.root {
            NbtValue::Compound(map) => Ok(map),
            _ => Err(NbtError::UnknownCompound("Data.Player".to_string())),
        }
    }


    pub fn get_pos(&self) -> Option<(f64, f64, f64)> {
        let map = self.as_compound().ok()?;
        match map.get("Pos") {
            Some(NbtValue::List(list)) if list.len() == 3 => {
                let x = match &list[0] { NbtValue::Double(v) => *v, _ => return None };
                let y = match &list[1] { NbtValue::Double(v) => *v, _ => return None };
                let z = match &list[2] { NbtValue::Double(v) => *v, _ => return None };
                Some((x, y, z))
            }
            _ => None,
        }
    }

    pub fn set_pos(&mut self, x: f64, y: f64, z: f64) -> Result<(), NbtError> {
        let map = self.as_compound_mut()?;
        map.insert(
            "Pos".to_string(),
            NbtValue::List(vec![
                NbtValue::Double(x),
                NbtValue::Double(y),
                NbtValue::Double(z),
            ]),
        );
        Ok(())
    }

    pub fn get_health(&self) -> Option<f32> {
        let map = self.as_compound().ok()?;
        match map.get("Health") {
            Some(NbtValue::Float(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn set_health(&mut self, v: f32) -> Result<(), NbtError> {
        let map = self.as_compound_mut()?;
        map.insert("Health".to_string(), NbtValue::Float(v));
        Ok(())
    }

    pub fn get_xp_level(&self) -> Option<i32> {
        let map = self.as_compound().ok()?;
        match map.get("XpLevel") {
            Some(NbtValue::Int(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn set_xp_level(&mut self, v: i32) -> Result<(), NbtError> {
        let map = self.as_compound_mut()?;
        map.insert("XpLevel".to_string(), NbtValue::Int(v));
        Ok(())
    }
}