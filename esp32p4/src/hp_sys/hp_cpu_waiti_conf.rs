#[doc = "Register `HP_CPU_WAITI_CONF` reader"]
pub type R = crate::R<HP_CPU_WAITI_CONF_SPEC>;
#[doc = "Register `HP_CPU_WAITI_CONF` writer"]
pub type W = crate::W<HP_CPU_WAITI_CONF_SPEC>;
#[doc = "Field `HP_CPU_WAIT_MODE_FORCE_ON` reader - Set 1 to force cpu_waiti_clk enable."]
pub type HP_CPU_WAIT_MODE_FORCE_ON_R = crate::BitReader;
#[doc = "Field `HP_CPU_WAIT_MODE_FORCE_ON` writer - Set 1 to force cpu_waiti_clk enable."]
pub type HP_CPU_WAIT_MODE_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_CPU_WAITI_DELAY_NUM` reader - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub type HP_CPU_WAITI_DELAY_NUM_R = crate::FieldReader;
#[doc = "Field `HP_CPU_WAITI_DELAY_NUM` writer - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
pub type HP_CPU_WAITI_DELAY_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Set 1 to force cpu_waiti_clk enable."]
    #[inline(always)]
    pub fn hp_cpu_wait_mode_force_on(&self) -> HP_CPU_WAIT_MODE_FORCE_ON_R {
        HP_CPU_WAIT_MODE_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    pub fn hp_cpu_waiti_delay_num(&self) -> HP_CPU_WAITI_DELAY_NUM_R {
        HP_CPU_WAITI_DELAY_NUM_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_CPU_WAITI_CONF")
            .field(
                "hp_cpu_wait_mode_force_on",
                &format_args!("{}", self.hp_cpu_wait_mode_force_on().bit()),
            )
            .field(
                "hp_cpu_waiti_delay_num",
                &format_args!("{}", self.hp_cpu_waiti_delay_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_CPU_WAITI_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to force cpu_waiti_clk enable."]
    #[inline(always)]
    #[must_use]
    pub fn hp_cpu_wait_mode_force_on(
        &mut self,
    ) -> HP_CPU_WAIT_MODE_FORCE_ON_W<HP_CPU_WAITI_CONF_SPEC> {
        HP_CPU_WAIT_MODE_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - This field used to set delay cycle when cpu enter waiti mode, after delay waiti_clk will close"]
    #[inline(always)]
    #[must_use]
    pub fn hp_cpu_waiti_delay_num(&mut self) -> HP_CPU_WAITI_DELAY_NUM_W<HP_CPU_WAITI_CONF_SPEC> {
        HP_CPU_WAITI_DELAY_NUM_W::new(self, 1)
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
#[doc = "CPU_WAITI configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hp_cpu_waiti_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hp_cpu_waiti_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_CPU_WAITI_CONF_SPEC;
impl crate::RegisterSpec for HP_CPU_WAITI_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_cpu_waiti_conf::R`](R) reader structure"]
impl crate::Readable for HP_CPU_WAITI_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_cpu_waiti_conf::W`](W) writer structure"]
impl crate::Writable for HP_CPU_WAITI_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_CPU_WAITI_CONF to value 0x01"]
impl crate::Resettable for HP_CPU_WAITI_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
