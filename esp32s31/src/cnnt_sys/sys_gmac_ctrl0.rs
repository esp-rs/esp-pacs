#[doc = "Register `SYS_GMAC_CTRL0` reader"]
pub type R = crate::R<SYS_GMAC_CTRL0_SPEC>;
#[doc = "Register `SYS_GMAC_CTRL0` writer"]
pub type W = crate::W<SYS_GMAC_CTRL0_SPEC>;
#[doc = "Field `SYS_PTP_PPS` reader - "]
pub type SYS_PTP_PPS_R = crate::BitReader;
#[doc = "Field `SYS_SBD_FLOWCTRL` reader - "]
pub type SYS_SBD_FLOWCTRL_R = crate::BitReader;
#[doc = "Field `SYS_SBD_FLOWCTRL` writer - "]
pub type SYS_SBD_FLOWCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_PHY_INTF_SEL` reader - "]
pub type SYS_PHY_INTF_SEL_R = crate::FieldReader;
#[doc = "Field `SYS_PHY_INTF_SEL` writer - "]
pub type SYS_PHY_INTF_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYS_GMAC_MEM_CLK_FORCE_ON` reader - "]
pub type SYS_GMAC_MEM_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `SYS_GMAC_MEM_CLK_FORCE_ON` writer - "]
pub type SYS_GMAC_MEM_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_GMAC_RST_CLK_TX_N` reader - "]
pub type SYS_GMAC_RST_CLK_TX_N_R = crate::BitReader;
#[doc = "Field `SYS_GMAC_RST_CLK_RX_N` reader - "]
pub type SYS_GMAC_RST_CLK_RX_N_R = crate::BitReader;
#[doc = "Field `SYS_GMAC_CACTIVE` reader - "]
pub type SYS_GMAC_CACTIVE_R = crate::BitReader;
#[doc = "Field `SYS_GMAC_CSYSACK` reader - "]
pub type SYS_GMAC_CSYSACK_R = crate::BitReader;
#[doc = "Field `SYS_GMAC_CSYSREQ` reader - "]
pub type SYS_GMAC_CSYSREQ_R = crate::BitReader;
#[doc = "Field `SYS_GMAC_CSYSREQ` writer - "]
pub type SYS_GMAC_CSYSREQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_GMAC_APB_POSTW_EN` reader - "]
pub type SYS_GMAC_APB_POSTW_EN_R = crate::BitReader;
#[doc = "Field `SYS_GMAC_APB_POSTW_EN` writer - "]
pub type SYS_GMAC_APB_POSTW_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_GMAC_MEM_PRDYN` reader - "]
pub type SYS_GMAC_MEM_PRDYN_R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sys_ptp_pps(&self) -> SYS_PTP_PPS_R {
        SYS_PTP_PPS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sys_sbd_flowctrl(&self) -> SYS_SBD_FLOWCTRL_R {
        SYS_SBD_FLOWCTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn sys_phy_intf_sel(&self) -> SYS_PHY_INTF_SEL_R {
        SYS_PHY_INTF_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sys_gmac_mem_clk_force_on(&self) -> SYS_GMAC_MEM_CLK_FORCE_ON_R {
        SYS_GMAC_MEM_CLK_FORCE_ON_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sys_gmac_rst_clk_tx_n(&self) -> SYS_GMAC_RST_CLK_TX_N_R {
        SYS_GMAC_RST_CLK_TX_N_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sys_gmac_rst_clk_rx_n(&self) -> SYS_GMAC_RST_CLK_RX_N_R {
        SYS_GMAC_RST_CLK_RX_N_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn sys_gmac_cactive(&self) -> SYS_GMAC_CACTIVE_R {
        SYS_GMAC_CACTIVE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn sys_gmac_csysack(&self) -> SYS_GMAC_CSYSACK_R {
        SYS_GMAC_CSYSACK_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sys_gmac_csysreq(&self) -> SYS_GMAC_CSYSREQ_R {
        SYS_GMAC_CSYSREQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sys_gmac_apb_postw_en(&self) -> SYS_GMAC_APB_POSTW_EN_R {
        SYS_GMAC_APB_POSTW_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn sys_gmac_mem_prdyn(&self) -> SYS_GMAC_MEM_PRDYN_R {
        SYS_GMAC_MEM_PRDYN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_GMAC_CTRL0")
            .field("sys_ptp_pps", &self.sys_ptp_pps())
            .field("sys_sbd_flowctrl", &self.sys_sbd_flowctrl())
            .field("sys_phy_intf_sel", &self.sys_phy_intf_sel())
            .field(
                "sys_gmac_mem_clk_force_on",
                &self.sys_gmac_mem_clk_force_on(),
            )
            .field("sys_gmac_rst_clk_tx_n", &self.sys_gmac_rst_clk_tx_n())
            .field("sys_gmac_rst_clk_rx_n", &self.sys_gmac_rst_clk_rx_n())
            .field("sys_gmac_cactive", &self.sys_gmac_cactive())
            .field("sys_gmac_csysack", &self.sys_gmac_csysack())
            .field("sys_gmac_csysreq", &self.sys_gmac_csysreq())
            .field("sys_gmac_apb_postw_en", &self.sys_gmac_apb_postw_en())
            .field("sys_gmac_mem_prdyn", &self.sys_gmac_mem_prdyn())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sys_sbd_flowctrl(&mut self) -> SYS_SBD_FLOWCTRL_W<'_, SYS_GMAC_CTRL0_SPEC> {
        SYS_SBD_FLOWCTRL_W::new(self, 1)
    }
    #[doc = "Bits 2:4"]
    #[inline(always)]
    pub fn sys_phy_intf_sel(&mut self) -> SYS_PHY_INTF_SEL_W<'_, SYS_GMAC_CTRL0_SPEC> {
        SYS_PHY_INTF_SEL_W::new(self, 2)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sys_gmac_mem_clk_force_on(
        &mut self,
    ) -> SYS_GMAC_MEM_CLK_FORCE_ON_W<'_, SYS_GMAC_CTRL0_SPEC> {
        SYS_GMAC_MEM_CLK_FORCE_ON_W::new(self, 5)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn sys_gmac_csysreq(&mut self) -> SYS_GMAC_CSYSREQ_W<'_, SYS_GMAC_CTRL0_SPEC> {
        SYS_GMAC_CSYSREQ_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn sys_gmac_apb_postw_en(&mut self) -> SYS_GMAC_APB_POSTW_EN_W<'_, SYS_GMAC_CTRL0_SPEC> {
        SYS_GMAC_APB_POSTW_EN_W::new(self, 11)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_gmac_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_gmac_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_GMAC_CTRL0_SPEC;
impl crate::RegisterSpec for SYS_GMAC_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_gmac_ctrl0::R`](R) reader structure"]
impl crate::Readable for SYS_GMAC_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_gmac_ctrl0::W`](W) writer structure"]
impl crate::Writable for SYS_GMAC_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_GMAC_CTRL0 to value 0x1000"]
impl crate::Resettable for SYS_GMAC_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x1000;
}
