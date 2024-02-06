#[doc = "Register `_1TXFIFO_POP` reader"]
pub type R = crate::R<_1TXFIFO_POP_SPEC>;
#[doc = "Register `_1TXFIFO_POP` writer"]
pub type W = crate::W<_1TXFIFO_POP_SPEC>;
#[doc = "Field `SLC1_TXFIFO_RDATA` reader - "]
pub type SLC1_TXFIFO_RDATA_R = crate::FieldReader<u16>;
#[doc = "Field `SLC1_TXFIFO_POP` reader - "]
pub type SLC1_TXFIFO_POP_R = crate::BitReader;
#[doc = "Field `SLC1_TXFIFO_POP` writer - "]
pub type SLC1_TXFIFO_POP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn slc1_txfifo_rdata(&self) -> SLC1_TXFIFO_RDATA_R {
        SLC1_TXFIFO_RDATA_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_txfifo_pop(&self) -> SLC1_TXFIFO_POP_R {
        SLC1_TXFIFO_POP_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1TXFIFO_POP")
            .field(
                "slc1_txfifo_rdata",
                &format_args!("{}", self.slc1_txfifo_rdata().bits()),
            )
            .field(
                "slc1_txfifo_pop",
                &format_args!("{}", self.slc1_txfifo_pop().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_1TXFIFO_POP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_txfifo_pop(&mut self) -> SLC1_TXFIFO_POP_W<_1TXFIFO_POP_SPEC> {
        SLC1_TXFIFO_POP_W::new(self, 16)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_1txfifo_pop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1txfifo_pop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _1TXFIFO_POP_SPEC;
impl crate::RegisterSpec for _1TXFIFO_POP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_1txfifo_pop::R`](R) reader structure"]
impl crate::Readable for _1TXFIFO_POP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_1txfifo_pop::W`](W) writer structure"]
impl crate::Writable for _1TXFIFO_POP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _1TXFIFO_POP to value 0"]
impl crate::Resettable for _1TXFIFO_POP_SPEC {
    const RESET_VALUE: u32 = 0;
}
