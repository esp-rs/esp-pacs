#[doc = "Register `DBG_SEL` reader"]
pub struct R(crate::R<DBG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBG_SEL` writer"]
pub struct W(crate::W<DBG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBG_SEL_SPEC>;
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
impl From<crate::W<DBG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` reader - use for debug"]
pub type RTC_DEBUG_12M_NO_GATING_R = crate::BitReader<bool>;
#[doc = "Field `RTC_DEBUG_12M_NO_GATING` writer - use for debug"]
pub type RTC_DEBUG_12M_NO_GATING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DBG_SEL_SPEC, bool, O>;
#[doc = "Field `RTC_DEBUG_BIT_SEL` reader - use for debug"]
pub type RTC_DEBUG_BIT_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_BIT_SEL` writer - use for debug"]
pub type RTC_DEBUG_BIT_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL0` reader - use for debug"]
pub type RTC_DEBUG_SEL0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL0` writer - use for debug"]
pub type RTC_DEBUG_SEL0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL1` reader - use for debug"]
pub type RTC_DEBUG_SEL1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL1` writer - use for debug"]
pub type RTC_DEBUG_SEL1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL2` reader - use for debug"]
pub type RTC_DEBUG_SEL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL2` writer - use for debug"]
pub type RTC_DEBUG_SEL2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL3` reader - use for debug"]
pub type RTC_DEBUG_SEL3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL3` writer - use for debug"]
pub type RTC_DEBUG_SEL3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_SEL_SPEC, u8, u8, 5, O>;
#[doc = "Field `RTC_DEBUG_SEL4` reader - use for debug"]
pub type RTC_DEBUG_SEL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_DEBUG_SEL4` writer - use for debug"]
pub type RTC_DEBUG_SEL4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DBG_SEL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 1 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&self) -> RTC_DEBUG_12M_NO_GATING_R {
        RTC_DEBUG_12M_NO_GATING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_bit_sel(&self) -> RTC_DEBUG_BIT_SEL_R {
        RTC_DEBUG_BIT_SEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&self) -> RTC_DEBUG_SEL0_R {
        RTC_DEBUG_SEL0_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&self) -> RTC_DEBUG_SEL1_R {
        RTC_DEBUG_SEL1_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&self) -> RTC_DEBUG_SEL2_R {
        RTC_DEBUG_SEL2_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&self) -> RTC_DEBUG_SEL3_R {
        RTC_DEBUG_SEL3_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&self) -> RTC_DEBUG_SEL4_R {
        RTC_DEBUG_SEL4_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_12m_no_gating(&mut self) -> RTC_DEBUG_12M_NO_GATING_W<1> {
        RTC_DEBUG_12M_NO_GATING_W::new(self)
    }
    #[doc = "Bits 2:6 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_bit_sel(&mut self) -> RTC_DEBUG_BIT_SEL_W<2> {
        RTC_DEBUG_BIT_SEL_W::new(self)
    }
    #[doc = "Bits 7:11 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel0(&mut self) -> RTC_DEBUG_SEL0_W<7> {
        RTC_DEBUG_SEL0_W::new(self)
    }
    #[doc = "Bits 12:16 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel1(&mut self) -> RTC_DEBUG_SEL1_W<12> {
        RTC_DEBUG_SEL1_W::new(self)
    }
    #[doc = "Bits 17:21 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel2(&mut self) -> RTC_DEBUG_SEL2_W<17> {
        RTC_DEBUG_SEL2_W::new(self)
    }
    #[doc = "Bits 22:26 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel3(&mut self) -> RTC_DEBUG_SEL3_W<22> {
        RTC_DEBUG_SEL3_W::new(self)
    }
    #[doc = "Bits 27:31 - use for debug"]
    #[inline(always)]
    pub fn rtc_debug_sel4(&mut self) -> RTC_DEBUG_SEL4_W<27> {
        RTC_DEBUG_SEL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbg_sel](index.html) module"]
pub struct DBG_SEL_SPEC;
impl crate::RegisterSpec for DBG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbg_sel::R](R) reader structure"]
impl crate::Readable for DBG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbg_sel::W](W) writer structure"]
impl crate::Writable for DBG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBG_SEL to value 0"]
impl crate::Resettable for DBG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
