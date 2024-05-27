#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `DUTY_RES` reader - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
pub type DUTY_RES_R = crate::FieldReader;
#[doc = "Field `DUTY_RES` writer - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
pub type DUTY_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DIV_NUM` reader - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `DIV_NUM` writer - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
pub type DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `PAUSE` reader - This bit is used to pause the counter in high speed timer0"]
pub type PAUSE_R = crate::BitReader;
#[doc = "Field `PAUSE` writer - This bit is used to pause the counter in high speed timer0"]
pub type PAUSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RST` reader - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
pub type RST_R = crate::BitReader;
#[doc = "Field `RST` writer - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
pub type RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TICK_SEL` reader - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
pub type TICK_SEL_R = crate::BitReader;
#[doc = "Field `TICK_SEL` writer - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
pub type TICK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
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
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("duty_res", &self.duty_res())
            .field("div_num", &self.div_num())
            .field("pause", &self.pause())
            .field("rst", &self.rst())
            .field("tick_sel", &self.tick_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - This register controls the range of the counter in high speed timer0. the counter range is \\[0 2**reg_hstimer0_lim\\] the max bit width for counter is 20."]
    #[inline(always)]
    #[must_use]
    pub fn duty_res(&mut self) -> DUTY_RES_W<CONF_SPEC> {
        DUTY_RES_W::new(self, 0)
    }
    #[doc = "Bits 5:22 - This register is used to configure parameter for divider in high speed timer0 the least significant eight bits represent the decimal part."]
    #[inline(always)]
    #[must_use]
    pub fn div_num(&mut self) -> DIV_NUM_W<CONF_SPEC> {
        DIV_NUM_W::new(self, 5)
    }
    #[doc = "Bit 23 - This bit is used to pause the counter in high speed timer0"]
    #[inline(always)]
    #[must_use]
    pub fn pause(&mut self) -> PAUSE_W<CONF_SPEC> {
        PAUSE_W::new(self, 23)
    }
    #[doc = "Bit 24 - This bit is used to reset high speed timer0 the counter will be 0 after reset."]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<CONF_SPEC> {
        RST_W::new(self, 24)
    }
    #[doc = "Bit 25 - This bit is used to choose apb_clk or ref_tick for high speed timer0. 1'b1:apb_clk 0:ref_tick"]
    #[inline(always)]
    #[must_use]
    pub fn tick_sel(&mut self) -> TICK_SEL_W<CONF_SPEC> {
        TICK_SEL_W::new(self, 25)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONF to value 0x0100_0000"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: u32 = 0x0100_0000;
}
