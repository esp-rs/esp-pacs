#[doc = "Register `CTRL1` reader"]
pub type R = crate::R<CTRL1_SPEC>;
#[doc = "Register `CTRL1` writer"]
pub type W = crate::W<CTRL1_SPEC>;
#[doc = "Field `SAR2_TRIGGER_STOP` writer - need_des"]
pub type SAR2_TRIGGER_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_TRIGGER_START` writer - need_des"]
pub type SAR2_TRIGGER_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_TRIGGER_MODE` reader - SAR clock divider"]
pub type SAR2_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `SAR2_TRIGGER_MODE` writer - SAR clock divider"]
pub type SAR2_TRIGGER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR2_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SAR2_PATT_LEN_R = crate::FieldReader;
#[doc = "Field `SAR2_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SAR2_PATT_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR2_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SAR2_PATT_P_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_PATT_TYPE` reader - 1: select meas start as table update 0:select mead done as table update"]
pub type SAR2_PATT_TYPE_R = crate::BitReader;
#[doc = "Field `SAR2_PATT_TYPE` writer - 1: select meas start as table update 0:select mead done as table update"]
pub type SAR2_PATT_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_CLK_POS_SEL` reader - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type SAR2_CLK_POS_SEL_R = crate::BitReader;
#[doc = "Field `SAR2_CLK_POS_SEL` writer - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type SAR2_CLK_POS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_CONTINUE_MODE_EN` reader - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type SAR2_CONTINUE_MODE_EN_R = crate::BitReader;
#[doc = "Field `SAR2_CONTINUE_MODE_EN` writer - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type SAR2_CONTINUE_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR2_FORCE` reader - force option to xpd sar1 blocks"]
pub type XPD_SAR2_FORCE_R = crate::FieldReader;
#[doc = "Field `XPD_SAR2_FORCE` writer - force option to xpd sar1 blocks"]
pub type XPD_SAR2_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - SAR clock divider"]
    #[inline(always)]
    pub fn sar2_trigger_mode(&self) -> SAR2_TRIGGER_MODE_R {
        SAR2_TRIGGER_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 14:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&self) -> SAR2_PATT_LEN_R {
        SAR2_PATT_LEN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 1: select meas start as table update 0:select mead done as table update"]
    #[inline(always)]
    pub fn sar2_patt_type(&self) -> SAR2_PATT_TYPE_R {
        SAR2_PATT_TYPE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn sar2_clk_pos_sel(&self) -> SAR2_CLK_POS_SEL_R {
        SAR2_CLK_POS_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn sar2_continue_mode_en(&self) -> SAR2_CONTINUE_MODE_EN_R {
        SAR2_CONTINUE_MODE_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - force option to xpd sar1 blocks"]
    #[inline(always)]
    pub fn xpd_sar2_force(&self) -> XPD_SAR2_FORCE_R {
        XPD_SAR2_FORCE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL1")
            .field("sar2_trigger_mode", &self.sar2_trigger_mode())
            .field("sar2_patt_len", &self.sar2_patt_len())
            .field("sar2_patt_type", &self.sar2_patt_type())
            .field("sar2_clk_pos_sel", &self.sar2_clk_pos_sel())
            .field("sar2_continue_mode_en", &self.sar2_continue_mode_en())
            .field("xpd_sar2_force", &self.xpd_sar2_force())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sar2_trigger_stop(&mut self) -> SAR2_TRIGGER_STOP_W<'_, CTRL1_SPEC> {
        SAR2_TRIGGER_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn sar2_trigger_start(&mut self) -> SAR2_TRIGGER_START_W<'_, CTRL1_SPEC> {
        SAR2_TRIGGER_START_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - SAR clock divider"]
    #[inline(always)]
    pub fn sar2_trigger_mode(&mut self) -> SAR2_TRIGGER_MODE_W<'_, CTRL1_SPEC> {
        SAR2_TRIGGER_MODE_W::new(self, 2)
    }
    #[doc = "Bits 14:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&mut self) -> SAR2_PATT_LEN_W<'_, CTRL1_SPEC> {
        SAR2_PATT_LEN_W::new(self, 14)
    }
    #[doc = "Bit 22 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn sar2_patt_p_clear(&mut self) -> SAR2_PATT_P_CLEAR_W<'_, CTRL1_SPEC> {
        SAR2_PATT_P_CLEAR_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: select meas start as table update 0:select mead done as table update"]
    #[inline(always)]
    pub fn sar2_patt_type(&mut self) -> SAR2_PATT_TYPE_W<'_, CTRL1_SPEC> {
        SAR2_PATT_TYPE_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn sar2_clk_pos_sel(&mut self) -> SAR2_CLK_POS_SEL_W<'_, CTRL1_SPEC> {
        SAR2_CLK_POS_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn sar2_continue_mode_en(&mut self) -> SAR2_CONTINUE_MODE_EN_W<'_, CTRL1_SPEC> {
        SAR2_CONTINUE_MODE_EN_W::new(self, 25)
    }
    #[doc = "Bits 26:27 - force option to xpd sar1 blocks"]
    #[inline(always)]
    pub fn xpd_sar2_force(&mut self) -> XPD_SAR2_FORCE_W<'_, CTRL1_SPEC> {
        XPD_SAR2_FORCE_W::new(self, 26)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL1 to value 0x0003_c000"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: u32 = 0x0003_c000;
}
