#[doc = "Register `RETENTION_CFG` reader"]
pub struct R(crate::R<RETENTION_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RETENTION_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RETENTION_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RETENTION_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RETENTION_CFG` writer"]
pub struct W(crate::W<RETENTION_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RETENTION_CFG_SPEC>;
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
impl From<crate::W<RETENTION_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RETENTION_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RET_INV_CFG` reader - retention inv scan out"]
pub type RET_INV_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `RET_INV_CFG` writer - retention inv scan out"]
pub type RET_INV_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, RETENTION_CFG_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - retention inv scan out"]
    #[inline(always)]
    pub fn ret_inv_cfg(&self) -> RET_INV_CFG_R {
        RET_INV_CFG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CFG")
            .field(
                "ret_inv_cfg",
                &format_args!("{}", self.ret_inv_cfg().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - retention inv scan out"]
    #[inline(always)]
    #[must_use]
    pub fn ret_inv_cfg(&mut self) -> RET_INV_CFG_W<0> {
        RET_INV_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "retention_cfg\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [retention_cfg](index.html) module"]
pub struct RETENTION_CFG_SPEC;
impl crate::RegisterSpec for RETENTION_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [retention_cfg::R](R) reader structure"]
impl crate::Readable for RETENTION_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [retention_cfg::W](W) writer structure"]
impl crate::Writable for RETENTION_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_CFG to value 0xffff_ffff"]
impl crate::Resettable for RETENTION_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
