#[doc = "Register `OUT_ST` reader"]
pub type R = crate::R<OUT_ST_SPEC>;
#[doc = "Field `OUTLINK_DSCR_ADDR` reader - This register stores the current transmit descriptor’s address."]
pub type OUTLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DSCR_STATE` reader - Reserved"]
pub type OUT_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `OUT_STATE` reader - Reserved"]
pub type OUT_STATE_R = crate::FieldReader;
#[doc = "Field `FIFO_FULL` reader - Copy DMA FIFO full signal."]
pub type FIFO_FULL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current transmit descriptor’s address."]
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - Reserved"]
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Reserved"]
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Copy DMA FIFO full signal."]
    #[inline(always)]
    pub fn fifo_full(&self) -> FIFO_FULL_R {
        FIFO_FULL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_ST")
            .field("outlink_dscr_addr", &self.outlink_dscr_addr())
            .field("out_dscr_state", &self.out_dscr_state())
            .field("out_state", &self.out_state())
            .field("fifo_full", &self.fifo_full())
            .finish()
    }
}
#[doc = "Status register of transmitting data\n\nYou can [`read`](crate::Reg::read) this register and get [`out_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ST_SPEC;
impl crate::RegisterSpec for OUT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_st::R`](R) reader structure"]
impl crate::Readable for OUT_ST_SPEC {}
#[doc = "`reset()` method sets OUT_ST to value 0"]
impl crate::Resettable for OUT_ST_SPEC {}
