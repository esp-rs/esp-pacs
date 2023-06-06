#[doc = "Register `TIMER%s_CONF` reader"]
pub struct R(crate::R<TIMER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER%s_CONF` writer"]
pub struct W(crate::W<TIMER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER_CONF_SPEC>;
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
impl From<crate::W<TIMER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_RES` reader - This register is used to control the range of the counter in timer %s."]
pub type DUTY_RES_R = crate::FieldReader;
#[doc = "Field `DUTY_RES` writer - This register is used to control the range of the counter in timer %s."]
pub type DUTY_RES_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER_CONF_SPEC, 5, O>;
#[doc = "Field `CLK_DIV` reader - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
pub type CLK_DIV_R = crate::FieldReader<u32>;
#[doc = "Field `CLK_DIV` writer - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
pub type CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, TIMER_CONF_SPEC, 18, O, u32>;
#[doc = "Field `PAUSE` reader - This bit is used to suspend the counter in timer %s."]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - This bit is used to suspend the counter in timer %s."]
pub type PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, TIMER_CONF_SPEC, O>;
#[doc = "Field `RST` reader - This bit is used to reset timer %s. The counter will show 0 after reset."]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - This bit is used to reset timer %s. The counter will show 0 after reset."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, TIMER_CONF_SPEC, O>;
#[doc = "Field `TICK_SEL` reader - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 1'h0: SLOW_CLK 1'h1: REF_TICK"]
pub type TICK_SEL_R = crate::BitReader;
#[doc = "Field `TICK_SEL` writer - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 1'h0: SLOW_CLK 1'h1: REF_TICK"]
pub type TICK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, TIMER_CONF_SPEC, O>;
#[doc = "Field `PARA_UP` writer - Set this bit to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES."]
pub type PARA_UP_W<'a, const O: u8> = crate::BitWriter<'a, TIMER_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:4 - This register is used to control the range of the counter in timer %s."]
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits >> 5) & 0x0003_ffff)
    }
    #[doc = "Bit 23 - This bit is used to suspend the counter in timer %s."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset timer %s. The counter will show 0 after reset."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 1'h0: SLOW_CLK 1'h1: REF_TICK"]
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CONF")
            .field("duty_res", &format_args!("{}", self.duty_res().bits()))
            .field("clk_div", &format_args!("{}", self.clk_div().bits()))
            .field("pause", &format_args!("{}", self.pause().bit()))
            .field("rst", &format_args!("{}", self.rst().bit()))
            .field("tick_sel", &format_args!("{}", self.tick_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to control the range of the counter in timer %s."]
    #[inline(always)]
    #[must_use]
    pub fn duty_res(&mut self) -> DUTY_RES_W<0> {
        DUTY_RES_W::new(self)
    }
    #[doc = "Bits 5:22 - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
    #[inline(always)]
    #[must_use]
    pub fn clk_div(&mut self) -> CLK_DIV_W<5> {
        CLK_DIV_W::new(self)
    }
    #[doc = "Bit 23 - This bit is used to suspend the counter in timer %s."]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<23> {
        PAUSE_W::new(self)
    }
    #[doc = "Bit 24 - This bit is used to reset timer %s. The counter will show 0 after reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<24> {
        RST_W::new(self)
    }
    #[doc = "Bit 25 - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 1'h0: SLOW_CLK 1'h1: REF_TICK"]
    #[inline(always)]
    #[must_use]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<25> {
        TICK_SEL_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES."]
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<26> {
        PARA_UP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer_conf](index.html) module"]
pub struct TIMER_CONF_SPEC;
impl crate::RegisterSpec for TIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer_conf::R](R) reader structure"]
impl crate::Readable for TIMER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer_conf::W](W) writer structure"]
impl crate::Writable for TIMER_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER%s_CONF to value 0x0100_0000"]
impl crate::Resettable for TIMER_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
