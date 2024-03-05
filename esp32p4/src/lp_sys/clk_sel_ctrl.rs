#[doc = "Register `CLK_SEL_CTRL` reader"]
pub type R = crate::R<CLK_SEL_CTRL_SPEC>;
#[doc = "Register `CLK_SEL_CTRL` writer"]
pub type W = crate::W<CLK_SEL_CTRL_SPEC>;
#[doc = "Field `ENA_SW_SEL_SYS_CLK` reader - reserved"]
pub type ENA_SW_SEL_SYS_CLK_R = crate::BitReader;
#[doc = "Field `ENA_SW_SEL_SYS_CLK` writer - reserved"]
pub type ENA_SW_SEL_SYS_CLK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_SYS_CLK_SRC_SEL` reader - reserved"]
pub type SW_SYS_CLK_SRC_SEL_R = crate::BitReader;
#[doc = "Field `SW_SYS_CLK_SRC_SEL` writer - reserved"]
pub type SW_SYS_CLK_SRC_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - reserved"]
    #[inline(always)]
    pub fn ena_sw_sel_sys_clk(&self) -> ENA_SW_SEL_SYS_CLK_R {
        ENA_SW_SEL_SYS_CLK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn sw_sys_clk_src_sel(&self) -> SW_SYS_CLK_SRC_SEL_R {
        SW_SYS_CLK_SRC_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_SEL_CTRL")
            .field(
                "ena_sw_sel_sys_clk",
                &format_args!("{}", self.ena_sw_sel_sys_clk().bit()),
            )
            .field(
                "sw_sys_clk_src_sel",
                &format_args!("{}", self.sw_sys_clk_src_sel().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_SEL_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 16 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn ena_sw_sel_sys_clk(&mut self) -> ENA_SW_SEL_SYS_CLK_W<CLK_SEL_CTRL_SPEC> {
        ENA_SW_SEL_SYS_CLK_W::new(self, 16)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn sw_sys_clk_src_sel(&mut self) -> SW_SYS_CLK_SRC_SEL_W<CLK_SEL_CTRL_SPEC> {
        SW_SYS_CLK_SRC_SEL_W::new(self, 17)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sel_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sel_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SEL_CTRL_SPEC;
impl crate::RegisterSpec for CLK_SEL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_sel_ctrl::R`](R) reader structure"]
impl crate::Readable for CLK_SEL_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_sel_ctrl::W`](W) writer structure"]
impl crate::Writable for CLK_SEL_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_SEL_CTRL to value 0"]
impl crate::Resettable for CLK_SEL_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
