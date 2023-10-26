#[doc = "Register `PVT_MONITOR_CONF` reader"]
pub type R = crate::R<PVT_MONITOR_CONF_SPEC>;
#[doc = "Register `PVT_MONITOR_CONF` writer"]
pub type W = crate::W<PVT_MONITOR_CONF_SPEC>;
#[doc = "Field `PVT_MONITOR_CLK_EN` reader - Set 1 to enable apb clock of pvt module"]
pub type PVT_MONITOR_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_MONITOR_CLK_EN` writer - Set 1 to enable apb clock of pvt module"]
pub type PVT_MONITOR_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVT_MONITOR_RST_EN` reader - Set 0 to reset all pvt monitor module"]
pub type PVT_MONITOR_RST_EN_R = crate::BitReader;
#[doc = "Field `PVT_MONITOR_RST_EN` writer - Set 0 to reset all pvt monitor module"]
pub type PVT_MONITOR_RST_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVT_MONITOR_SITE1_CLK_EN` reader - Set 1 to enable function clock of modem pvt module"]
pub type PVT_MONITOR_SITE1_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_MONITOR_SITE1_CLK_EN` writer - Set 1 to enable function clock of modem pvt module"]
pub type PVT_MONITOR_SITE1_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVT_MONITOR_SITE2_CLK_EN` reader - Set 1 to enable function clock of cpu pvt module"]
pub type PVT_MONITOR_SITE2_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_MONITOR_SITE2_CLK_EN` writer - Set 1 to enable function clock of cpu pvt module"]
pub type PVT_MONITOR_SITE2_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `PVT_MONITOR_SITE3_CLK_EN` reader - Set 1 to enable function clock of hp_peri pvt module"]
pub type PVT_MONITOR_SITE3_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_MONITOR_SITE3_CLK_EN` writer - Set 1 to enable function clock of hp_peri pvt module"]
pub type PVT_MONITOR_SITE3_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable apb clock of pvt module"]
    #[inline(always)]
    pub fn pvt_monitor_clk_en(&self) -> PVT_MONITOR_CLK_EN_R {
        PVT_MONITOR_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset all pvt monitor module"]
    #[inline(always)]
    pub fn pvt_monitor_rst_en(&self) -> PVT_MONITOR_RST_EN_R {
        PVT_MONITOR_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable function clock of modem pvt module"]
    #[inline(always)]
    pub fn pvt_monitor_site1_clk_en(&self) -> PVT_MONITOR_SITE1_CLK_EN_R {
        PVT_MONITOR_SITE1_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to enable function clock of cpu pvt module"]
    #[inline(always)]
    pub fn pvt_monitor_site2_clk_en(&self) -> PVT_MONITOR_SITE2_CLK_EN_R {
        PVT_MONITOR_SITE2_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to enable function clock of hp_peri pvt module"]
    #[inline(always)]
    pub fn pvt_monitor_site3_clk_en(&self) -> PVT_MONITOR_SITE3_CLK_EN_R {
        PVT_MONITOR_SITE3_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PVT_MONITOR_CONF")
            .field(
                "pvt_monitor_clk_en",
                &format_args!("{}", self.pvt_monitor_clk_en().bit()),
            )
            .field(
                "pvt_monitor_rst_en",
                &format_args!("{}", self.pvt_monitor_rst_en().bit()),
            )
            .field(
                "pvt_monitor_site1_clk_en",
                &format_args!("{}", self.pvt_monitor_site1_clk_en().bit()),
            )
            .field(
                "pvt_monitor_site2_clk_en",
                &format_args!("{}", self.pvt_monitor_site2_clk_en().bit()),
            )
            .field(
                "pvt_monitor_site3_clk_en",
                &format_args!("{}", self.pvt_monitor_site3_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PVT_MONITOR_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable apb clock of pvt module"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_monitor_clk_en(&mut self) -> PVT_MONITOR_CLK_EN_W<PVT_MONITOR_CONF_SPEC, 0> {
        PVT_MONITOR_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 0 to reset all pvt monitor module"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_monitor_rst_en(&mut self) -> PVT_MONITOR_RST_EN_W<PVT_MONITOR_CONF_SPEC, 1> {
        PVT_MONITOR_RST_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable function clock of modem pvt module"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_monitor_site1_clk_en(
        &mut self,
    ) -> PVT_MONITOR_SITE1_CLK_EN_W<PVT_MONITOR_CONF_SPEC, 2> {
        PVT_MONITOR_SITE1_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set 1 to enable function clock of cpu pvt module"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_monitor_site2_clk_en(
        &mut self,
    ) -> PVT_MONITOR_SITE2_CLK_EN_W<PVT_MONITOR_CONF_SPEC, 3> {
        PVT_MONITOR_SITE2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 4 - Set 1 to enable function clock of hp_peri pvt module"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_monitor_site3_clk_en(
        &mut self,
    ) -> PVT_MONITOR_SITE3_CLK_EN_W<PVT_MONITOR_CONF_SPEC, 4> {
        PVT_MONITOR_SITE3_CLK_EN_W::new(self)
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
#[doc = "PVT_MONITOR configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pvt_monitor_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pvt_monitor_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PVT_MONITOR_CONF_SPEC;
impl crate::RegisterSpec for PVT_MONITOR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pvt_monitor_conf::R`](R) reader structure"]
impl crate::Readable for PVT_MONITOR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pvt_monitor_conf::W`](W) writer structure"]
impl crate::Writable for PVT_MONITOR_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PVT_MONITOR_CONF to value 0x1d"]
impl crate::Resettable for PVT_MONITOR_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x1d;
}
