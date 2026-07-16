#[doc = "Register `SYS_USB_OTG20_CTRL` reader"]
pub type R = crate::R<SYS_USB_OTG20_CTRL_SPEC>;
#[doc = "Register `SYS_USB_OTG20_CTRL` writer"]
pub type W = crate::W<SYS_USB_OTG20_CTRL_SPEC>;
#[doc = "Field `SYS_USB_OTG20_UTMIFS_CLK_EN` reader - "]
pub type SYS_USB_OTG20_UTMIFS_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_USB_OTG20_UTMIFS_CLK_EN` writer - "]
pub type SYS_USB_OTG20_UTMIFS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_USB_OTG20_ULPI_CLK_EN` reader - "]
pub type SYS_USB_OTG20_ULPI_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_USB_OTG20_ULPI_CLK_EN` writer - "]
pub type SYS_USB_OTG20_ULPI_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_USB_OTG20_PHYREF_CLK_SRC_SEL` reader - "]
pub type SYS_USB_OTG20_PHYREF_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `SYS_USB_OTG20_PHYREF_CLK_SRC_SEL` writer - "]
pub type SYS_USB_OTG20_PHYREF_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYS_USB_OTG20_PHYREF_CLK_EN` reader - "]
pub type SYS_USB_OTG20_PHYREF_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_USB_OTG20_PHYREF_CLK_EN` writer - "]
pub type SYS_USB_OTG20_PHYREF_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_USB_OTG20_PHY_RST_EN` reader - "]
pub type SYS_USB_OTG20_PHY_RST_EN_R = crate::BitReader;
#[doc = "Field `SYS_USB_OTG20_PHY_RST_EN` writer - "]
pub type SYS_USB_OTG20_PHY_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_USB_OTG20_AHB_RST_EN` reader - "]
pub type SYS_USB_OTG20_AHB_RST_EN_R = crate::BitReader;
#[doc = "Field `SYS_USB_OTG20_AHB_RST_EN` writer - "]
pub type SYS_USB_OTG20_AHB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_USB_OTG20_APB_RST_EN` reader - "]
pub type SYS_USB_OTG20_APB_RST_EN_R = crate::BitReader;
#[doc = "Field `SYS_USB_OTG20_APB_RST_EN` writer - "]
pub type SYS_USB_OTG20_APB_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sys_usb_otg20_utmifs_clk_en(&self) -> SYS_USB_OTG20_UTMIFS_CLK_EN_R {
        SYS_USB_OTG20_UTMIFS_CLK_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sys_usb_otg20_ulpi_clk_en(&self) -> SYS_USB_OTG20_ULPI_CLK_EN_R {
        SYS_USB_OTG20_ULPI_CLK_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn sys_usb_otg20_phyref_clk_src_sel(&self) -> SYS_USB_OTG20_PHYREF_CLK_SRC_SEL_R {
        SYS_USB_OTG20_PHYREF_CLK_SRC_SEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sys_usb_otg20_phyref_clk_en(&self) -> SYS_USB_OTG20_PHYREF_CLK_EN_R {
        SYS_USB_OTG20_PHYREF_CLK_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sys_usb_otg20_phy_rst_en(&self) -> SYS_USB_OTG20_PHY_RST_EN_R {
        SYS_USB_OTG20_PHY_RST_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_usb_otg20_ahb_rst_en(&self) -> SYS_USB_OTG20_AHB_RST_EN_R {
        SYS_USB_OTG20_AHB_RST_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_usb_otg20_apb_rst_en(&self) -> SYS_USB_OTG20_APB_RST_EN_R {
        SYS_USB_OTG20_APB_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_USB_OTG20_CTRL")
            .field(
                "sys_usb_otg20_utmifs_clk_en",
                &self.sys_usb_otg20_utmifs_clk_en(),
            )
            .field(
                "sys_usb_otg20_ulpi_clk_en",
                &self.sys_usb_otg20_ulpi_clk_en(),
            )
            .field(
                "sys_usb_otg20_phyref_clk_src_sel",
                &self.sys_usb_otg20_phyref_clk_src_sel(),
            )
            .field(
                "sys_usb_otg20_phyref_clk_en",
                &self.sys_usb_otg20_phyref_clk_en(),
            )
            .field("sys_usb_otg20_phy_rst_en", &self.sys_usb_otg20_phy_rst_en())
            .field("sys_usb_otg20_ahb_rst_en", &self.sys_usb_otg20_ahb_rst_en())
            .field("sys_usb_otg20_apb_rst_en", &self.sys_usb_otg20_apb_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn sys_usb_otg20_utmifs_clk_en(
        &mut self,
    ) -> SYS_USB_OTG20_UTMIFS_CLK_EN_W<'_, SYS_USB_OTG20_CTRL_SPEC> {
        SYS_USB_OTG20_UTMIFS_CLK_EN_W::new(self, 23)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn sys_usb_otg20_ulpi_clk_en(
        &mut self,
    ) -> SYS_USB_OTG20_ULPI_CLK_EN_W<'_, SYS_USB_OTG20_CTRL_SPEC> {
        SYS_USB_OTG20_ULPI_CLK_EN_W::new(self, 24)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn sys_usb_otg20_phyref_clk_src_sel(
        &mut self,
    ) -> SYS_USB_OTG20_PHYREF_CLK_SRC_SEL_W<'_, SYS_USB_OTG20_CTRL_SPEC> {
        SYS_USB_OTG20_PHYREF_CLK_SRC_SEL_W::new(self, 25)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sys_usb_otg20_phyref_clk_en(
        &mut self,
    ) -> SYS_USB_OTG20_PHYREF_CLK_EN_W<'_, SYS_USB_OTG20_CTRL_SPEC> {
        SYS_USB_OTG20_PHYREF_CLK_EN_W::new(self, 27)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn sys_usb_otg20_phy_rst_en(
        &mut self,
    ) -> SYS_USB_OTG20_PHY_RST_EN_W<'_, SYS_USB_OTG20_CTRL_SPEC> {
        SYS_USB_OTG20_PHY_RST_EN_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_usb_otg20_ahb_rst_en(
        &mut self,
    ) -> SYS_USB_OTG20_AHB_RST_EN_W<'_, SYS_USB_OTG20_CTRL_SPEC> {
        SYS_USB_OTG20_AHB_RST_EN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_usb_otg20_apb_rst_en(
        &mut self,
    ) -> SYS_USB_OTG20_APB_RST_EN_W<'_, SYS_USB_OTG20_CTRL_SPEC> {
        SYS_USB_OTG20_APB_RST_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_usb_otg20_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_usb_otg20_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_USB_OTG20_CTRL_SPEC;
impl crate::RegisterSpec for SYS_USB_OTG20_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_usb_otg20_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_USB_OTG20_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_usb_otg20_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_USB_OTG20_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_USB_OTG20_CTRL to value 0xe080_0000"]
impl crate::Resettable for SYS_USB_OTG20_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xe080_0000;
}
