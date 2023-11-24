#[doc = "Register `LSTIMER%s_CONF` reader"]
pub type R = crate::R<LSTIMER_CONF_SPEC>;
#[doc = "Register `LSTIMER%s_CONF` writer"]
pub type W = crate::W<LSTIMER_CONF_SPEC>;
#[doc = "Field `DUTY_RES` reader - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\] the max bit width for counter is 20."]
pub type DUTY_RES_R = crate::FieldReader;
#[doc = "Field `DUTY_RES` writer - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\] the max bit width for counter is 20."]
pub type DUTY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DIV_NUM` reader - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `DIV_NUM` writer - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `PAUSE` reader - This bit is used to pause the counter in low speed timer0."]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - This bit is used to pause the counter in low speed timer0."]
pub type PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICK_SEL` reader - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
pub type TICK_SEL_R = crate::BitReader;
#[doc = "Field `TICK_SEL` writer - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
pub type TICK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARA_UP` reader - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
pub type PARA_UP_R = crate::BitReader;
#[doc = "Field `PARA_UP` writer - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
pub type PARA_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    pub fn div_num(&self) -> DIV_NUM_R {
        DIV_NUM_R::new((self.bits >> 5) & 0x0003_ffff)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer0."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
    #[inline(always)]
    pub fn para_up(&self) -> PARA_UP_R {
        PARA_UP_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LSTIMER_CONF")
            .field("duty_res", &format_args!("{}", self.duty_res().bits()))
            .field("div_num", &format_args!("{}", self.div_num().bits()))
            .field("pause", &format_args!("{}", self.pause().bit()))
            .field("rst", &format_args!("{}", self.rst().bit()))
            .field("tick_sel", &format_args!("{}", self.tick_sel().bit()))
            .field("para_up", &format_args!("{}", self.para_up().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LSTIMER_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - This register controls the range of the counter in low speed timer0. the counter range is \\[0 2**reg_lstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    #[must_use]
    pub fn duty_res(&mut self) -> DUTY_RES_W<LSTIMER_CONF_SPEC> {
        DUTY_RES_W::new(self, 0)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in low speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    #[must_use]
    pub fn div_num(&mut self) -> DIV_NUM_W<LSTIMER_CONF_SPEC> {
        DIV_NUM_W::new(self, 5)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in low speed timer0."]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<LSTIMER_CONF_SPEC> {
        PAUSE_W::new(self, 23)
    }
    #[doc = "Bit 24 - This bit is used to reset low speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<LSTIMER_CONF_SPEC> {
        RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - This bit is used to choose slow_clk or ref_tick for low speed timer0. 1'b1:slow_clk 0:ref_tick"]
    #[inline(always)]
    #[must_use]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<LSTIMER_CONF_SPEC> {
        TICK_SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to update reg_div_num_lstime0 and reg_lstimer0_lim."]
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<LSTIMER_CONF_SPEC> {
        PARA_UP_W::new(self, 26)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lstimer_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lstimer_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LSTIMER_CONF_SPEC;
impl crate::RegisterSpec for LSTIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lstimer_conf::R`](R) reader structure"]
impl crate::Readable for LSTIMER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lstimer_conf::W`](W) writer structure"]
impl crate::Writable for LSTIMER_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LSTIMER%s_CONF to value 0x0100_0000"]
impl crate::Resettable for LSTIMER_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100_0000;
}
