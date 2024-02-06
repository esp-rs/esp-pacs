#[doc = "Register `PRESENT_STATE0` reader"]
pub type R = crate::R<PRESENT_STATE0_SPEC>;
#[doc = "Field `SDA_LVL` reader - This bit is used to check the SCL line level to recover from error and for debugging. This bit reflects the value of synchronized scl_in_a."]
pub type SDA_LVL_R = crate::BitReader;
#[doc = "Field `SCL_LVL` reader - This bit is used to check the SDA line level to recover from error and for debugging. This bit reflects the value of synchronized sda_in_a."]
pub type SCL_LVL_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` reader - NA"]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `BUS_FREE` reader - NA"]
pub type BUS_FREE_R = crate::BitReader;
#[doc = "Field `CMD_TID` reader - NA"]
pub type CMD_TID_R = crate::FieldReader;
#[doc = "Field `SCL_GEN_FSM_STATE` reader - NA"]
pub type SCL_GEN_FSM_STATE_R = crate::FieldReader;
#[doc = "Field `IBI_EV_HANDLE_FSM_STATE` reader - NA"]
pub type IBI_EV_HANDLE_FSM_STATE_R = crate::FieldReader;
#[doc = "Field `I2C_MODE_FSM_STATE` reader - NA"]
pub type I2C_MODE_FSM_STATE_R = crate::FieldReader;
#[doc = "Field `SDR_MODE_FSM_STATE` reader - NA"]
pub type SDR_MODE_FSM_STATE_R = crate::FieldReader;
#[doc = "Field `DAA_MODE_FSM_STATE` reader - Reflects whether the Master Controller is in IDLE or not. This bit will be set when all the buffer(Command, Response, IBI, Transmit, Receive) are empty along with the Master State machine is in idle state. 0X0: not in idle 0x1: in idle"]
pub type DAA_MODE_FSM_STATE_R = crate::FieldReader;
#[doc = "Field `MAIN_FSM_STATE` reader - NA"]
pub type MAIN_FSM_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This bit is used to check the SCL line level to recover from error and for debugging. This bit reflects the value of synchronized scl_in_a."]
    #[inline(always)]
    pub fn sda_lvl(&self) -> SDA_LVL_R {
        SDA_LVL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is used to check the SDA line level to recover from error and for debugging. This bit reflects the value of synchronized sda_in_a."]
    #[inline(always)]
    pub fn scl_lvl(&self) -> SCL_LVL_R {
        SCL_LVL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn bus_free(&self) -> BUS_FREE_R {
        BUS_FREE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 9:12 - NA"]
    #[inline(always)]
    pub fn cmd_tid(&self) -> CMD_TID_R {
        CMD_TID_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - NA"]
    #[inline(always)]
    pub fn scl_gen_fsm_state(&self) -> SCL_GEN_FSM_STATE_R {
        SCL_GEN_FSM_STATE_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:18 - NA"]
    #[inline(always)]
    pub fn ibi_ev_handle_fsm_state(&self) -> IBI_EV_HANDLE_FSM_STATE_R {
        IBI_EV_HANDLE_FSM_STATE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - NA"]
    #[inline(always)]
    pub fn i2c_mode_fsm_state(&self) -> I2C_MODE_FSM_STATE_R {
        I2C_MODE_FSM_STATE_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:25 - NA"]
    #[inline(always)]
    pub fn sdr_mode_fsm_state(&self) -> SDR_MODE_FSM_STATE_R {
        SDR_MODE_FSM_STATE_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:28 - Reflects whether the Master Controller is in IDLE or not. This bit will be set when all the buffer(Command, Response, IBI, Transmit, Receive) are empty along with the Master State machine is in idle state. 0X0: not in idle 0x1: in idle"]
    #[inline(always)]
    pub fn daa_mode_fsm_state(&self) -> DAA_MODE_FSM_STATE_R {
        DAA_MODE_FSM_STATE_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - NA"]
    #[inline(always)]
    pub fn main_fsm_state(&self) -> MAIN_FSM_STATE_R {
        MAIN_FSM_STATE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESENT_STATE0")
            .field("sda_lvl", &format_args!("{}", self.sda_lvl().bit()))
            .field("scl_lvl", &format_args!("{}", self.scl_lvl().bit()))
            .field("bus_busy", &format_args!("{}", self.bus_busy().bit()))
            .field("bus_free", &format_args!("{}", self.bus_free().bit()))
            .field("cmd_tid", &format_args!("{}", self.cmd_tid().bits()))
            .field(
                "scl_gen_fsm_state",
                &format_args!("{}", self.scl_gen_fsm_state().bits()),
            )
            .field(
                "ibi_ev_handle_fsm_state",
                &format_args!("{}", self.ibi_ev_handle_fsm_state().bits()),
            )
            .field(
                "i2c_mode_fsm_state",
                &format_args!("{}", self.i2c_mode_fsm_state().bits()),
            )
            .field(
                "sdr_mode_fsm_state",
                &format_args!("{}", self.sdr_mode_fsm_state().bits()),
            )
            .field(
                "daa_mode_fsm_state",
                &format_args!("{}", self.daa_mode_fsm_state().bits()),
            )
            .field(
                "main_fsm_state",
                &format_args!("{}", self.main_fsm_state().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRESENT_STATE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`present_state0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESENT_STATE0_SPEC;
impl crate::RegisterSpec for PRESENT_STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`present_state0::R`](R) reader structure"]
impl crate::Readable for PRESENT_STATE0_SPEC {}
#[doc = "`reset()` method sets PRESENT_STATE0 to value 0x03"]
impl crate::Resettable for PRESENT_STATE0_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
