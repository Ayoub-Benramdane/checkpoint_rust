#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum RhFactor {
    Positive,
    Negative,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        if (self.antigen == other.antigen || other.antigen == Antigen::O) && (self.rh_factor == RhFactor::Positive || self.rh_factor == other.rh_factor) {
            return  true;
        }
        if (self.antigen == Antigen::AB && (other.antigen == Antigen::B ||other.antigen == Antigen::B)) && (self.rh_factor == RhFactor::Positive || self.rh_factor == other.rh_factor) {
            return  true;
        }
        return false;
    }
    
    pub fn donors(&self) -> Vec<Self> {
        let mut res = vec![];
        let types = vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        for type_blood in types {
            if self.can_receive_from(&BloodType { antigen: type_blood.clone(), rh_factor: RhFactor::Negative }) {
                res.push(BloodType { antigen: type_blood.clone(), rh_factor: RhFactor::Negative })
            }
            if self.can_receive_from(&BloodType { antigen: type_blood.clone(), rh_factor: RhFactor::Positive }) {
                res.push(BloodType { antigen: type_blood, rh_factor: RhFactor::Positive })
            }
        }
        res
    }
    
    pub fn recipients(&self) -> Vec<Self> {
        let mut res = vec![];
        let types = vec![Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
        for type_blood in types {
            if (BloodType { antigen: type_blood.clone(), rh_factor: RhFactor::Negative }).can_receive_from(&self) {
                res.push(BloodType { antigen: type_blood.clone(), rh_factor: RhFactor::Negative })
            }
            if (BloodType { antigen: type_blood.clone(), rh_factor: RhFactor::Positive }).can_receive_from(&self) {
                res.push(BloodType { antigen: type_blood, rh_factor: RhFactor::Positive })
            }
        }
        res
    }
}
