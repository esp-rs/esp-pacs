///Register `MISC_CONF` reader
pub type R = crate::R<MISC_CONF_SPEC>;
///Register `MISC_CONF` writer
pub type W = crate::W<MISC_CONF_SPEC>;
///Field `AHBM_RST_INTER` reader - Set this bit, then clear this bit to reset the internal ahb FSM.
pub type AHBM_RST_INTER_R = crate::BitReader;
///Field `AHBM_RST_INTER` writer - Set this bit, then clear this bit to reset the internal ahb FSM.
pub type AHBM_RST_INTER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARB_PRI_DIS` reader - Set this bit to disable priority arbitration function.
pub type ARB_PRI_DIS_R = crate::BitReader;
///Field `ARB_PRI_DIS` writer - Set this bit to disable priority arbitration function.
pub type ARB_PRI_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_EN` reader - reg_clk_en
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - reg_clk_en
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set this bit, then clear this bit to reset the internal ahb FSM.
    #[inline(always)]
    pub fn ahbm_rst_inter(&self) -> AHBM_RST_INTER_R {
        AHBM_RST_INTER_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - Set this bit to disable priority arbitration function.
    #[inline(always)]
    pub fn arb_pri_dis(&self) -> ARB_PRI_DIS_R {
        ARB_PRI_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - reg_clk_en
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC_CONF")
            .field("ahbm_rst_inter", &self.ahbm_rst_inter())
            .field("arb_pri_dis", &self.arb_pri_dis())
            .field("clk_en", &self.clk_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit, then clear this bit to reset the internal ahb FSM.
    #[inline(always)]
    #[must_use]
    pub fn ahbm_rst_inter(&mut self) -> AHBM_RST_INTER_W<MISC_CONF_SPEC> {
        AHBM_RST_INTER_W::new(self, 0)
    }
    ///Bit 2 - Set this bit to disable priority arbitration function.
    #[inline(always)]
    #[must_use]
    pub fn arb_pri_dis(&mut self) -> ARB_PRI_DIS_W<MISC_CONF_SPEC> {
        ARB_PRI_DIS_W::new(self, 2)
    }
    ///Bit 3 - reg_clk_en
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<MISC_CONF_SPEC> {
        CLK_EN_W::new(self, 3)
    }
}
/**DMA_MISC_CONF_REG.

You can [`read`](crate::generic::Reg::read) this register and get [`misc_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MISC_CONF_SPEC;
impl crate::RegisterSpec for MISC_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`misc_conf::R`](R) reader structure
impl crate::Readable for MISC_CONF_SPEC {}
///`write(|w| ..)` method takes [`misc_conf::W`](W) writer structure
impl crate::Writable for MISC_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MISC_CONF to value 0
impl crate::Resettable for MISC_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
