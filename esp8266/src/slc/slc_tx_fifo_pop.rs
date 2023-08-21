#[doc = "Register `SLC_TX_FIFO_POP` reader"]
pub type R = crate::R<SLC_TX_FIFO_POP_SPEC>;
#[doc = "Register `SLC_TX_FIFO_POP` writer"]
pub type W = crate::W<SLC_TX_FIFO_POP_SPEC>;
#[doc = "Field `SLC_TXFIFO_RDATA` reader - "]
pub type SLC_TXFIFO_RDATA_R = crate::FieldReader<u16>;
#[doc = "Field `SLC_TXFIFO_RDATA` writer - "]
pub type SLC_TXFIFO_RDATA_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 11, O, u16>;
#[doc = "Field `SLC_TXFIFO_POP` reader - "]
pub type SLC_TXFIFO_POP_R = crate::BitReader;
#[doc = "Field `SLC_TXFIFO_POP` writer - "]
pub type SLC_TXFIFO_POP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc_txfifo_rdata(&self) -> SLC_TXFIFO_RDATA_R {
        SLC_TXFIFO_RDATA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc_txfifo_pop(&self) -> SLC_TXFIFO_POP_R {
        SLC_TXFIFO_POP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC_TX_FIFO_POP")
            .field(
                "slc_txfifo_pop",
                &format_args!("{}", self.slc_txfifo_pop().bit()),
            )
            .field(
                "slc_txfifo_rdata",
                &format_args!("{}", self.slc_txfifo_rdata().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC_TX_FIFO_POP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txfifo_rdata(&mut self) -> SLC_TXFIFO_RDATA_W<SLC_TX_FIFO_POP_SPEC, 0> {
        SLC_TXFIFO_RDATA_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc_txfifo_pop(&mut self) -> SLC_TXFIFO_POP_W<SLC_TX_FIFO_POP_SPEC, 16> {
        SLC_TXFIFO_POP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SLC_TX_FIFO_POP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc_tx_fifo_pop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc_tx_fifo_pop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC_TX_FIFO_POP_SPEC;
impl crate::RegisterSpec for SLC_TX_FIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc_tx_fifo_pop::R`](R) reader structure"]
impl crate::Readable for SLC_TX_FIFO_POP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc_tx_fifo_pop::W`](W) writer structure"]
impl crate::Writable for SLC_TX_FIFO_POP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLC_TX_FIFO_POP to value 0"]
impl crate::Resettable for SLC_TX_FIFO_POP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
