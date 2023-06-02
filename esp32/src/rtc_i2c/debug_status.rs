#[doc = "Register `DEBUG_STATUS` reader"]
pub struct R(crate::R<DEBUG_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_STATUS` writer"]
pub struct W(crate::W<DEBUG_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DEBUG_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK_VAL` reader - The value of an acknowledge signal on the bus"]
pub type ACK_VAL_R = crate::BitReader;
#[doc = "Field `ACK_VAL` writer - The value of an acknowledge signal on the bus"]
pub type ACK_VAL_W<'a, const O: u8> = crate::BitWriter<'a, DEBUG_STATUS_SPEC, O>;
#[doc = "Field `SLAVE_RW` reader - When working as a slave, the value of R/W bit received"]
pub type SLAVE_RW_R = crate::BitReader;
#[doc = "Field `SLAVE_RW` writer - When working as a slave, the value of R/W bit received"]
pub type SLAVE_RW_W<'a, const O: u8> = crate::BitWriter<'a, DEBUG_STATUS_SPEC, O>;
#[doc = "Field `TIMED_OUT` reader - Transfer has timed out"]
pub type TIMED_OUT_R = crate::BitReader;
#[doc = "Field `TIMED_OUT` writer - Transfer has timed out"]
pub type TIMED_OUT_W<'a, const O: u8> = crate::BitWriter<'a, DEBUG_STATUS_SPEC, O>;
#[doc = "Field `ARB_LOST` reader - When working as a master, lost control of I2C bus"]
pub type ARB_LOST_R = crate::BitReader;
#[doc = "Field `ARB_LOST` writer - When working as a master, lost control of I2C bus"]
pub type ARB_LOST_W<'a, const O: u8> = crate::BitWriter<'a, DEBUG_STATUS_SPEC, O>;
#[doc = "Field `BUS_BUSY` reader - operation is in progress"]
pub type BUS_BUSY_R = crate::BitReader;
#[doc = "Field `BUS_BUSY` writer - operation is in progress"]
pub type BUS_BUSY_W<'a, const O: u8> = crate::BitWriter<'a, DEBUG_STATUS_SPEC, O>;
#[doc = "Field `SLAVE_ADDR_MATCH` reader - When working as a slave, whether address was matched"]
pub type SLAVE_ADDR_MATCH_R = crate::BitReader;
#[doc = "Field `SLAVE_ADDR_MATCH` writer - When working as a slave, whether address was matched"]
pub type SLAVE_ADDR_MATCH_W<'a, const O: u8> = crate::BitWriter<'a, DEBUG_STATUS_SPEC, O>;
#[doc = "Field `BYTE_TRANS` reader - 8 bit transmit done"]
pub type BYTE_TRANS_R = crate::BitReader;
#[doc = "Field `BYTE_TRANS` writer - 8 bit transmit done"]
pub type BYTE_TRANS_W<'a, const O: u8> = crate::BitWriter<'a, DEBUG_STATUS_SPEC, O>;
#[doc = "Field `MAIN_STATE` reader - state of the main state machine"]
pub type MAIN_STATE_R = crate::FieldReader;
#[doc = "Field `MAIN_STATE` writer - state of the main state machine"]
pub type MAIN_STATE_W<'a, const O: u8> = crate::FieldWriter<'a, DEBUG_STATUS_SPEC, 3, O>;
#[doc = "Field `SCL_STATE` reader - state of SCL state machine"]
pub type SCL_STATE_R = crate::FieldReader;
#[doc = "Field `SCL_STATE` writer - state of SCL state machine"]
pub type SCL_STATE_W<'a, const O: u8> = crate::FieldWriter<'a, DEBUG_STATUS_SPEC, 3, O>;
impl R {
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    pub fn ack_val(&self) -> ACK_VAL_R {
        ACK_VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    pub fn slave_rw(&self) -> SLAVE_RW_R {
        SLAVE_RW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    pub fn timed_out(&self) -> TIMED_OUT_R {
        TIMED_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    pub fn arb_lost(&self) -> ARB_LOST_R {
        ARB_LOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    pub fn bus_busy(&self) -> BUS_BUSY_R {
        BUS_BUSY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    pub fn slave_addr_match(&self) -> SLAVE_ADDR_MATCH_R {
        SLAVE_ADDR_MATCH_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    pub fn byte_trans(&self) -> BYTE_TRANS_R {
        BYTE_TRANS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    pub fn main_state(&self) -> MAIN_STATE_R {
        MAIN_STATE_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    pub fn scl_state(&self) -> SCL_STATE_R {
        SCL_STATE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_STATUS")
            .field("ack_val", &format_args!("{}", self.ack_val().bit()))
            .field("slave_rw", &format_args!("{}", self.slave_rw().bit()))
            .field("timed_out", &format_args!("{}", self.timed_out().bit()))
            .field("arb_lost", &format_args!("{}", self.arb_lost().bit()))
            .field("bus_busy", &format_args!("{}", self.bus_busy().bit()))
            .field(
                "slave_addr_match",
                &format_args!("{}", self.slave_addr_match().bit()),
            )
            .field("byte_trans", &format_args!("{}", self.byte_trans().bit()))
            .field("main_state", &format_args!("{}", self.main_state().bits()))
            .field("scl_state", &format_args!("{}", self.scl_state().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DEBUG_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The value of an acknowledge signal on the bus"]
    #[inline(always)]
    #[must_use]
    pub fn ack_val(&mut self) -> ACK_VAL_W<0> {
        ACK_VAL_W::new(self)
    }
    #[doc = "Bit 1 - When working as a slave, the value of R/W bit received"]
    #[inline(always)]
    #[must_use]
    pub fn slave_rw(&mut self) -> SLAVE_RW_W<1> {
        SLAVE_RW_W::new(self)
    }
    #[doc = "Bit 2 - Transfer has timed out"]
    #[inline(always)]
    #[must_use]
    pub fn timed_out(&mut self) -> TIMED_OUT_W<2> {
        TIMED_OUT_W::new(self)
    }
    #[doc = "Bit 3 - When working as a master, lost control of I2C bus"]
    #[inline(always)]
    #[must_use]
    pub fn arb_lost(&mut self) -> ARB_LOST_W<3> {
        ARB_LOST_W::new(self)
    }
    #[doc = "Bit 4 - operation is in progress"]
    #[inline(always)]
    #[must_use]
    pub fn bus_busy(&mut self) -> BUS_BUSY_W<4> {
        BUS_BUSY_W::new(self)
    }
    #[doc = "Bit 5 - When working as a slave, whether address was matched"]
    #[inline(always)]
    #[must_use]
    pub fn slave_addr_match(&mut self) -> SLAVE_ADDR_MATCH_W<5> {
        SLAVE_ADDR_MATCH_W::new(self)
    }
    #[doc = "Bit 6 - 8 bit transmit done"]
    #[inline(always)]
    #[must_use]
    pub fn byte_trans(&mut self) -> BYTE_TRANS_W<6> {
        BYTE_TRANS_W::new(self)
    }
    #[doc = "Bits 25:27 - state of the main state machine"]
    #[inline(always)]
    #[must_use]
    pub fn main_state(&mut self) -> MAIN_STATE_W<25> {
        MAIN_STATE_W::new(self)
    }
    #[doc = "Bits 28:30 - state of SCL state machine"]
    #[inline(always)]
    #[must_use]
    pub fn scl_state(&mut self) -> SCL_STATE_W<28> {
        SCL_STATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_status](index.html) module"]
pub struct DEBUG_STATUS_SPEC;
impl crate::RegisterSpec for DEBUG_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_status::R](R) reader structure"]
impl crate::Readable for DEBUG_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_status::W](W) writer structure"]
impl crate::Writable for DEBUG_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_STATUS to value 0"]
impl crate::Resettable for DEBUG_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
