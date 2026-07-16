#[doc = "Register `TIMEOUT_LIMIT` reader"]
pub type R = crate::R<TIMEOUT_LIMIT_SPEC>;
#[doc = "Register `TIMEOUT_LIMIT` writer"]
pub type W = crate::W<TIMEOUT_LIMIT_SPEC>;
#[doc = "Field `TIMEOUT_LIMIT` reader - The timeout limitation for waiting sub-IP."]
pub type TIMEOUT_LIMIT_R = crate::FieldReader<u32>;
#[doc = "Field `TIMEOUT_LIMIT` writer - The timeout limitation for waiting sub-IP."]
pub type TIMEOUT_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The timeout limitation for waiting sub-IP."]
    #[inline(always)]
    pub fn timeout_limit(&self) -> TIMEOUT_LIMIT_R {
        TIMEOUT_LIMIT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMEOUT_LIMIT")
            .field("timeout_limit", &self.timeout_limit())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - The timeout limitation for waiting sub-IP."]
    #[inline(always)]
    pub fn timeout_limit(&mut self) -> TIMEOUT_LIMIT_W<'_, TIMEOUT_LIMIT_SPEC> {
        TIMEOUT_LIMIT_W::new(self, 0)
    }
}
#[doc = "KEYMNG timeout limit configurate register\n\nYou can [`read`](crate::Reg::read) this register and get [`timeout_limit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timeout_limit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMEOUT_LIMIT_SPEC;
impl crate::RegisterSpec for TIMEOUT_LIMIT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timeout_limit::R`](R) reader structure"]
impl crate::Readable for TIMEOUT_LIMIT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timeout_limit::W`](W) writer structure"]
impl crate::Writable for TIMEOUT_LIMIT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMEOUT_LIMIT to value 0xffff_ffff"]
impl crate::Resettable for TIMEOUT_LIMIT_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
