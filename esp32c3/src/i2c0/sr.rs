#[doc = "Register `SR` reader"]
pub type R = crate::R<SR_SPEC>;
#[doc = "Field `RESP_REC` reader - reg_resp_rec"]
pub type RESP_REC_R = crate::BitReader;
#[doc = "Field `SLAVE_RW` reader - reg_slave_rw"]
pub type SLAVE_RW_R = crate::BitReader;
#[doc = "Field `ARB_LOST` reader - reg_arb_lost"]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` reader - reg_bus_busy"]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDRESSED` reader - reg_slave_addressed"]
pub type SLAVE_ADDRESSED_R = crate::BitReader;
#[doc = "Field `RXFIFO_CNT` reader - reg_rxfifo_cnt"]
pub type RXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `STRETCH_CAUSE` reader - reg_stretch_cause"]
pub type STRETCH_CAUSE_R = crate::FieldReader;
#[doc = "Field `TXFIFO_CNT` reader - reg_txfifo_cnt"]
pub type TXFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - reg_scl_main_state_last"]
pub type SCL_MAIN_STATE_LAST_R = crate::FieldReader;
#[doc = "Field `SCL_STATE_LAST` reader - reg_scl_state_last"]
pub type SCL_STATE_LAST_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - reg_resp_rec"]
    #[inline(always)]
    pub fn resp_rec(&self) -> RESP_REC_R {
        RESP_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_slave_rw"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_arb_lost"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_bus_busy"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_slave_addressed"]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - reg_rxfifo_cnt"]
    #[inline(always)]
    pub fn rxfifo_cnt(&self) -> RXFIFO_CNT_R {
        RXFIFO_CNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - reg_stretch_cause"]
    #[inline(always)]
    pub fn stretch_cause(&self) -> STRETCH_CAUSE_R {
        STRETCH_CAUSE_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 18:23 - reg_txfifo_cnt"]
    #[inline(always)]
    pub fn txfifo_cnt(&self) -> TXFIFO_CNT_R {
        TXFIFO_CNT_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:26 - reg_scl_main_state_last"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - reg_scl_state_last"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("resp_rec", &format_args!("{}", self.resp_rec().bit()))
            .field("slave_rw", &format_args!("{}", self.slave_rw().bit()))
            .field("arb_lost", &format_args!("{}", self.arb_lost().bit()))
            .field("bus_busy", &format_args!("{}", self.bus_busy().bit()))
            .field(
                "slave_addressed",
                &format_args!("{}", self.slave_addressed().bit()),
            )
            .field("rxfifo_cnt", &format_args!("{}", self.rxfifo_cnt().bits()))
            .field(
                "stretch_cause",
                &format_args!("{}", self.stretch_cause().bits()),
            )
            .field("txfifo_cnt", &format_args!("{}", self.txfifo_cnt().bits()))
            .field(
                "scl_main_state_last",
                &format_args!("{}", self.scl_main_state_last().bits()),
            )
            .field(
                "scl_state_last",
                &format_args!("{}", self.scl_state_last().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "I2C_SR_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SR_SPEC {}
#[doc = "`reset()` method sets SR to value 0xc000"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: u32 = 0xc000;
}
