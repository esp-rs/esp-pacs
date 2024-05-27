#[doc = "Register `SDIO_0` reader"]
pub type R = crate::R<SDIO_0_SPEC>;
#[doc = "Register `SDIO_0` writer"]
pub type W = crate::W<SDIO_0_SPEC>;
#[doc = "Field `SDIO_LOCK` reader - Lock register. Setting to 1 locks SDIO permission control registers."]
pub type SDIO_LOCK_R = crate::BitReader;
#[doc = "Field `SDIO_LOCK` writer - Lock register. Setting to 1 locks SDIO permission control registers."]
pub type SDIO_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks SDIO permission control registers."]
    #[inline(always)]
    pub fn sdio_lock(&self) -> SDIO_LOCK_R {
        SDIO_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_0")
            .field("sdio_lock", &self.sdio_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks SDIO permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn sdio_lock(&mut self) -> SDIO_LOCK_W<SDIO_0_SPEC> {
        SDIO_LOCK_W::new(self, 0)
    }
}
#[doc = "SDIO permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_0_SPEC;
impl crate::RegisterSpec for SDIO_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_0::R`](R) reader structure"]
impl crate::Readable for SDIO_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_0::W`](W) writer structure"]
impl crate::Writable for SDIO_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO_0 to value 0"]
impl crate::Resettable for SDIO_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
