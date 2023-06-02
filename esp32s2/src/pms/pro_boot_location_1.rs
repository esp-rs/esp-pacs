#[doc = "Register `PRO_BOOT_LOCATION_1` reader"]
pub struct R(crate::R<PRO_BOOT_LOCATION_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_BOOT_LOCATION_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_BOOT_LOCATION_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_BOOT_LOCATION_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_BOOT_LOCATION_1` writer"]
pub struct W(crate::W<PRO_BOOT_LOCATION_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_BOOT_LOCATION_1_SPEC>;
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
impl From<crate::W<PRO_BOOT_LOCATION_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_BOOT_LOCATION_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_BOOT_REMAP` reader - If set to 1, enable boot remap function."]
pub type PRO_BOOT_REMAP_R = crate::BitReader;
#[doc = "Field `PRO_BOOT_REMAP` writer - If set to 1, enable boot remap function."]
pub type PRO_BOOT_REMAP_W<'a, const O: u8> = crate::BitWriter<'a, PRO_BOOT_LOCATION_1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - If set to 1, enable boot remap function."]
    #[inline(always)]
    pub fn pro_boot_remap(&self) -> PRO_BOOT_REMAP_R {
        PRO_BOOT_REMAP_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_BOOT_LOCATION_1")
            .field(
                "pro_boot_remap",
                &format_args!("{}", self.pro_boot_remap().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_BOOT_LOCATION_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - If set to 1, enable boot remap function."]
    #[inline(always)]
    #[must_use]
    pub fn pro_boot_remap(&mut self) -> PRO_BOOT_REMAP_W<0> {
        PRO_BOOT_REMAP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot permission control register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_boot_location_1](index.html) module"]
pub struct PRO_BOOT_LOCATION_1_SPEC;
impl crate::RegisterSpec for PRO_BOOT_LOCATION_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_boot_location_1::R](R) reader structure"]
impl crate::Readable for PRO_BOOT_LOCATION_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_boot_location_1::W](W) writer structure"]
impl crate::Writable for PRO_BOOT_LOCATION_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_BOOT_LOCATION_1 to value 0"]
impl crate::Resettable for PRO_BOOT_LOCATION_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
