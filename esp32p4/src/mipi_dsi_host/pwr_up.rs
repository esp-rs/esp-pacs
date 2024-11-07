#[doc = "Register `PWR_UP` reader"]
pub type R = crate::R<PWR_UP_SPEC>;
#[doc = "Register `PWR_UP` writer"]
pub type W = crate::W<PWR_UP_SPEC>;
#[doc = "Field `SHUTDOWNZ` reader - NA"]
pub type SHUTDOWNZ_R = crate::BitReader;
#[doc = "Field `SHUTDOWNZ` writer - NA"]
pub type SHUTDOWNZ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn shutdownz(&self) -> SHUTDOWNZ_R {
        SHUTDOWNZ_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_UP")
            .field("shutdownz", &self.shutdownz())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn shutdownz(&mut self) -> SHUTDOWNZ_W<PWR_UP_SPEC> {
        SHUTDOWNZ_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr_up::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_up::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_UP_SPEC;
impl crate::RegisterSpec for PWR_UP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_up::R`](R) reader structure"]
impl crate::Readable for PWR_UP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwr_up::W`](W) writer structure"]
impl crate::Writable for PWR_UP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWR_UP to value 0"]
impl crate::Resettable for PWR_UP_SPEC {
    const RESET_VALUE: u32 = 0;
}
