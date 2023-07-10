#[doc = "Register `LPCORE` reader"]
pub struct R(crate::R<LPCORE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPCORE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPCORE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPCORE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPCORE` writer"]
pub struct W(crate::W<LPCORE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPCORE_SPEC>;
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
impl From<crate::W<LPCORE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPCORE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ETM_WAKEUP_FLAG_CLR` writer - need_des"]
pub type ETM_WAKEUP_FLAG_CLR_W<'a, const O: u8> = crate::BitWriter<'a, LPCORE_SPEC, O>;
#[doc = "Field `ETM_WAKEUP_FLAG` reader - need_des"]
pub type ETM_WAKEUP_FLAG_R = crate::BitReader;
#[doc = "Field `ETM_WAKEUP_FLAG` writer - need_des"]
pub type ETM_WAKEUP_FLAG_W<'a, const O: u8> = crate::BitWriter<'a, LPCORE_SPEC, O>;
#[doc = "Field `DISABLE` reader - need_des"]
pub type DISABLE_R = crate::BitReader;
#[doc = "Field `DISABLE` writer - need_des"]
pub type DISABLE_W<'a, const O: u8> = crate::BitWriter<'a, LPCORE_SPEC, O>;
impl R {
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn etm_wakeup_flag(&self) -> ETM_WAKEUP_FLAG_R {
        ETM_WAKEUP_FLAG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn disable(&self) -> DISABLE_R {
        DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCORE")
            .field(
                "etm_wakeup_flag",
                &format_args!("{}", self.etm_wakeup_flag().bit()),
            )
            .field("disable", &format_args!("{}", self.disable().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LPCORE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn etm_wakeup_flag_clr(&mut self) -> ETM_WAKEUP_FLAG_CLR_W<0> {
        ETM_WAKEUP_FLAG_CLR_W::new(self)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn etm_wakeup_flag(&mut self) -> ETM_WAKEUP_FLAG_W<1> {
        ETM_WAKEUP_FLAG_W::new(self)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn disable(&mut self) -> DISABLE_W<31> {
        DISABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpcore](index.html) module"]
pub struct LPCORE_SPEC;
impl crate::RegisterSpec for LPCORE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpcore::R](R) reader structure"]
impl crate::Readable for LPCORE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpcore::W](W) writer structure"]
impl crate::Writable for LPCORE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPCORE to value 0"]
impl crate::Resettable for LPCORE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
