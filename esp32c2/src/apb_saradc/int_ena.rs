#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `THRES1_LOW_INT_ENA` reader - Need add description"]
pub type THRES1_LOW_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `THRES1_LOW_INT_ENA` writer - Need add description"]
pub type THRES1_LOW_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 26>;
#[doc = "Field `THRES0_LOW_INT_ENA` reader - Need add description"]
pub type THRES0_LOW_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `THRES0_LOW_INT_ENA` writer - Need add description"]
pub type THRES0_LOW_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 27>;
#[doc = "Field `THRES1_HIGH_INT_ENA` reader - Need add description"]
pub type THRES1_HIGH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `THRES1_HIGH_INT_ENA` writer - Need add description"]
pub type THRES1_HIGH_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 28>;
#[doc = "Field `THRES0_HIGH_INT_ENA` reader - Need add description"]
pub type THRES0_HIGH_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `THRES0_HIGH_INT_ENA` writer - Need add description"]
pub type THRES0_HIGH_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 29>;
#[doc = "Field `APB_SARADC2_DONE_INT_ENA` reader - Need add description"]
pub type APB_SARADC2_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `APB_SARADC2_DONE_INT_ENA` writer - Need add description"]
pub type APB_SARADC2_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 30>;
#[doc = "Field `APB_SARADC1_DONE_INT_ENA` reader - Need add description"]
pub type APB_SARADC1_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `APB_SARADC1_DONE_INT_ENA` writer - Need add description"]
pub type APB_SARADC1_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 26 - Need add description"]
    #[inline(always)]
    pub fn thres1_low_int_ena(&self) -> THRES1_LOW_INT_ENA_R {
        THRES1_LOW_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Need add description"]
    #[inline(always)]
    pub fn thres0_low_int_ena(&self) -> THRES0_LOW_INT_ENA_R {
        THRES0_LOW_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Need add description"]
    #[inline(always)]
    pub fn thres1_high_int_ena(&self) -> THRES1_HIGH_INT_ENA_R {
        THRES1_HIGH_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    pub fn thres0_high_int_ena(&self) -> THRES0_HIGH_INT_ENA_R {
        THRES0_HIGH_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_ena(&self) -> APB_SARADC2_DONE_INT_ENA_R {
        APB_SARADC2_DONE_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_ena(&self) -> APB_SARADC1_DONE_INT_ENA_R {
        APB_SARADC1_DONE_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 26 - Need add description"]
    #[inline(always)]
    pub fn thres1_low_int_ena(&mut self) -> THRES1_LOW_INT_ENA_W {
        THRES1_LOW_INT_ENA_W::new(self)
    }
    #[doc = "Bit 27 - Need add description"]
    #[inline(always)]
    pub fn thres0_low_int_ena(&mut self) -> THRES0_LOW_INT_ENA_W {
        THRES0_LOW_INT_ENA_W::new(self)
    }
    #[doc = "Bit 28 - Need add description"]
    #[inline(always)]
    pub fn thres1_high_int_ena(&mut self) -> THRES1_HIGH_INT_ENA_W {
        THRES1_HIGH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 29 - Need add description"]
    #[inline(always)]
    pub fn thres0_high_int_ena(&mut self) -> THRES0_HIGH_INT_ENA_W {
        THRES0_HIGH_INT_ENA_W::new(self)
    }
    #[doc = "Bit 30 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc2_done_int_ena(&mut self) -> APB_SARADC2_DONE_INT_ENA_W {
        APB_SARADC2_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 31 - Need add description"]
    #[inline(always)]
    pub fn apb_saradc1_done_int_ena(&mut self) -> APB_SARADC1_DONE_INT_ENA_W {
        APB_SARADC1_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
