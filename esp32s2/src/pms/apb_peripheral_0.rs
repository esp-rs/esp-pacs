#[doc = "Register `APB_PERIPHERAL_0` reader"]
pub struct R(crate::R<APB_PERIPHERAL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_PERIPHERAL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_PERIPHERAL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_PERIPHERAL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_PERIPHERAL_0` writer"]
pub struct W(crate::W<APB_PERIPHERAL_0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_PERIPHERAL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<APB_PERIPHERAL_0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_PERIPHERAL_0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_PERIPHERAL_LOCK` reader - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
pub type APB_PERIPHERAL_LOCK_R = crate::BitReader;
#[doc = "Field `APB_PERIPHERAL_LOCK` writer - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
pub type APB_PERIPHERAL_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, APB_PERIPHERAL_0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
    #[inline(always)]
    pub fn apb_peripheral_lock(&self) -> APB_PERIPHERAL_LOCK_R {
        APB_PERIPHERAL_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_0")
            .field(
                "apb_peripheral_lock",
                &format_args!("{}", self.apb_peripheral_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_PERIPHERAL_0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Lock register. Setting to 1 locks TX Copy DMA permission control registers."]
    #[inline(always)]
    #[must_use]
    pub fn apb_peripheral_lock(&mut self) -> APB_PERIPHERAL_LOCK_W<0> {
        APB_PERIPHERAL_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral access permission control register 0.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_peripheral_0](index.html) module"]
pub struct APB_PERIPHERAL_0_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_peripheral_0::R](R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_peripheral_0::W](W) writer structure"]
impl crate::Writable for APB_PERIPHERAL_0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_PERIPHERAL_0 to value 0"]
impl crate::Resettable for APB_PERIPHERAL_0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
