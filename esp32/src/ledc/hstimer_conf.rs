#[doc = "Register `HSTIMER%s_CONF` reader"]
pub struct R(crate::R<HSTIMER_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTIMER_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTIMER_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTIMER_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTIMER%s_CONF` writer"]
pub struct W(crate::W<HSTIMER_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTIMER_CONF_SPEC>;
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
impl From<crate::W<HSTIMER_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTIMER_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTY_RES` reader - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
pub type DUTY_RES_R = crate::FieldReader;
#[doc = "Field `DUTY_RES` writer - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
pub type DUTY_RES_W<'a, const O: u8> = crate::FieldWriter<'a, HSTIMER_CONF_SPEC, 5, O>;
#[doc = "Field `DIV_NUM` reader - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DIV_NUM` writer - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, HSTIMER_CONF_SPEC, 18, O, u32, u32>;
#[doc = "Field `PAUSE` reader - This bit is used to pause the counter in high speed timer0"]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - This bit is used to pause the counter in high speed timer0"]
pub type PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, HSTIMER_CONF_SPEC, O>;
#[doc = "Field `RST` reader - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, HSTIMER_CONF_SPEC, O>;
#[doc = "Field `TICK_SEL` reader - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
pub type TICK_SEL_R = crate::BitReader;
#[doc = "Field `TICK_SEL` writer - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
pub type TICK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, HSTIMER_CONF_SPEC, O>;
#[doc = "Field `LIM` reader - "]
pub type LIM_R = crate::FieldReader;
#[doc = "Field `LIM` writer - "]
pub type LIM_W<'a, const O: u8> = crate::FieldWriter<'a, HSTIMER_CONF_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num(&self) -> DIV_NUM_R {
        DIV_NUM_R::new((self.bits >> 5) & 0x0003_ffff)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer0"]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    pub fn lim(&self) -> LIM_R {
        LIM_R::new(((self.bits >> 31) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSTIMER_CONF")
            .field("duty_res", &format_args!("{}", self.duty_res().bits()))
            .field("div_num", &format_args!("{}", self.div_num().bits()))
            .field("pause", &format_args!("{}", self.pause().bit()))
            .field("rst", &format_args!("{}", self.rst().bit()))
            .field("tick_sel", &format_args!("{}", self.tick_sel().bit()))
            .field("lim", &format_args!("{}", self.lim().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HSTIMER_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    #[must_use]
    pub fn duty_res(&mut self) -> DUTY_RES_W<0> {
        DUTY_RES_W::new(self)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    #[must_use]
    pub fn div_num(&mut self) -> DIV_NUM_W<5> {
        DIV_NUM_W::new(self)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer0"]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<23> {
        PAUSE_W::new(self)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<24> {
        RST_W::new(self)
    }
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    #[must_use]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<25> {
        TICK_SEL_W::new(self)
    }
    #[doc = "Bits 31:35"]
    #[inline(always)]
    #[must_use]
    pub fn lim(&mut self) -> LIM_W<31> {
        LIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstimer_conf](index.html) module"]
pub struct HSTIMER_CONF_SPEC;
impl crate::RegisterSpec for HSTIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstimer_conf::R](R) reader structure"]
impl crate::Readable for HSTIMER_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstimer_conf::W](W) writer structure"]
impl crate::Writable for HSTIMER_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HSTIMER%s_CONF to value 0x0100_0000"]
impl crate::Resettable for HSTIMER_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
