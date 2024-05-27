#[doc = "Register `LP_AONCLKRST_HP_USB_CLKRST_CTRL0` reader"]
pub type R = crate::R<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC>;
#[doc = "Register `LP_AONCLKRST_HP_USB_CLKRST_CTRL0` writer"]
pub type W = crate::W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC>;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_SLEEP_MODE` reader - unused."]
pub type LP_AONCLKRST_USB_OTG20_SLEEP_MODE_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_SLEEP_MODE` writer - unused."]
pub type LP_AONCLKRST_USB_OTG20_SLEEP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN` reader - unused."]
pub type LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN` writer - unused."]
pub type LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_SLEEP_MODE` reader - unused."]
pub type LP_AONCLKRST_USB_OTG11_SLEEP_MODE_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_SLEEP_MODE` writer - unused."]
pub type LP_AONCLKRST_USB_OTG11_SLEEP_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN` reader - unused."]
pub type LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN` writer - unused."]
pub type LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_48M_CLK_EN` reader - usb otg11 fs phy clock enable."]
pub type LP_AONCLKRST_USB_OTG11_48M_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_OTG11_48M_CLK_EN` writer - usb otg11 fs phy clock enable."]
pub type LP_AONCLKRST_USB_OTG11_48M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_DEVICE_48M_CLK_EN` reader - usb device fs phy clock enable."]
pub type LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_USB_DEVICE_48M_CLK_EN` writer - usb device fs phy clock enable."]
pub type LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_USB_48M_DIV_NUM` reader - usb 480m to 25m divide number."]
pub type LP_AONCLKRST_USB_48M_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_USB_48M_DIV_NUM` writer - usb 480m to 25m divide number."]
pub type LP_AONCLKRST_USB_48M_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AONCLKRST_USB_25M_DIV_NUM` reader - usb 500m to 25m divide number."]
pub type LP_AONCLKRST_USB_25M_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_USB_25M_DIV_NUM` writer - usb 500m to 25m divide number."]
pub type LP_AONCLKRST_USB_25M_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AONCLKRST_USB_12M_DIV_NUM` reader - usb 480m to 12m divide number."]
pub type LP_AONCLKRST_USB_12M_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_USB_12M_DIV_NUM` writer - usb 480m to 12m divide number."]
pub type LP_AONCLKRST_USB_12M_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_sleep_mode(&self) -> LP_AONCLKRST_USB_OTG20_SLEEP_MODE_R {
        LP_AONCLKRST_USB_OTG20_SLEEP_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg20_bk_sys_clk_en(&self) -> LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_R {
        LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_sleep_mode(&self) -> LP_AONCLKRST_USB_OTG11_SLEEP_MODE_R {
        LP_AONCLKRST_USB_OTG11_SLEEP_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - unused."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_bk_sys_clk_en(&self) -> LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_R {
        LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - usb otg11 fs phy clock enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_otg11_48m_clk_en(&self) -> LP_AONCLKRST_USB_OTG11_48M_CLK_EN_R {
        LP_AONCLKRST_USB_OTG11_48M_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - usb device fs phy clock enable."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_device_48m_clk_en(&self) -> LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_R {
        LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:13 - usb 480m to 25m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_48m_div_num(&self) -> LP_AONCLKRST_USB_48M_DIV_NUM_R {
        LP_AONCLKRST_USB_48M_DIV_NUM_R::new(((self.bits >> 6) & 0xff) as u8)
    }
    #[doc = "Bits 14:21 - usb 500m to 25m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_25m_div_num(&self) -> LP_AONCLKRST_USB_25M_DIV_NUM_R {
        LP_AONCLKRST_USB_25M_DIV_NUM_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    #[doc = "Bits 22:29 - usb 480m to 12m divide number."]
    #[inline(always)]
    pub fn lp_aonclkrst_usb_12m_div_num(&self) -> LP_AONCLKRST_USB_12M_DIV_NUM_R {
        LP_AONCLKRST_USB_12M_DIV_NUM_R::new(((self.bits >> 22) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HP_USB_CLKRST_CTRL0")
            .field(
                "lp_aonclkrst_usb_otg20_sleep_mode",
                &self.lp_aonclkrst_usb_otg20_sleep_mode(),
            )
            .field(
                "lp_aonclkrst_usb_otg20_bk_sys_clk_en",
                &self.lp_aonclkrst_usb_otg20_bk_sys_clk_en(),
            )
            .field(
                "lp_aonclkrst_usb_otg11_sleep_mode",
                &self.lp_aonclkrst_usb_otg11_sleep_mode(),
            )
            .field(
                "lp_aonclkrst_usb_otg11_bk_sys_clk_en",
                &self.lp_aonclkrst_usb_otg11_bk_sys_clk_en(),
            )
            .field(
                "lp_aonclkrst_usb_otg11_48m_clk_en",
                &self.lp_aonclkrst_usb_otg11_48m_clk_en(),
            )
            .field(
                "lp_aonclkrst_usb_device_48m_clk_en",
                &self.lp_aonclkrst_usb_device_48m_clk_en(),
            )
            .field(
                "lp_aonclkrst_usb_48m_div_num",
                &self.lp_aonclkrst_usb_48m_div_num(),
            )
            .field(
                "lp_aonclkrst_usb_25m_div_num",
                &self.lp_aonclkrst_usb_25m_div_num(),
            )
            .field(
                "lp_aonclkrst_usb_12m_div_num",
                &self.lp_aonclkrst_usb_12m_div_num(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - unused."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_otg20_sleep_mode(
        &mut self,
    ) -> LP_AONCLKRST_USB_OTG20_SLEEP_MODE_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_OTG20_SLEEP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - unused."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_otg20_bk_sys_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_OTG20_BK_SYS_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - unused."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_otg11_sleep_mode(
        &mut self,
    ) -> LP_AONCLKRST_USB_OTG11_SLEEP_MODE_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_OTG11_SLEEP_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - unused."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_otg11_bk_sys_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_OTG11_BK_SYS_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - usb otg11 fs phy clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_otg11_48m_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_USB_OTG11_48M_CLK_EN_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_OTG11_48M_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - usb device fs phy clock enable."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_device_48m_clk_en(
        &mut self,
    ) -> LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_DEVICE_48M_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bits 6:13 - usb 480m to 25m divide number."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_48m_div_num(
        &mut self,
    ) -> LP_AONCLKRST_USB_48M_DIV_NUM_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_48M_DIV_NUM_W::new(self, 6)
    }
    #[doc = "Bits 14:21 - usb 500m to 25m divide number."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_25m_div_num(
        &mut self,
    ) -> LP_AONCLKRST_USB_25M_DIV_NUM_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_25M_DIV_NUM_W::new(self, 14)
    }
    #[doc = "Bits 22:29 - usb 480m to 12m divide number."]
    #[inline(always)]
    #[must_use]
    pub fn lp_aonclkrst_usb_12m_div_num(
        &mut self,
    ) -> LP_AONCLKRST_USB_12M_DIV_NUM_W<LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC> {
        LP_AONCLKRST_USB_12M_DIV_NUM_W::new(self, 22)
    }
}
#[doc = "HP USB Clock Reset Control Register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_aonclkrst_hp_usb_clkrst_ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_aonclkrst_hp_usb_clkrst_ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hp_usb_clkrst_ctrl0::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hp_usb_clkrst_ctrl0::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HP_USB_CLKRST_CTRL0 to value 0x09c4_c27a"]
impl crate::Resettable for LP_AONCLKRST_HP_USB_CLKRST_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x09c4_c27a;
}
