mod p9n_interface;
mod ps4_dualshock4;
use drobo_interfaces::msg::MdLibMsg;
use safe_drive::{
    context::Context,
    error::DynError,
    logger::Logger,
    msg::common_interfaces::{sensor_msgs, std_msgs},
    pr_fatal, pr_info,
    selector::Selector,
    topic::{publisher::{self, Publisher}, subscriber::Subscriber},
    RecvResult,
};
use std::{rc::Rc, time::Duration};

pub mod DealShock4state {
    pub const B: usize = 0;
    pub const A: usize = 1;
    pub const X: usize = 2;
    pub const Y: usize = 3;
    pub const SCROT: usize = 4;
    pub const L: usize = 5;
    pub const R: usize = 6;
    pub const ZL: usize = 7;
    pub const ZR: usize = 8;
    pub const D_PAD_UP: usize = 8;
    //pub const D_PAD_DOWN: usize = 9;
    //pub const D_PAD_LEFT: usize = 10;
    //pub const D_PAD_RIGHT: usize = 11;
    //pub const START: usize = 12;
    //pub const SELECT: usize = 13;
    //pub const PS: usize = 14;
}

fn main() -> Result<(), DynError> {
    let ctx = Context::new()?;
    let node = ctx.create_node("injection_2023_b", None, Default::default())?;

    let selector = ctx.create_selector()?;
    let subscriber = node.create_subscriber::<sensor_msgs::msg::Joy>("joy", None)?;
    let publisher = node.create_publisher::<drobo_interfaces::msg::MdLibMsg>("md_driver_topic", None)?;
    
    worker(
        selector,
        subscriber,                               
        publisher
    )?;
    Ok(())
}

fn worker(
    mut selector: Selector,
    subscriber: Subscriber<sensor_msgs::msg::Joy>,
    publisher:Publisher<MdLibMsg>
    
) -> Result<(), DynError> {
    let mut p9n = p9n_interface::DualShock4Interface::new(sensor_msgs::msg::Joy::new().unwrap());
    let logger = Rc::new(Logger::new("injection_2023_b"));
   
    
    selector.add_subscriber(
        subscriber,
        Box::new(move |_msg| {
            p9n.set_joy_msg(_msg.get_owned().unwrap());

            if p9n.pressed_r2(){
                send_pwm(0x04, 0, false, 400,&publisher, /* publisher */);
                send_pwm(0x05, 0, true, 400,&publisher, /* publisher */);
                pr_info!(logger, "injection run", );
            }
            if !p9n.pressed_r2(){
                send_pwm(0x04, 0, false, 0,&publisher, /* publisher */);
                send_pwm(0x05, 0, true, 0,&publisher, /* publisher */);
                pr_info!(logger, "injection stop", );
            }
        
            
            if p9n.pressed_r1(){
                send_pwm(0x07, 0, true, 800,&publisher, /* publisher */);
                pr_info!(logger, "装填 閉じる", );
            }
            if !p9n.pressed_r1(){
                send_pwm(0x07, 0, true, 0, &publisher, /* publisher */);
                pr_info!(logger, "装填 stop", );
            }
            if p9n.pressed_l1(){
                send_pwm(0x07, 0, false, 800,&publisher, /* publisher */);
                pr_info!(logger, "装填 開く", );
            }
            if !p9n.pressed_l1(){
                send_pwm(0x07, 0, false, 0,&publisher, /* publisher */);
                pr_info!(logger, "装填 stop", );
            }

            
            
           
        }),
    );
    loop {
        selector.wait()?;
    }

}
fn send_speed(_address:u32, _semi_id:u32,_phase:bool,_speed:u32,_angle:i32,_timeout:u16,publisher:&Publisher<MdLibMsg>){
    let mut msg = drobo_interfaces::msg::MdLibMsg::new().unwrap();
    msg.address = _address as u8;
    msg.semi_id = _semi_id as u8;
    msg.mode = 3 as u8; //MotorLibのspeedモードに倣いました
    msg.phase = _phase as bool;
    msg.power = _speed as u16;
    msg.angle = _angle as i32;
    msg.timeout = _timeout as u16;

    publisher.send(&msg).unwrap()

}

fn send_pwm(_address:u32, _semi_id:u32,_phase:bool,_power:u16,publisher:&Publisher<MdLibMsg>){
    let mut msg = drobo_interfaces::msg::MdLibMsg::new().unwrap();
    msg.address = _address as u8;
    msg.semi_id = _semi_id as u8;
    msg.mode = 2 as u8; //MotorLibのPWMモードに倣いました
    msg.phase = _phase as bool;
    msg.power = _power as u16;
    

    publisher.send(&msg).unwrap()

}

