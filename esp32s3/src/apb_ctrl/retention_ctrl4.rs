#[doc = "Register `RETENTION_CTRL4` reader"]
pub struct R(crate::R<RETENTION_CTRL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CTRL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CTRL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CTRL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CTRL4` writer"]
pub struct W(crate::W<RETENTION_CTRL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CTRL4_SPEC>;
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
impl From<crate::W<RETENTION_CTRL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CTRL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RETENTION_INV_CFG` reader - ******* Description ***********"]
pub type RETENTION_INV_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `RETENTION_INV_CFG` writer - ******* Description ***********"]
pub type RETENTION_INV_CFG_W<'a, const O: u8> =
    crate::FieldWriter<'a, RETENTION_CTRL4_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_inv_cfg(&self) -> RETENTION_INV_CFG_R {
        RETENTION_INV_CFG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL4")
            .field(
                "retention_inv_cfg",
                &format_args!("{}", self.retention_inv_cfg().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CTRL4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn retention_inv_cfg(&mut self) -> RETENTION_INV_CFG_W<0> {
        RETENTION_INV_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_ctrl4](index.html) module"]
pub struct RETENTION_CTRL4_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_ctrl4::R](R) reader structure"]
impl crate::Readable for RETENTION_CTRL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_ctrl4::W](W) writer structure"]
impl crate::Writable for RETENTION_CTRL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL4 to value 0xffff_ffff"]
impl crate::Resettable for RETENTION_CTRL4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
