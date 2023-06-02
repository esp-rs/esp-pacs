#[doc = "Register `APB_PERIPHERAL_ACCESS_1` reader"]
pub struct R(crate::R<APB_PERIPHERAL_ACCESS_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_PERIPHERAL_ACCESS_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_PERIPHERAL_ACCESS_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_PERIPHERAL_ACCESS_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_PERIPHERAL_ACCESS_1` writer"]
pub struct W(crate::W<APB_PERIPHERAL_ACCESS_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_PERIPHERAL_ACCESS_1_SPEC>;
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
impl From<crate::W<APB_PERIPHERAL_ACCESS_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_PERIPHERAL_ACCESS_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_PERIPHERAL_ACCESS_SPLIT_BURST` reader - Set 1 to support split function for AHB access to APB peripherals."]
pub type APB_PERIPHERAL_ACCESS_SPLIT_BURST_R = crate::BitReader;
#[doc = "Field `APB_PERIPHERAL_ACCESS_SPLIT_BURST` writer - Set 1 to support split function for AHB access to APB peripherals."]
pub type APB_PERIPHERAL_ACCESS_SPLIT_BURST_W<'a, const O: u8> =
    crate::BitWriter<'a, APB_PERIPHERAL_ACCESS_1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to support split function for AHB access to APB peripherals."]
    #[inline(always)]
    pub fn apb_peripheral_access_split_burst(&self) -> APB_PERIPHERAL_ACCESS_SPLIT_BURST_R {
        APB_PERIPHERAL_ACCESS_SPLIT_BURST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_PERIPHERAL_ACCESS_1")
            .field(
                "apb_peripheral_access_split_burst",
                &format_args!("{}", self.apb_peripheral_access_split_burst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_PERIPHERAL_ACCESS_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to support split function for AHB access to APB peripherals."]
    #[inline(always)]
    #[must_use]
    pub fn apb_peripheral_access_split_burst(&mut self) -> APB_PERIPHERAL_ACCESS_SPLIT_BURST_W<0> {
        APB_PERIPHERAL_ACCESS_SPLIT_BURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB peripheral configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_peripheral_access_1](index.html) module"]
pub struct APB_PERIPHERAL_ACCESS_1_SPEC;
impl crate::RegisterSpec for APB_PERIPHERAL_ACCESS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_peripheral_access_1::R](R) reader structure"]
impl crate::Readable for APB_PERIPHERAL_ACCESS_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_peripheral_access_1::W](W) writer structure"]
impl crate::Writable for APB_PERIPHERAL_ACCESS_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_PERIPHERAL_ACCESS_1 to value 0x01"]
impl crate::Resettable for APB_PERIPHERAL_ACCESS_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
