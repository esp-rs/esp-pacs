#[doc = "Register `PIN6` reader"]
pub struct R(crate::R<PIN6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIN6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIN6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIN6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIN6` writer"]
pub struct W(crate::W<PIN6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIN6_SPEC>;
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
impl From<crate::W<PIN6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIN6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_GPIO6_SYNC_BYPASS` reader - need des"]
pub type LP_GPIO6_SYNC_BYPASS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_GPIO6_SYNC_BYPASS` writer - need des"]
pub type LP_GPIO6_SYNC_BYPASS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PIN6_SPEC, u8, u8, 2, O>;
#[doc = "Field `LP_GPIO6_PAD_DRIVER` reader - need des"]
pub type LP_GPIO6_PAD_DRIVER_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO6_PAD_DRIVER` writer - need des"]
pub type LP_GPIO6_PAD_DRIVER_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN6_SPEC, bool, O>;
#[doc = "Field `LP_GPIO6_EDGE_WAKEUP_CLR` writer - need des"]
pub type LP_GPIO6_EDGE_WAKEUP_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, PIN6_SPEC, bool, O>;
#[doc = "Field `LP_GPIO6_INT_TYPE` reader - need des"]
pub type LP_GPIO6_INT_TYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_GPIO6_INT_TYPE` writer - need des"]
pub type LP_GPIO6_INT_TYPE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PIN6_SPEC, u8, u8, 3, O>;
#[doc = "Field `LP_GPIO6_WAKEUP_ENABLE` reader - need des"]
pub type LP_GPIO6_WAKEUP_ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO6_WAKEUP_ENABLE` writer - need des"]
pub type LP_GPIO6_WAKEUP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN6_SPEC, bool, O>;
#[doc = "Field `LP_GPIO6_FILTER_EN` reader - need des"]
pub type LP_GPIO6_FILTER_EN_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO6_FILTER_EN` writer - need des"]
pub type LP_GPIO6_FILTER_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PIN6_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - need des"]
    #[inline(always)]
    pub fn lp_gpio6_sync_bypass(&self) -> LP_GPIO6_SYNC_BYPASS_R {
        LP_GPIO6_SYNC_BYPASS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    pub fn lp_gpio6_pad_driver(&self) -> LP_GPIO6_PAD_DRIVER_R {
        LP_GPIO6_PAD_DRIVER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:9 - need des"]
    #[inline(always)]
    pub fn lp_gpio6_int_type(&self) -> LP_GPIO6_INT_TYPE_R {
        LP_GPIO6_INT_TYPE_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - need des"]
    #[inline(always)]
    pub fn lp_gpio6_wakeup_enable(&self) -> LP_GPIO6_WAKEUP_ENABLE_R {
        LP_GPIO6_WAKEUP_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn lp_gpio6_filter_en(&self) -> LP_GPIO6_FILTER_EN_R {
        LP_GPIO6_FILTER_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio6_sync_bypass(&mut self) -> LP_GPIO6_SYNC_BYPASS_W<0> {
        LP_GPIO6_SYNC_BYPASS_W::new(self)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio6_pad_driver(&mut self) -> LP_GPIO6_PAD_DRIVER_W<2> {
        LP_GPIO6_PAD_DRIVER_W::new(self)
    }
    #[doc = "Bit 3 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio6_edge_wakeup_clr(&mut self) -> LP_GPIO6_EDGE_WAKEUP_CLR_W<3> {
        LP_GPIO6_EDGE_WAKEUP_CLR_W::new(self)
    }
    #[doc = "Bits 7:9 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio6_int_type(&mut self) -> LP_GPIO6_INT_TYPE_W<7> {
        LP_GPIO6_INT_TYPE_W::new(self)
    }
    #[doc = "Bit 10 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio6_wakeup_enable(&mut self) -> LP_GPIO6_WAKEUP_ENABLE_W<10> {
        LP_GPIO6_WAKEUP_ENABLE_W::new(self)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio6_filter_en(&mut self) -> LP_GPIO6_FILTER_EN_W<11> {
        LP_GPIO6_FILTER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin6](index.html) module"]
pub struct PIN6_SPEC;
impl crate::RegisterSpec for PIN6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pin6::R](R) reader structure"]
impl crate::Readable for PIN6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pin6::W](W) writer structure"]
impl crate::Writable for PIN6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIN6 to value 0"]
impl crate::Resettable for PIN6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}