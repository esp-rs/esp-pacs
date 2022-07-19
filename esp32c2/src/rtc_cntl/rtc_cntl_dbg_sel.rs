#[doc = "Register `RTC_CNTL_DBG_SEL` reader"]
pub struct R(crate::R<RTC_CNTL_DBG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_CNTL_DBG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_CNTL_DBG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_CNTL_DBG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_CNTL_DBG_SEL` writer"]
pub struct W(crate::W<RTC_CNTL_DBG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_CNTL_DBG_SEL_SPEC>;
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
impl From<crate::W<RTC_CNTL_DBG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_CNTL_DBG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` reader - Need add desc"]
pub type RTC_DEBUG_12M_NO_GATING_R = crate::BitReader<bool>;
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` writer - Need add desc"]
pub type RTC_DEBUG_12M_NO_GATING_W<'a> = crate::BitWriter<'a, u32, RTC_CNTL_DBG_SEL_SPEC, bool, 1>;
#[doc = "Field `RTC_DEBUG_BIT_SEL` reader - Need add desc"]
pub type RTC_DEBUG_BIT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_BIT_SEL` writer - Need add desc"]
pub type RTC_DEBUG_BIT_SEL_W<'a> = crate::FieldWriter<'a, u32, RTC_CNTL_DBG_SEL_SPEC, u8, u8, 5, 2>;
#[doc = "Field `RTC_DEBUG_SEL0` reader - Need add desc"]
pub type RTC_DEBUG_SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL0` writer - Need add desc"]
pub type RTC_DEBUG_SEL0_W<'a> = crate::FieldWriter<'a, u32, RTC_CNTL_DBG_SEL_SPEC, u8, u8, 5, 7>;
#[doc = "Field `RTC_DEBUG_SEL1` reader - Need add desc"]
pub type RTC_DEBUG_SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL1` writer - Need add desc"]
pub type RTC_DEBUG_SEL1_W<'a> = crate::FieldWriter<'a, u32, RTC_CNTL_DBG_SEL_SPEC, u8, u8, 5, 12>;
#[doc = "Field `RTC_DEBUG_SEL2` reader - Need add desc"]
pub type RTC_DEBUG_SEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL2` writer - Need add desc"]
pub type RTC_DEBUG_SEL2_W<'a> = crate::FieldWriter<'a, u32, RTC_CNTL_DBG_SEL_SPEC, u8, u8, 5, 17>;
#[doc = "Field `RTC_DEBUG_SEL3` reader - Need add desc"]
pub type RTC_DEBUG_SEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL3` writer - Need add desc"]
pub type RTC_DEBUG_SEL3_W<'a> = crate::FieldWriter<'a, u32, RTC_CNTL_DBG_SEL_SPEC, u8, u8, 5, 22>;
#[doc = "Field `RTC_DEBUG_SEL4` reader - Need add desc"]
pub type RTC_DEBUG_SEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL4` writer - Need add desc"]
pub type RTC_DEBUG_SEL4_W<'a> = crate::FieldWriter<'a, u32, RTC_CNTL_DBG_SEL_SPEC, u8, u8, 5, 27>;
impl R {
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&self) -> RTC_DEBUG_12M_NO_GATING_R {
        RTC_DEBUG_12M_NO_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_bit_sel(&self) -> RTC_DEBUG_BIT_SEL_R {
        RTC_DEBUG_BIT_SEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&self) -> RTC_DEBUG_SEL0_R {
        RTC_DEBUG_SEL0_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&self) -> RTC_DEBUG_SEL1_R {
        RTC_DEBUG_SEL1_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&self) -> RTC_DEBUG_SEL2_R {
        RTC_DEBUG_SEL2_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&self) -> RTC_DEBUG_SEL3_R {
        RTC_DEBUG_SEL3_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&self) -> RTC_DEBUG_SEL4_R {
        RTC_DEBUG_SEL4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&mut self) -> RTC_DEBUG_12M_NO_GATING_W {
        RTC_DEBUG_12M_NO_GATING_W::new(self)
    }
    #[doc = "Bits 2:6 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_bit_sel(&mut self) -> RTC_DEBUG_BIT_SEL_W {
        RTC_DEBUG_BIT_SEL_W::new(self)
    }
    #[doc = "Bits 7:11 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&mut self) -> RTC_DEBUG_SEL0_W {
        RTC_DEBUG_SEL0_W::new(self)
    }
    #[doc = "Bits 12:16 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&mut self) -> RTC_DEBUG_SEL1_W {
        RTC_DEBUG_SEL1_W::new(self)
    }
    #[doc = "Bits 17:21 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&mut self) -> RTC_DEBUG_SEL2_W {
        RTC_DEBUG_SEL2_W::new(self)
    }
    #[doc = "Bits 22:26 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&mut self) -> RTC_DEBUG_SEL3_W {
        RTC_DEBUG_SEL3_W::new(self)
    }
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&mut self) -> RTC_DEBUG_SEL4_W {
        RTC_DEBUG_SEL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_cntl_dbg_sel](index.html) module"]
pub struct RTC_CNTL_DBG_SEL_SPEC;
impl crate::RegisterSpec for RTC_CNTL_DBG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_cntl_dbg_sel::R](R) reader structure"]
impl crate::Readable for RTC_CNTL_DBG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_cntl_dbg_sel::W](W) writer structure"]
impl crate::Writable for RTC_CNTL_DBG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_CNTL_DBG_SEL to value 0"]
impl crate::Resettable for RTC_CNTL_DBG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
