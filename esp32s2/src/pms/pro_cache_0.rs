#[doc = "Register `PRO_CACHE_0` reader"]
pub type R = crate::R<PRO_CACHE_0_SPEC>;
#[doc = "Register `PRO_CACHE_0` writer"]
pub type W = crate::W<PRO_CACHE_0_SPEC>;
#[doc = "Field `PRO_CACHE_LOCK` reader - Lock register. Setting to 1 locks cache permission control registers."]
pub type PRO_CACHE_LOCK_R = crate::BitReader;
#[doc = "Field `PRO_CACHE_LOCK` writer - Lock register. Setting to 1 locks cache permission control registers."]
pub type PRO_CACHE_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks cache permission control registers."]
    #[inline(always)]
    pub fn pro_cache_lock(&self) -> PRO_CACHE_LOCK_R {
        PRO_CACHE_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CACHE_0")
            .field(
                "pro_cache_lock",
                &format_args!("{}", self.pro_cache_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CACHE_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks cache permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn pro_cache_lock(&mut self) -> PRO_CACHE_LOCK_W<PRO_CACHE_0_SPEC> {
        PRO_CACHE_LOCK_W::new(self, 0)
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
#[doc = "Cache permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cache_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cache_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CACHE_0_SPEC;
impl crate::RegisterSpec for PRO_CACHE_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cache_0::R`](R) reader structure"]
impl crate::Readable for PRO_CACHE_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cache_0::W`](W) writer structure"]
impl crate::Writable for PRO_CACHE_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_CACHE_0 to value 0"]
impl crate::Resettable for PRO_CACHE_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
