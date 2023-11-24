#[doc = "Register `SDIO_1` reader"]
pub type R = crate::R<SDIO_1_SPEC>;
#[doc = "Register `SDIO_1` writer"]
pub type W = crate::W<SDIO_1_SPEC>;
#[doc = "Field `SDIO_DISABLE` reader - Setting to 1 disables the SDIO function."]
pub type SDIO_DISABLE_R = crate::BitReader;
#[doc = "Field `SDIO_DISABLE` writer - Setting to 1 disables the SDIO function."]
pub type SDIO_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Setting to 1 disables the SDIO function."]
    #[inline(always)]
    pub fn sdio_disable(&self) -> SDIO_DISABLE_R {
        SDIO_DISABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_1")
            .field(
                "sdio_disable",
                &format_args!("{}", self.sdio_disable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDIO_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Setting to 1 disables the SDIO function."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_disable(&mut self) -> SDIO_DISABLE_W<SDIO_1_SPEC> {
        SDIO_DISABLE_W::new(self, 0)
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
#[doc = "SDIO permission control register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_1_SPEC;
impl crate::RegisterSpec for SDIO_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_1::R`](R) reader structure"]
impl crate::Readable for SDIO_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_1::W`](W) writer structure"]
impl crate::Writable for SDIO_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SDIO_1 to value 0"]
impl crate::Resettable for SDIO_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
