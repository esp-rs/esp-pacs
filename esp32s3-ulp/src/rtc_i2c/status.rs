#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `ACK_REC` reader - ack response"]
pub type ACK_REC_R = crate::BitReader;
#[doc = "Field `SLAVE_RW` reader - slave read or write"]
pub type SLAVE_RW_R = crate::BitReader;
#[doc = "Field `ARB_LOST` reader - arbitration is lost"]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` reader - bus is busy"]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDRESSED` reader - slave reg sub address"]
pub type SLAVE_ADDRESSED_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS` reader - One byte transit done"]
pub type BYTE_TRANS_R = crate::BitReader;
#[doc = "Field `OP_CNT` reader - which operation is working"]
pub type OP_CNT_R = crate::FieldReader;
#[doc = "Field `SHIFT` reader - shifter content"]
pub type SHIFT_R = crate::FieldReader;
#[doc = "Field `SCL_MAIN_STATE_LAST` reader - i2c last main status"]
pub type SCL_MAIN_STATE_LAST_R = crate::FieldReader;
#[doc = "Field `SCL_STATE_LAST` reader - scl last status"]
pub type SCL_STATE_LAST_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - ack response"]
    #[inline(always)]
    pub fn ack_rec(&self) -> ACK_REC_R {
        ACK_REC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - slave read or write"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - arbitration is lost"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - bus is busy"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - slave reg sub address"]
    #[inline(always)]
    pub fn slave_addressed(&self) -> SLAVE_ADDRESSED_R {
        SLAVE_ADDRESSED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - One byte transit done"]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - which operation is working"]
    #[inline(always)]
    pub fn op_cnt(&self) -> OP_CNT_R {
        OP_CNT_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 16:23 - shifter content"]
    #[inline(always)]
    pub fn shift(&self) -> SHIFT_R {
        SHIFT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - i2c last main status"]
    #[inline(always)]
    pub fn scl_main_state_last(&self) -> SCL_MAIN_STATE_LAST_R {
        SCL_MAIN_STATE_LAST_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - scl last status"]
    #[inline(always)]
    pub fn scl_state_last(&self) -> SCL_STATE_LAST_R {
        SCL_STATE_LAST_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("ack_rec", &self.ack_rec())
            .field("slave_rw", &self.slave_rw())
            .field("arb_lost", &self.arb_lost())
            .field("bus_busy", &self.bus_busy())
            .field("slave_addressed", &self.slave_addressed())
            .field("byte_trans", &self.byte_trans())
            .field("op_cnt", &self.op_cnt())
            .field("shift", &self.shift())
            .field("scl_main_state_last", &self.scl_main_state_last())
            .field("scl_state_last", &self.scl_state_last())
            .finish()
    }
}
#[doc = "get i2c status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
