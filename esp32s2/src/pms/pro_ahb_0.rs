#[doc = "Register `PRO_AHB_0` reader"]
pub type R = crate::R<PRO_AHB_0_SPEC>;
#[doc = "Register `PRO_AHB_0` writer"]
pub type W = crate::W<PRO_AHB_0_SPEC>;
#[doc = "Field `PRO_AHB_LOCK` reader - Lock register. Setting to 1 locks PeriBus2 permission control registers."]
pub type PRO_AHB_LOCK_R = crate::BitReader;
#[doc = "Field `PRO_AHB_LOCK` writer - Lock register. Setting to 1 locks PeriBus2 permission control registers."]
pub type PRO_AHB_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks PeriBus2 permission control registers."]
    #[inline(always)]
    pub fn pro_ahb_lock(&self) -> PRO_AHB_LOCK_R {
        PRO_AHB_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_AHB_0")
            .field(
                "pro_ahb_lock",
                &format_args!("{}", self.pro_ahb_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_AHB_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks PeriBus2 permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn pro_ahb_lock(&mut self) -> PRO_AHB_LOCK_W<PRO_AHB_0_SPEC> {
        PRO_AHB_LOCK_W::new(self, 0)
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
#[doc = "PeriBus2 permission control register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_ahb_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_ahb_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_AHB_0_SPEC;
impl crate::RegisterSpec for PRO_AHB_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_ahb_0::R`](R) reader structure"]
impl crate::Readable for PRO_AHB_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_ahb_0::W`](W) writer structure"]
impl crate::Writable for PRO_AHB_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_AHB_0 to value 0"]
impl crate::Resettable for PRO_AHB_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
