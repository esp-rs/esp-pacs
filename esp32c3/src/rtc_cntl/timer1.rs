#[doc = "Register `TIMER1` reader"]
pub struct R(crate::R<TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1` writer"]
pub struct W(crate::W<TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_SPEC>;
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
impl From<crate::W<TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPU_STALL_EN` reader - CPU stall enable bit"]
pub type CPU_STALL_EN_R = crate::BitReader;
#[doc = "Field `CPU_STALL_EN` writer - CPU stall enable bit"]
pub type CPU_STALL_EN_W<'a, const O: u8> = crate::BitWriter<'a, TIMER1_SPEC, O>;
#[doc = "Field `CPU_STALL_WAIT` reader - CPU stall wait cycles in fast_clk_rtc"]
pub type CPU_STALL_WAIT_R = crate::FieldReader;
#[doc = "Field `CPU_STALL_WAIT` writer - CPU stall wait cycles in fast_clk_rtc"]
pub type CPU_STALL_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER1_SPEC, 5, O>;
#[doc = "Field `CK8M_WAIT` reader - CK8M wait cycles in slow_clk_rtc"]
pub type CK8M_WAIT_R = crate::FieldReader;
#[doc = "Field `CK8M_WAIT` writer - CK8M wait cycles in slow_clk_rtc"]
pub type CK8M_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER1_SPEC, 8, O>;
#[doc = "Field `XTL_BUF_WAIT` reader - XTAL wait cycles in slow_clk_rtc"]
pub type XTL_BUF_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `XTL_BUF_WAIT` writer - XTAL wait cycles in slow_clk_rtc"]
pub type XTL_BUF_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER1_SPEC, 10, O, u16>;
#[doc = "Field `PLL_BUF_WAIT` reader - PLL wait cycles in slow_clk_rtc"]
pub type PLL_BUF_WAIT_R = crate::FieldReader;
#[doc = "Field `PLL_BUF_WAIT` writer - PLL wait cycles in slow_clk_rtc"]
pub type PLL_BUF_WAIT_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER1_SPEC, 8, O>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1")
            .field(
                "cpu_stall_en",
                &format_args!("{}", self.cpu_stall_en().bit()),
            )
            .field(
                "cpu_stall_wait",
                &format_args!("{}", self.cpu_stall_wait().bits()),
            )
            .field("ck8m_wait", &format_args!("{}", self.ck8m_wait().bits()))
            .field(
                "xtl_buf_wait",
                &format_args!("{}", self.xtl_buf_wait().bits()),
            )
            .field(
                "pll_buf_wait",
                &format_args!("{}", self.pll_buf_wait().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - CPU stall enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_stall_en(&mut self) -> CPU_STALL_EN_W<0> {
        CPU_STALL_EN_W::new(self)
    }
    #[doc = "Bits 1:5 - CPU stall wait cycles in fast_clk_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_stall_wait(&mut self) -> CPU_STALL_WAIT_W<1> {
        CPU_STALL_WAIT_W::new(self)
    }
    #[doc = "Bits 6:13 - CK8M wait cycles in slow_clk_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_wait(&mut self) -> CK8M_WAIT_W<6> {
        CK8M_WAIT_W::new(self)
    }
    #[doc = "Bits 14:23 - XTAL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn xtl_buf_wait(&mut self) -> XTL_BUF_WAIT_W<14> {
        XTL_BUF_WAIT_W::new(self)
    }
    #[doc = "Bits 24:31 - PLL wait cycles in slow_clk_rtc"]
    #[inline(always)]
    #[must_use]
    pub fn pll_buf_wait(&mut self) -> PLL_BUF_WAIT_W<24> {
        PLL_BUF_WAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "rtc configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1](index.html) module"]
pub struct TIMER1_SPEC;
impl crate::RegisterSpec for TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1::R](R) reader structure"]
impl crate::Readable for TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1::W](W) writer structure"]
impl crate::Writable for TIMER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER1 to value 0x2814_0403"]
impl crate::Resettable for TIMER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2814_0403;
}
