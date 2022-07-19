#[doc = "Register `RTC_TIMER1` reader"]
pub struct R(crate::R<RTC_TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTC_TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTC_TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTC_TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTC_TIMER1` writer"]
pub struct W(crate::W<RTC_TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTC_TIMER1_SPEC>;
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
impl From<crate::W<RTC_TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTC_TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_STALL_EN` reader - CPU stall enable bit"]
pub type CPU_STALL_EN_R = crate::BitReader<bool>;
#[doc = "Field `CPU_STALL_EN` writer - CPU stall enable bit"]
pub type CPU_STALL_EN_W<'a> = crate::BitWriter<'a, u32, RTC_TIMER1_SPEC, bool, 0>;
#[doc = "Field `CPU_STALL_WAIT` reader - CPU stall wait cycles in fast_clk_rtc"]
pub type CPU_STALL_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPU_STALL_WAIT` writer - CPU stall wait cycles in fast_clk_rtc"]
pub type CPU_STALL_WAIT_W<'a> = crate::FieldWriter<'a, u32, RTC_TIMER1_SPEC, u8, u8, 5, 1>;
#[doc = "Field `CK8M_WAIT` reader - CK8M wait cycles in slow_clk_rtc"]
pub type CK8M_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CK8M_WAIT` writer - CK8M wait cycles in slow_clk_rtc"]
pub type CK8M_WAIT_W<'a> = crate::FieldWriter<'a, u32, RTC_TIMER1_SPEC, u8, u8, 8, 6>;
#[doc = "Field `XTL_BUF_WAIT` reader - XTAL wait cycles in slow_clk_rtc"]
pub type XTL_BUF_WAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `XTL_BUF_WAIT` writer - XTAL wait cycles in slow_clk_rtc"]
pub type XTL_BUF_WAIT_W<'a> = crate::FieldWriter<'a, u32, RTC_TIMER1_SPEC, u16, u16, 10, 14>;
#[doc = "Field `PLL_BUF_WAIT` reader - PLL wait cycles in slow_clk_rtc"]
pub type PLL_BUF_WAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLL_BUF_WAIT` writer - PLL wait cycles in slow_clk_rtc"]
pub type PLL_BUF_WAIT_W<'a> = crate::FieldWriter<'a, u32, RTC_TIMER1_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    pub fn cpu_stall_en(&self) -> CPU_STALL_EN_R {
        CPU_STALL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    pub fn cpu_stall_wait(&self) -> CPU_STALL_WAIT_R {
        CPU_STALL_WAIT_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn ck8m_wait(&self) -> CK8M_WAIT_R {
        CK8M_WAIT_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn xtl_buf_wait(&self) -> XTL_BUF_WAIT_R {
        XTL_BUF_WAIT_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:31 - PLL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn pll_buf_wait(&self) -> PLL_BUF_WAIT_R {
        PLL_BUF_WAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    pub fn cpu_stall_en(&mut self) -> CPU_STALL_EN_W {
        CPU_STALL_EN_W::new(self)
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    pub fn cpu_stall_wait(&mut self) -> CPU_STALL_WAIT_W {
        CPU_STALL_WAIT_W::new(self)
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn ck8m_wait(&mut self) -> CK8M_WAIT_W {
        CK8M_WAIT_W::new(self)
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn xtl_buf_wait(&mut self) -> XTL_BUF_WAIT_W {
        XTL_BUF_WAIT_W::new(self)
    }
    #[doc = "Bits 24:31 - PLL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    pub fn pll_buf_wait(&mut self) -> PLL_BUF_WAIT_W {
        PLL_BUF_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtc_timer1](index.html) module"]
pub struct RTC_TIMER1_SPEC;
impl crate::RegisterSpec for RTC_TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtc_timer1::R](R) reader structure"]
impl crate::Readable for RTC_TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtc_timer1::W](W) writer structure"]
impl crate::Writable for RTC_TIMER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTC_TIMER1 to value 0x2814_0403"]
impl crate::Resettable for RTC_TIMER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2814_0403
    }
}
