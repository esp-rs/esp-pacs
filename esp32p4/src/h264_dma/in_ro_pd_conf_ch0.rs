#[doc = "Register `IN_RO_PD_CONF_CH0` reader"]
pub type R = crate::R<IN_RO_PD_CONF_CH0_SPEC>;
#[doc = "Register `IN_RO_PD_CONF_CH0` writer"]
pub type W = crate::W<IN_RO_PD_CONF_CH0_SPEC>;
#[doc = "Field `IN_RO_RAM_CLK_FO_CH0` reader - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type IN_RO_RAM_CLK_FO_CH0_R = crate::BitReader;
#[doc = "Field `IN_RO_RAM_CLK_FO_CH0` writer - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
pub type IN_RO_RAM_CLK_FO_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    pub fn in_ro_ram_clk_fo_ch0(&self) -> IN_RO_RAM_CLK_FO_CH0_R {
        IN_RO_RAM_CLK_FO_CH0_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_RO_PD_CONF_CH0")
            .field(
                "in_ro_ram_clk_fo_ch0",
                &format_args!("{}", self.in_ro_ram_clk_fo_ch0().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_RO_PD_CONF_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 6 - 1: Force to open the clock and bypass the gate-clock when accessing the RAM in DMA. 0: A gate-clock will be used when accessing the RAM in DMA."]
    #[inline(always)]
    #[must_use]
    pub fn in_ro_ram_clk_fo_ch0(&mut self) -> IN_RO_RAM_CLK_FO_CH0_W<IN_RO_PD_CONF_CH0_SPEC> {
        IN_RO_RAM_CLK_FO_CH0_W::new(self, 6)
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
#[doc = "RX CH0 reorder power config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_ro_pd_conf_ch0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_ro_pd_conf_ch0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_RO_PD_CONF_CH0_SPEC;
impl crate::RegisterSpec for IN_RO_PD_CONF_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_ro_pd_conf_ch0::R`](R) reader structure"]
impl crate::Readable for IN_RO_PD_CONF_CH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_ro_pd_conf_ch0::W`](W) writer structure"]
impl crate::Writable for IN_RO_PD_CONF_CH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_RO_PD_CONF_CH0 to value 0"]
impl crate::Resettable for IN_RO_PD_CONF_CH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
