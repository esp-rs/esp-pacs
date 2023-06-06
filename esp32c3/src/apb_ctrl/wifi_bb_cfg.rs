#[doc = "Register `WIFI_BB_CFG` reader"]
pub struct R(crate::R<WIFI_BB_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_BB_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_BB_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_BB_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFI_BB_CFG` writer"]
pub struct W(crate::W<WIFI_BB_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_BB_CFG_SPEC>;
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
impl From<crate::W<WIFI_BB_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_BB_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_BB_CFG` reader - reg_wifi_bb_cfg"]
pub type WIFI_BB_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `WIFI_BB_CFG` writer - reg_wifi_bb_cfg"]
pub type WIFI_BB_CFG_W<'a, const O: u8> = crate::FieldWriter<'a, WIFI_BB_CFG_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - reg_wifi_bb_cfg"]
    #[inline(always)]
    pub fn wifi_bb_cfg(&self) -> WIFI_BB_CFG_R {
        WIFI_BB_CFG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_BB_CFG")
            .field(
                "wifi_bb_cfg",
                &format_args!("{}", self.wifi_bb_cfg().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WIFI_BB_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reg_wifi_bb_cfg"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_bb_cfg(&mut self) -> WIFI_BB_CFG_W<0> {
        WIFI_BB_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APB_CTRL_WIFI_BB_CFG_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_bb_cfg](index.html) module"]
pub struct WIFI_BB_CFG_SPEC;
impl crate::RegisterSpec for WIFI_BB_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_bb_cfg::R](R) reader structure"]
impl crate::Readable for WIFI_BB_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_bb_cfg::W](W) writer structure"]
impl crate::Writable for WIFI_BB_CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WIFI_BB_CFG to value 0"]
impl crate::Resettable for WIFI_BB_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
