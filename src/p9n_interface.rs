use safe_drive::msg::common_interfaces::sensor_msgs;
use crate::pro_controller::AXES_SWITCH_PRO;
use crate::pro_controller::BUTTONS_SWITCH_PRO;

pub struct NintendoSwitchInterface {
    msg: sensor_msgs::msg::Joy,
}

impl NintendoSwitchInterface {
    pub fn new(_msg: sensor_msgs::msg::Joy) -> NintendoSwitchInterface {
        NintendoSwitchInterface { msg: _msg, }
    }
    pub fn set_joy_msg(&mut self, _msg: sensor_msgs::msg::Joy){
        self.msg = _msg;
    }
    pub fn pressed_start(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_SWITCH_PRO::PLUS] == 1
    }
    pub fn pressed_l(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_SWITCH_PRO::L] == 1
    }
    pub fn pressed_r(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_SWITCH_PRO::R] == 1
    }
    pub fn pressed_zl(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_SWITCH_PRO::ZL] == 1
    }
    pub fn pressed_zr(&self) -> bool {
        self.msg.buttons.as_slice()[BUTTONS_SWITCH_PRO::ZR] == 1
    }
    pub fn pressed_dpad_left(&self) -> bool {
         self.msg.axes.as_slice()[AXES_SWITCH_PRO::DPAD_X] > 0.0
    }
    pub fn pressed_dpad_up(&self) -> bool {
        self.msg.axes.as_slice()[AXES_SWITCH_PRO::DPAD_Y] > 0.0
    }
    pub fn pressed_dpad_right(&self) -> bool {
        self.msg.axes.as_slice()[AXES_SWITCH_PRO::DPAD_X] < 0.0
    }
}