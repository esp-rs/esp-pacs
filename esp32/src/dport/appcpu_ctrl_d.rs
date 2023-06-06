#[doc = "Register `APPCPU_CTRL_D` reader"]
pub struct R(crate::R<APPCPU_CTRL_D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APPCPU_CTRL_D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APPCPU_CTRL_D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APPCPU_CTRL_D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APPCPU_CTRL_D` writer"]
pub struct W(crate::W<APPCPU_CTRL_D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APPCPU_CTRL_D_SPEC>;
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
impl From<crate::W<APPCPU_CTRL_D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APPCPU_CTRL_D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPCPU_BOOT_ADDR` reader - "]
pub type APPCPU_BOOT_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `APPCPU_BOOT_ADDR` writer - "]
pub type APPCPU_BOOT_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, APPCPU_CTRL_D_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn appcpu_boot_addr(&self) -> APPCPU_BOOT_ADDR_R {
        APPCPU_BOOT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPCPU_CTRL_D")
            .field(
                "appcpu_boot_addr",
                &format_args!("{}", self.appcpu_boot_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APPCPU_CTRL_D_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn appcpu_boot_addr(&mut self) -> APPCPU_BOOT_ADDR_W<0> {
        APPCPU_BOOT_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [appcpu_ctrl_d](index.html) module"]
pub struct APPCPU_CTRL_D_SPEC;
impl crate::RegisterSpec for APPCPU_CTRL_D_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [appcpu_ctrl_d::R](R) reader structure"]
impl crate::Readable for APPCPU_CTRL_D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [appcpu_ctrl_d::W](W) writer structure"]
impl crate::Writable for APPCPU_CTRL_D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APPCPU_CTRL_D to value 0"]
impl crate::Resettable for APPCPU_CTRL_D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
