#[doc = "Register `CTRL0` reader"]
pub type R = crate::R<CTRL0_SPEC>;
#[doc = "Register `CTRL0` writer"]
pub type W = crate::W<CTRL0_SPEC>;
#[doc = "Field `SAR1_TRIGGER_STOP` writer - need_des"]
pub type SAR1_TRIGGER_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_TRIGGER_START` writer - need_des"]
pub type SAR1_TRIGGER_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_TRIGGER_MODE` reader - SAR clock divider"]
pub type SAR1_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `SAR1_TRIGGER_MODE` writer - SAR clock divider"]
pub type SAR1_TRIGGER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR1_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SAR1_PATT_LEN_R = crate::FieldReader;
#[doc = "Field `SAR1_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SAR1_PATT_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR1_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SAR1_PATT_P_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_PATT_TYPE` reader - 1: select meas start as table update 0:select mead done as table update"]
pub type SAR1_PATT_TYPE_R = crate::BitReader;
#[doc = "Field `SAR1_PATT_TYPE` writer - 1: select meas start as table update 0:select mead done as table update"]
pub type SAR1_PATT_TYPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_CLK_POS_SEL` reader - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type SAR1_CLK_POS_SEL_R = crate::BitReader;
#[doc = "Field `SAR1_CLK_POS_SEL` writer - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type SAR1_CLK_POS_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_CONTINUE_MODE_EN` reader - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type SAR1_CONTINUE_MODE_EN_R = crate::BitReader;
#[doc = "Field `SAR1_CONTINUE_MODE_EN` writer - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type SAR1_CONTINUE_MODE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR1_FORCE` reader - force option to xpd sar1 blocks"]
pub type XPD_SAR1_FORCE_R = crate::FieldReader;
#[doc = "Field `XPD_SAR1_FORCE` writer - force option to xpd sar1 blocks"]
pub type XPD_SAR1_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 2:3 - SAR clock divider"]
    #[inline(always)]
    pub fn sar1_trigger_mode(&self) -> SAR1_TRIGGER_MODE_R {
        SAR1_TRIGGER_MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 14:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&self) -> SAR1_PATT_LEN_R {
        SAR1_PATT_LEN_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - 1: select meas start as table update 0:select mead done as table update"]
    #[inline(always)]
    pub fn sar1_patt_type(&self) -> SAR1_PATT_TYPE_R {
        SAR1_PATT_TYPE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn sar1_clk_pos_sel(&self) -> SAR1_CLK_POS_SEL_R {
        SAR1_CLK_POS_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn sar1_continue_mode_en(&self) -> SAR1_CONTINUE_MODE_EN_R {
        SAR1_CONTINUE_MODE_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - force option to xpd sar1 blocks"]
    #[inline(always)]
    pub fn xpd_sar1_force(&self) -> XPD_SAR1_FORCE_R {
        XPD_SAR1_FORCE_R::new(((self.bits >> 26) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL0")
            .field("sar1_trigger_mode", &self.sar1_trigger_mode())
            .field("sar1_patt_len", &self.sar1_patt_len())
            .field("sar1_patt_type", &self.sar1_patt_type())
            .field("sar1_clk_pos_sel", &self.sar1_clk_pos_sel())
            .field("sar1_continue_mode_en", &self.sar1_continue_mode_en())
            .field("xpd_sar1_force", &self.xpd_sar1_force())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sar1_trigger_stop(&mut self) -> SAR1_TRIGGER_STOP_W<'_, CTRL0_SPEC> {
        SAR1_TRIGGER_STOP_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn sar1_trigger_start(&mut self) -> SAR1_TRIGGER_START_W<'_, CTRL0_SPEC> {
        SAR1_TRIGGER_START_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - SAR clock divider"]
    #[inline(always)]
    pub fn sar1_trigger_mode(&mut self) -> SAR1_TRIGGER_MODE_W<'_, CTRL0_SPEC> {
        SAR1_TRIGGER_MODE_W::new(self, 2)
    }
    #[doc = "Bits 14:17 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&mut self) -> SAR1_PATT_LEN_W<'_, CTRL0_SPEC> {
        SAR1_PATT_LEN_W::new(self, 14)
    }
    #[doc = "Bit 22 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn sar1_patt_p_clear(&mut self) -> SAR1_PATT_P_CLEAR_W<'_, CTRL0_SPEC> {
        SAR1_PATT_P_CLEAR_W::new(self, 22)
    }
    #[doc = "Bit 23 - 1: select meas start as table update 0:select mead done as table update"]
    #[inline(always)]
    pub fn sar1_patt_type(&mut self) -> SAR1_PATT_TYPE_W<'_, CTRL0_SPEC> {
        SAR1_PATT_TYPE_W::new(self, 23)
    }
    #[doc = "Bit 24 - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn sar1_clk_pos_sel(&mut self) -> SAR1_CLK_POS_SEL_W<'_, CTRL0_SPEC> {
        SAR1_CLK_POS_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn sar1_continue_mode_en(&mut self) -> SAR1_CONTINUE_MODE_EN_W<'_, CTRL0_SPEC> {
        SAR1_CONTINUE_MODE_EN_W::new(self, 25)
    }
    #[doc = "Bits 26:27 - force option to xpd sar1 blocks"]
    #[inline(always)]
    pub fn xpd_sar1_force(&mut self) -> XPD_SAR1_FORCE_W<'_, CTRL0_SPEC> {
        XPD_SAR1_FORCE_W::new(self, 26)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL0_SPEC;
impl crate::RegisterSpec for CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL0 to value 0x0003_c000"]
impl crate::Resettable for CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0003_c000;
}
