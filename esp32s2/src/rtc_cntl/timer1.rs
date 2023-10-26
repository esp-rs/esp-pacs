#[doc = "Register `TIMER1` reader"]
pub type R = crate::R<TIMER1_SPEC>;
#[doc = "Register `TIMER1` writer"]
pub type W = crate::W<TIMER1_SPEC>;
#[doc = "Field `CPU_STALL_EN` reader - Enables CPU stalling."]
pub type CPU_STALL_EN_R = crate::BitReader;
#[doc = "Field `CPU_STALL_EN` writer - Enables CPU stalling."]
pub type CPU_STALL_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CPU_STALL_WAIT` reader - Sets the CPU stall waiting cycle (using the RTC fast clock)."]
pub type CPU_STALL_WAIT_R = crate::FieldReader;
#[doc = "Field `CPU_STALL_WAIT` writer - Sets the CPU stall waiting cycle (using the RTC fast clock)."]
pub type CPU_STALL_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CK8M_WAIT` reader - Sets the 8 MHz clock waiting (using the RTC slow clock)."]
pub type CK8M_WAIT_R = crate::FieldReader;
#[doc = "Field `CK8M_WAIT` writer - Sets the 8 MHz clock waiting (using the RTC slow clock)."]
pub type CK8M_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `XTL_BUF_WAIT` reader - Sets the XTAL waiting cycle (using the RTC slow clock)."]
pub type XTL_BUF_WAIT_R = crate::FieldReader<u16>;
#[doc = "Field `XTL_BUF_WAIT` writer - Sets the XTAL waiting cycle (using the RTC slow clock)."]
pub type XTL_BUF_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 10, O, u16>;
#[doc = "Field `PLL_BUF_WAIT` reader - Sets the PLL waiting cycle (using the RTC slow clock)."]
pub type PLL_BUF_WAIT_R = crate::FieldReader;
#[doc = "Field `PLL_BUF_WAIT` writer - Sets the PLL waiting cycle (using the RTC slow clock)."]
pub type PLL_BUF_WAIT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bit 0 - Enables CPU stalling."]
    #[inline(always)]
    pub fn cpu_stall_en(&self) -> CPU_STALL_EN_R {
        CPU_STALL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Sets the CPU stall waiting cycle (using the RTC fast clock)."]
    #[inline(always)]
    pub fn cpu_stall_wait(&self) -> CPU_STALL_WAIT_R {
        CPU_STALL_WAIT_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:13 - Sets the 8 MHz clock waiting (using the RTC slow clock)."]
    #[inline(always)]
    pub fn ck8m_wait(&self) -> CK8M_WAIT_R {
        CK8M_WAIT_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:23 - Sets the XTAL waiting cycle (using the RTC slow clock)."]
    #[inline(always)]
    pub fn xtl_buf_wait(&self) -> XTL_BUF_WAIT_R {
        XTL_BUF_WAIT_R::new(((self.bits >> 14) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:31 - Sets the PLL waiting cycle (using the RTC slow clock)."]
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
    #[doc = "Bit 0 - Enables CPU stalling."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_stall_en(&mut self) -> CPU_STALL_EN_W<TIMER1_SPEC, 0> {
        CPU_STALL_EN_W::new(self)
    }
    #[doc = "Bits 1:5 - Sets the CPU stall waiting cycle (using the RTC fast clock)."]
    #[inline(always)]
    #[must_use]
    pub fn cpu_stall_wait(&mut self) -> CPU_STALL_WAIT_W<TIMER1_SPEC, 1> {
        CPU_STALL_WAIT_W::new(self)
    }
    #[doc = "Bits 6:13 - Sets the 8 MHz clock waiting (using the RTC slow clock)."]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_wait(&mut self) -> CK8M_WAIT_W<TIMER1_SPEC, 6> {
        CK8M_WAIT_W::new(self)
    }
    #[doc = "Bits 14:23 - Sets the XTAL waiting cycle (using the RTC slow clock)."]
    #[inline(always)]
    #[must_use]
    pub fn xtl_buf_wait(&mut self) -> XTL_BUF_WAIT_W<TIMER1_SPEC, 14> {
        XTL_BUF_WAIT_W::new(self)
    }
    #[doc = "Bits 24:31 - Sets the PLL waiting cycle (using the RTC slow clock)."]
    #[inline(always)]
    #[must_use]
    pub fn pll_buf_wait(&mut self) -> PLL_BUF_WAIT_W<TIMER1_SPEC, 24> {
        PLL_BUF_WAIT_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Configures CPU stall options\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1_SPEC;
impl crate::RegisterSpec for TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer1::R`](R) reader structure"]
impl crate::Readable for TIMER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer1::W`](W) writer structure"]
impl crate::Writable for TIMER1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER1 to value 0x2814_0403"]
impl crate::Resettable for TIMER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x2814_0403;
}
