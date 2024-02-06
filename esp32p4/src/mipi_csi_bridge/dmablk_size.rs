#[doc = "Register `DMABLK_SIZE` reader"]
pub type R = crate::R<DMABLK_SIZE_SPEC>;
#[doc = "Register `DMABLK_SIZE` writer"]
pub type W = crate::W<DMABLK_SIZE_SPEC>;
#[doc = "Field `DMABLK_SIZE` reader - the number of reg_dma_burst_len in a block"]
pub type DMABLK_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `DMABLK_SIZE` writer - the number of reg_dma_burst_len in a block"]
pub type DMABLK_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - the number of reg_dma_burst_len in a block"]
    #[inline(always)]
    pub fn dmablk_size(&self) -> DMABLK_SIZE_R {
        DMABLK_SIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMABLK_SIZE")
            .field(
                "dmablk_size",
                &format_args!("{}", self.dmablk_size().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMABLK_SIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:12 - the number of reg_dma_burst_len in a block"]
    #[inline(always)]
    #[must_use]
    pub fn dmablk_size(&mut self) -> DMABLK_SIZE_W<DMABLK_SIZE_SPEC> {
        DMABLK_SIZE_W::new(self, 0)
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
#[doc = "DMA block size configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmablk_size::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmablk_size::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMABLK_SIZE_SPEC;
impl crate::RegisterSpec for DMABLK_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmablk_size::R`](R) reader structure"]
impl crate::Readable for DMABLK_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmablk_size::W`](W) writer structure"]
impl crate::Writable for DMABLK_SIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMABLK_SIZE to value 0x1fff"]
impl crate::Resettable for DMABLK_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x1fff;
}
