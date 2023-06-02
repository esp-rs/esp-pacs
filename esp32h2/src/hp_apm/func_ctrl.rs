#[doc = "Register `FUNC_CTRL` reader"]
pub struct R(crate::R<FUNC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC_CTRL` writer"]
pub struct W(crate::W<FUNC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_CTRL_SPEC>;
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
impl From<crate::W<FUNC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M0_PMS_FUNC_EN` reader - PMS M0 function enable"]
pub type M0_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M0_PMS_FUNC_EN` writer - PMS M0 function enable"]
pub type M0_PMS_FUNC_EN_W<'a, const O: u8> = crate::BitWriter<'a, FUNC_CTRL_SPEC, O>;
#[doc = "Field `M1_PMS_FUNC_EN` reader - PMS M1 function enable"]
pub type M1_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M1_PMS_FUNC_EN` writer - PMS M1 function enable"]
pub type M1_PMS_FUNC_EN_W<'a, const O: u8> = crate::BitWriter<'a, FUNC_CTRL_SPEC, O>;
#[doc = "Field `M2_PMS_FUNC_EN` reader - PMS M2 function enable"]
pub type M2_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M2_PMS_FUNC_EN` writer - PMS M2 function enable"]
pub type M2_PMS_FUNC_EN_W<'a, const O: u8> = crate::BitWriter<'a, FUNC_CTRL_SPEC, O>;
#[doc = "Field `M3_PMS_FUNC_EN` reader - PMS M3 function enable"]
pub type M3_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M3_PMS_FUNC_EN` writer - PMS M3 function enable"]
pub type M3_PMS_FUNC_EN_W<'a, const O: u8> = crate::BitWriter<'a, FUNC_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    pub fn m0_pms_func_en(&self) -> M0_PMS_FUNC_EN_R {
        M0_PMS_FUNC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    pub fn m1_pms_func_en(&self) -> M1_PMS_FUNC_EN_R {
        M1_PMS_FUNC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PMS M2 function enable"]
    #[inline(always)]
    pub fn m2_pms_func_en(&self) -> M2_PMS_FUNC_EN_R {
        M2_PMS_FUNC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMS M3 function enable"]
    #[inline(always)]
    pub fn m3_pms_func_en(&self) -> M3_PMS_FUNC_EN_R {
        M3_PMS_FUNC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_CTRL")
            .field(
                "m0_pms_func_en",
                &format_args!("{}", self.m0_pms_func_en().bit()),
            )
            .field(
                "m1_pms_func_en",
                &format_args!("{}", self.m1_pms_func_en().bit()),
            )
            .field(
                "m2_pms_func_en",
                &format_args!("{}", self.m2_pms_func_en().bit()),
            )
            .field(
                "m3_pms_func_en",
                &format_args!("{}", self.m3_pms_func_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FUNC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn m0_pms_func_en(&mut self) -> M0_PMS_FUNC_EN_W<0> {
        M0_PMS_FUNC_EN_W::new(self)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn m1_pms_func_en(&mut self) -> M1_PMS_FUNC_EN_W<1> {
        M1_PMS_FUNC_EN_W::new(self)
    }
    #[doc = "Bit 2 - PMS M2 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn m2_pms_func_en(&mut self) -> M2_PMS_FUNC_EN_W<2> {
        M2_PMS_FUNC_EN_W::new(self)
    }
    #[doc = "Bit 3 - PMS M3 function enable"]
    #[inline(always)]
    #[must_use]
    pub fn m3_pms_func_en(&mut self) -> M3_PMS_FUNC_EN_W<3> {
        M3_PMS_FUNC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMS function control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_ctrl](index.html) module"]
pub struct FUNC_CTRL_SPEC;
impl crate::RegisterSpec for FUNC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_ctrl::R](R) reader structure"]
impl crate::Readable for FUNC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_ctrl::W](W) writer structure"]
impl crate::Writable for FUNC_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FUNC_CTRL to value 0x0f"]
impl crate::Resettable for FUNC_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
