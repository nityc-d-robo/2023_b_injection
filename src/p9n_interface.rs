use safe_drive::msg::common_interfaces::sensor_msgs;
use crate::ps4_dualshock4::AXES_DUALSHOCK4;
use crate::ps4_dualshock4::BUTTONS_DUALSHOCK4;

pub struct DualShock4Interface {
    msg: sensor_msgs::msg::Joy,
}

impl DualShock4Interface {
    pub fn new(_msg: sensor_msgs::msg::Joy) -> DualShock4Interface {
        DualShock4Interface { msg: _msg, }
    }
    pub fn set_joy_msg(&mut self, _msg: sensor_msgs::msg::Joy){
        self.msg = _msg;
    }
    pub fn pressed_PS(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::PS] == 1
    }
    pub fn pressed_l1(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::L1] == 1
    }
    pub fn pressed_r1(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::R1] == 1
    }
    pub fn pressed_r2(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::R2] == 1
    }
    pub fn pressed_l2(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_DUALSHOCK4::L2] == 1
    }
    pub fn pressed_dpad_left(&self) -> bool {
         self.msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_X] > 0.0
    }
    pub fn pressed_dpad_up(&self) -> bool {
        self.msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_Y] > 0.0
    }
    pub fn pressed_dpad_right(&self) -> bool {
        self.msg.axes.as_slice()[AXES_DUALSHOCK4::DPAD_X] < 0.0
    }
}