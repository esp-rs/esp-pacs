#[doc = "Register `APB_PERIPHERAL_ACCESS_0` reader"]
pub type R = crate::R<APB_PERIPHERAL_ACCESS_0_SPEC>;
#[doc = "Register `APB_PERIPHERAL_ACCESS_0` writer"]
pub type W = crate::W<APB_PERIPHERAL_ACCESS_0_SPEC>;
#[doc = "Field `APB_PERIPHERAL_ACCESS_LOCK` reader - apb_peripheral_access_lock"]
pub type APB_PERIPHERAL_ACCESS_LOCK_R = crate::BitReader;
#[doc = "Field `APB_PERIPHERAL_ACCESS_LOCK` writer - apb_peripheral_access_lock"]
pub type APB_PERIPHERAL_ACCESS_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - apb_peripheral_access_lock"]
    #[inline(always)]
    pub fn apb_peripheral_access_lock(&self) -> APB_PERIPHERAL_ACCESS_LOCK_R {
        APB_PERIPHERAL_ACCESS_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_ACCESS_0")
            .field(
                "apb_peripheral_access_lock",
                &format_args!("{}", self.apb_peripheral_access_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_PERIPHERAL_ACCESS_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - apb_peripheral_access_lock"]
    #[inline(always)]
    #[must_use]
    pub fn apb_peripheral_access_lock(
        &mut self,
    ) -> APB_PERIPHERAL_ACCESS_LOCK_W<APB_PERIPHERAL_ACCESS_0_SPEC, 0> {
        APB_PERIPHERAL_ACCESS_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SENSITIVE_APB_PERIPHERAL_ACCESS_0_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_peripheral_access_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_peripheral_access_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_PERIPHERAL_ACCESS_0_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_ACCESS_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_peripheral_access_0::R`](R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_ACCESS_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_peripheral_access_0::W`](W) writer structure"]
impl crate::Writable for APB_PERIPHERAL_ACCESS_0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_PERIPHERAL_ACCESS_0 to value 0"]
impl crate::Resettable for APB_PERIPHERAL_ACCESS_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
