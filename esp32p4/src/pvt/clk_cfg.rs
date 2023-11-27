#[doc = "Register `CLK_CFG` reader"]
pub type R = crate::R<CLK_CFG_SPEC>;
#[doc = "Register `CLK_CFG` writer"]
pub type W = crate::W<CLK_CFG_SPEC>;
#[doc = "Field `PUMP_CLK_DIV_NUM` reader - needs field desc"]
pub type PUMP_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PUMP_CLK_DIV_NUM` writer - needs field desc"]
pub type PUMP_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MONITOR_CLK_PVT_EN` reader - needs field desc"]
pub type MONITOR_CLK_PVT_EN_R = crate::BitReader;
#[doc = "Field `MONITOR_CLK_PVT_EN` writer - needs field desc"]
pub type MONITOR_CLK_PVT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SEL` reader - select pvt clk"]
pub type CLK_SEL_R = crate::BitReader;
#[doc = "Field `CLK_SEL` writer - select pvt clk"]
pub type CLK_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - needs field desc"]
    #[inline(always)]
    pub fn pump_clk_div_num(&self) -> PUMP_CLK_DIV_NUM_R {
        PUMP_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - needs field desc"]
    #[inline(always)]
    pub fn monitor_clk_pvt_en(&self) -> MONITOR_CLK_PVT_EN_R {
        MONITOR_CLK_PVT_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - select pvt clk"]
    #[inline(always)]
    pub fn clk_sel(&self) -> CLK_SEL_R {
        CLK_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_CFG")
            .field(
                "pump_clk_div_num",
                &format_args!("{}", self.pump_clk_div_num().bits()),
            )
            .field(
                "monitor_clk_pvt_en",
                &format_args!("{}", self.monitor_clk_pvt_en().bit()),
            )
            .field("clk_sel", &format_args!("{}", self.clk_sel().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn pump_clk_div_num(&mut self) -> PUMP_CLK_DIV_NUM_W<CLK_CFG_SPEC> {
        PUMP_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn monitor_clk_pvt_en(&mut self) -> MONITOR_CLK_PVT_EN_W<CLK_CFG_SPEC> {
        MONITOR_CLK_PVT_EN_W::new(self, 8)
    }
    #[doc = "Bit 31 - select pvt clk"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sel(&mut self) -> CLK_SEL_W<CLK_CFG_SPEC> {
        CLK_SEL_W::new(self, 31)
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
#[doc = "configure pvt clk\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_CFG_SPEC;
impl crate::RegisterSpec for CLK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_cfg::R`](R) reader structure"]
impl crate::Readable for CLK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_cfg::W`](W) writer structure"]
impl crate::Writable for CLK_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_CFG to value 0"]
impl crate::Resettable for CLK_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
