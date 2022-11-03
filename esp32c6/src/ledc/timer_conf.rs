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
#[doc = "Field `TIMER_DUTY_RES` reader - This register is used to control the range of the counter in timer %s."]
pub type TIMER_DUTY_RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER_DUTY_RES` writer - This register is used to control the range of the counter in timer %s."]
pub type TIMER_DUTY_RES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER_CONF_SPEC, u8, u8, 5, O>;
#[doc = "Field `CLK_DIV_TIMER` reader - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
pub type CLK_DIV_TIMER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CLK_DIV_TIMER` writer - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
pub type CLK_DIV_TIMER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TIMER_CONF_SPEC, u32, u32, 18, O>;
#[doc = "Field `TIMER_PAUSE` reader - This bit is used to suspend the counter in timer %s."]
pub type TIMER_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_PAUSE` writer - This bit is used to suspend the counter in timer %s."]
pub type TIMER_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CONF_SPEC, bool, O>;
#[doc = "Field `TIMER_RST` reader - This bit is used to reset timer %s. The counter will show 0 after reset."]
pub type TIMER_RST_R = crate::BitReader<bool>;
#[doc = "Field `TIMER_RST` writer - This bit is used to reset timer %s. The counter will show 0 after reset."]
pub type TIMER_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CONF_SPEC, bool, O>;
#[doc = "Field `TICK_SEL_TIMER` reader - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 1'h0: SLOW_CLK 1'h1: REF_TICK"]
pub type TICK_SEL_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `TICK_SEL_TIMER` writer - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 1'h0: SLOW_CLK 1'h1: REF_TICK"]
pub type TICK_SEL_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CONF_SPEC, bool, O>;
#[doc = "Field `TIMER_PARA_UP` writer - Set this bit to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES."]
pub type TIMER_PARA_UP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMER_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - This register is used to control the range of the counter in timer %s."]
    #[inline(always)]
    pub fn timer_duty_res(&self) -> TIMER_DUTY_RES_R {
        TIMER_DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
    #[inline(always)]
    pub fn clk_div_timer(&self) -> CLK_DIV_TIMER_R {
        CLK_DIV_TIMER_R::new((self.bits >> 5) & 0x0003_ffff)
    }
    #[doc = "Bit 23 - This bit is used to suspend the counter in timer %s."]
    #[inline(always)]
    pub fn timer_pause(&self) -> TIMER_PAUSE_R {
        TIMER_PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset timer %s. The counter will show 0 after reset."]
    #[inline(always)]
    pub fn timer_rst(&self) -> TIMER_RST_R {
        TIMER_RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 1'h0: SLOW_CLK 1'h1: REF_TICK"]
    #[inline(always)]
    pub fn tick_sel_timer(&self) -> TICK_SEL_TIMER_R {
        TICK_SEL_TIMER_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register is used to control the range of the counter in timer %s."]
    #[inline(always)]
    #[must_use]
    pub fn timer_duty_res(&mut self) -> TIMER_DUTY_RES_W<0> {
        TIMER_DUTY_RES_W::new(self)
    }
    #[doc = "Bits 5:22 - This register is used to configure the divisor for the divider in timer %s. The least significant eight bits represent the fractional part."]
    #[inline(always)]
    #[must_use]
    pub fn clk_div_timer(&mut self) -> CLK_DIV_TIMER_W<5> {
        CLK_DIV_TIMER_W::new(self)
    }
    #[doc = "Bit 23 - This bit is used to suspend the counter in timer %s."]
    #[inline(always)]
    #[must_use]
    pub fn timer_pause(&mut self) -> TIMER_PAUSE_W<23> {
        TIMER_PAUSE_W::new(self)
    }
    #[doc = "Bit 24 - This bit is used to reset timer %s. The counter will show 0 after reset."]
    #[inline(always)]
    #[must_use]
    pub fn timer_rst(&mut self) -> TIMER_RST_W<24> {
        TIMER_RST_W::new(self)
    }
    #[doc = "Bit 25 - This bit is used to select clock for timer %s. When this bit is set to 1 LEDC_APB_CLK_SEL\\[1:0\\] should be 1, otherwise the timer clock may be not accurate. 1'h0: SLOW_CLK 1'h1: REF_TICK"]
    #[inline(always)]
    #[must_use]
    pub fn tick_sel_timer(&mut self) -> TICK_SEL_TIMER_W<25> {
        TICK_SEL_TIMER_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to update LEDC_CLK_DIV_TIMER%s and LEDC_TIMER%s_DUTY_RES."]
    #[inline(always)]
    #[must_use]
    pub fn timer_para_up(&mut self) -> TIMER_PARA_UP_W<26> {
        TIMER_PARA_UP_W::new(self)
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
