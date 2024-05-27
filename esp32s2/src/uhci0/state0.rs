#[doc = "Register `STATE0` reader"]
pub type R = crate::R<STATE0_SPEC>;
#[doc = "Field `INLINK_DSCR_ADDR` reader - This register stores the current receive descriptor's address."]
pub type INLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `IN_DSCR_STATE` reader - Reserved."]
pub type IN_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `IN_STATE` reader - Reserved."]
pub type IN_STATE_R = crate::FieldReader;
#[doc = "Field `INFIFO_CNT_DEBUG` reader - This register stores the number of data bytes in RX FIFO."]
pub type INFIFO_CNT_DEBUG_R = crate::FieldReader;
#[doc = "Field `DECODE_STATE` reader - UHCI decoder status."]
pub type DECODE_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current receive descriptor's address."]
    #[inline(always)]
    pub fn inlink_dscr_addr(&self) -> INLINK_DSCR_ADDR_R {
        INLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - Reserved."]
    #[inline(always)]
    pub fn in_dscr_state(&self) -> IN_DSCR_STATE_R {
        IN_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Reserved."]
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:27 - This register stores the number of data bytes in RX FIFO."]
    #[inline(always)]
    pub fn infifo_cnt_debug(&self) -> INFIFO_CNT_DEBUG_R {
        INFIFO_CNT_DEBUG_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:30 - UHCI decoder status."]
    #[inline(always)]
    pub fn decode_state(&self) -> DECODE_STATE_R {
        DECODE_STATE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE0")
            .field("inlink_dscr_addr", &self.inlink_dscr_addr())
            .field("in_dscr_state", &self.in_dscr_state())
            .field("in_state", &self.in_state())
            .field("infifo_cnt_debug", &self.infifo_cnt_debug())
            .field("decode_state", &self.decode_state())
            .finish()
    }
}
#[doc = "UHCI decoder status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE0_SPEC;
impl crate::RegisterSpec for STATE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state0::R`](R) reader structure"]
impl crate::Readable for STATE0_SPEC {}
#[doc = "`reset()` method sets STATE0 to value 0"]
impl crate::Resettable for STATE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
