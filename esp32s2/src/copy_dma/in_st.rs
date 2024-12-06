#[doc = "Register `IN_ST` reader"]
pub type R = crate::R<IN_ST_SPEC>;
#[doc = "Field `INLINK_DSCR_ADDR` reader - This register stores the current receive descriptor’s address."]
pub type INLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `IN_DSCR_STATE` reader - Reserved"]
pub type IN_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `IN_STATE` reader - Reserved"]
pub type IN_STATE_R = crate::FieldReader;
#[doc = "Field `FIFO_EMPTY` reader - Copy DMA FIFO empty signal."]
pub type FIFO_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current receive descriptor’s address."]
    #[inline(always)]
    pub fn inlink_dscr_addr(&self) -> INLINK_DSCR_ADDR_R {
        INLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - Reserved"]
    #[inline(always)]
    pub fn in_dscr_state(&self) -> IN_DSCR_STATE_R {
        IN_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Reserved"]
    #[inline(always)]
    pub fn in_state(&self) -> IN_STATE_R {
        IN_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Copy DMA FIFO empty signal."]
    #[inline(always)]
    pub fn fifo_empty(&self) -> FIFO_EMPTY_R {
        FIFO_EMPTY_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ST")
            .field("inlink_dscr_addr", &self.inlink_dscr_addr())
            .field("in_dscr_state", &self.in_dscr_state())
            .field("in_state", &self.in_state())
            .field("fifo_empty", &self.fifo_empty())
            .finish()
    }
}
#[doc = "Status register of receiving data\n\nYou can [`read`](crate::Reg::read) this register and get [`in_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ST_SPEC;
impl crate::RegisterSpec for IN_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_st::R`](R) reader structure"]
impl crate::Readable for IN_ST_SPEC {}
#[doc = "`reset()` method sets IN_ST to value 0"]
impl crate::Resettable for IN_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
