#[doc = "Register `TIMEOUT` reader"]
pub type R = crate::R<TIMEOUT_SPEC>;
#[doc = "Register `TIMEOUT` writer"]
pub type W = crate::W<TIMEOUT_SPEC>;
#[doc = "Field `TIMEOUT` reader - Maximum number of FAST_CLK cycles that the transmission can take"]
pub type TIMEOUT_R = crate::FieldReader<u32>;
#[doc = "Field `TIMEOUT` writer - Maximum number of FAST_CLK cycles that the transmission can take"]
pub type TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Maximum number of FAST_CLK cycles that the transmission can take"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT")
            .field("timeout", &self.timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - Maximum number of FAST_CLK cycles that the transmission can take"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<TIMEOUT_SPEC> {
        TIMEOUT_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_SPEC;
impl crate::RegisterSpec for TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMEOUT to value 0"]
impl crate::Resettable for TIMEOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
