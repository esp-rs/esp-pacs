#[doc = "Register `THRES_CTRL` reader"]
pub struct R(crate::R<THRES_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<THRES_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<THRES_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<THRES_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `THRES_CTRL` writer"]
pub struct W(crate::W<THRES_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<THRES_CTRL_SPEC>;
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
impl From<crate::W<THRES_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<THRES_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APB_SARADC_THRES_ALL_EN` reader - enable thres to all channel"]
pub type APB_SARADC_THRES_ALL_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES_ALL_EN` writer - enable thres to all channel"]
pub type APB_SARADC_THRES_ALL_EN_W<'a, const O: u8> = crate::BitWriter<'a, THRES_CTRL_SPEC, O>;
#[doc = "Field `APB_SARADC_THRES1_EN` reader - enable thres1"]
pub type APB_SARADC_THRES1_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES1_EN` writer - enable thres1"]
pub type APB_SARADC_THRES1_EN_W<'a, const O: u8> = crate::BitWriter<'a, THRES_CTRL_SPEC, O>;
#[doc = "Field `APB_SARADC_THRES0_EN` reader - enable thres0"]
pub type APB_SARADC_THRES0_EN_R = crate::BitReader;
#[doc = "Field `APB_SARADC_THRES0_EN` writer - enable thres0"]
pub type APB_SARADC_THRES0_EN_W<'a, const O: u8> = crate::BitWriter<'a, THRES_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 27 - enable thres to all channel"]
    #[inline(always)]
    pub fn apb_saradc_thres_all_en(&self) -> APB_SARADC_THRES_ALL_EN_R {
        APB_SARADC_THRES_ALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - enable thres1"]
    #[inline(always)]
    pub fn apb_saradc_thres1_en(&self) -> APB_SARADC_THRES1_EN_R {
        APB_SARADC_THRES1_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - enable thres0"]
    #[inline(always)]
    pub fn apb_saradc_thres0_en(&self) -> APB_SARADC_THRES0_EN_R {
        APB_SARADC_THRES0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES_CTRL")
            .field(
                "apb_saradc_thres_all_en",
                &format_args!("{}", self.apb_saradc_thres_all_en().bit()),
            )
            .field(
                "apb_saradc_thres1_en",
                &format_args!("{}", self.apb_saradc_thres1_en().bit()),
            )
            .field(
                "apb_saradc_thres0_en",
                &format_args!("{}", self.apb_saradc_thres0_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<THRES_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 27 - enable thres to all channel"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres_all_en(&mut self) -> APB_SARADC_THRES_ALL_EN_W<27> {
        APB_SARADC_THRES_ALL_EN_W::new(self)
    }
    #[doc = "Bit 30 - enable thres1"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres1_en(&mut self) -> APB_SARADC_THRES1_EN_W<30> {
        APB_SARADC_THRES1_EN_W::new(self)
    }
    #[doc = "Bit 31 - enable thres0"]
    #[inline(always)]
    #[must_use]
    pub fn apb_saradc_thres0_en(&mut self) -> APB_SARADC_THRES0_EN_W<31> {
        APB_SARADC_THRES0_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "digital saradc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [thres_ctrl](index.html) module"]
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [thres_ctrl::R](R) reader structure"]
impl crate::Readable for THRES_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [thres_ctrl::W](W) writer structure"]
impl crate::Writable for THRES_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets THRES_CTRL to value 0"]
impl crate::Resettable for THRES_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
