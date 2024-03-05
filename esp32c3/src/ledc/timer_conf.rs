#[doc = "Register `TIMER%s_CONF` reader"]
pub type R = crate::R<TIMER_CONF_SPEC>;
#[doc = "Register `TIMER%s_CONF` writer"]
pub type W = crate::W<TIMER_CONF_SPEC>;
#[doc = "Field `DUTY_RES` reader - reg_lstimer0_duty_res."]
pub type DUTY_RES_R = crate::FieldReader;
#[doc = "Field `DUTY_RES` writer - reg_lstimer0_duty_res."]
pub type DUTY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK_DIV` reader - reg_clk_div_lstimer0."]
pub type CLK_DIV_R = crate::FieldReader<u32>;
#[doc = "Field `CLK_DIV` writer - reg_clk_div_lstimer0."]
pub type CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `PAUSE` reader - reg_lstimer0_pause."]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - reg_lstimer0_pause."]
pub type PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - reg_lstimer0_rst."]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - reg_lstimer0_rst."]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICK_SEL` reader - reg_tick_sel_lstimer0."]
pub type TICK_SEL_R = crate::BitReader;
#[doc = "Field `TICK_SEL` writer - reg_tick_sel_lstimer0."]
pub type TICK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARA_UP` writer - reg_lstimer0_para_up."]
pub type PARA_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - reg_lstimer0_duty_res."]
    #[inline(always)]
    pub fn duty_res(&self) -> DUTY_RES_R {
        DUTY_RES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer0."]
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new((self.bits >> 4) & 0x0003_ffff)
    }
    #[doc = "Bit 22 - reg_lstimer0_pause."]
    #[inline(always)]
    pub fn pause(&self) -> PAUSE_R {
        PAUSE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - reg_lstimer0_rst."]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer0."]
    #[inline(always)]
    pub fn tick_sel(&self) -> TICK_SEL_R {
        TICK_SEL_R::new(((self.bits >> 24) & 1) != 0)
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_lstimer0_duty_res."]
    #[inline(always)]
    #[must_use]
    pub fn duty_res(&mut self) -> DUTY_RES_W<TIMER_CONF_SPEC> {
        DUTY_RES_W::new(self, 0)
    }
    #[doc = "Bits 4:21 - reg_clk_div_lstimer0."]
    #[inline(always)]
    #[must_use]
    pub fn clk_div(&mut self) -> CLK_DIV_W<TIMER_CONF_SPEC> {
        CLK_DIV_W::new(self, 4)
    }
    #[doc = "Bit 22 - reg_lstimer0_pause."]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<TIMER_CONF_SPEC> {
        PAUSE_W::new(self, 22)
    }
    #[doc = "Bit 23 - reg_lstimer0_rst."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<TIMER_CONF_SPEC> {
        RST_W::new(self, 23)
    }
    #[doc = "Bit 24 - reg_tick_sel_lstimer0."]
    #[inline(always)]
    #[must_use]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<TIMER_CONF_SPEC> {
        TICK_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - reg_lstimer0_para_up."]
    #[inline(always)]
    #[must_use]
    pub fn para_up(&mut self) -> PARA_UP_W<TIMER_CONF_SPEC> {
        PARA_UP_W::new(self, 25)
    }
}
#[doc = "LEDC_LSTIMER%s_CONF.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CONF_SPEC;
impl crate::RegisterSpec for TIMER_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_conf::R`](R) reader structure"]
impl crate::Readable for TIMER_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_conf::W`](W) writer structure"]
impl crate::Writable for TIMER_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER%s_CONF to value 0x0080_0000"]
impl crate::Resettable for TIMER_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0080_0000;
}
