#[doc = "Register `APB_SARADC_CTRL` reader"]
pub type R = crate::R<APB_SARADC_CTRL_SPEC>;
#[doc = "Register `APB_SARADC_CTRL` writer"]
pub type W = crate::W<APB_SARADC_CTRL_SPEC>;
#[doc = "Field `SARADC_START_FORCE` reader - "]
pub type SARADC_START_FORCE_R = crate::BitReader;
#[doc = "Field `SARADC_START_FORCE` writer - "]
pub type SARADC_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_START` reader - "]
pub type SARADC_START_R = crate::BitReader;
#[doc = "Field `SARADC_START` writer - "]
pub type SARADC_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_SAR2_MUX` reader - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
pub type SARADC_SAR2_MUX_R = crate::BitReader;
#[doc = "Field `SARADC_SAR2_MUX` writer - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
pub type SARADC_SAR2_MUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_WORK_MODE` reader - 0: single mode 1: double mode 2: alternate mode"]
pub type SARADC_WORK_MODE_R = crate::FieldReader;
#[doc = "Field `SARADC_WORK_MODE` writer - 0: single mode 1: double mode 2: alternate mode"]
pub type SARADC_WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SARADC_SAR_SEL` reader - 0: SAR1 1: SAR2 only work for single SAR mode"]
pub type SARADC_SAR_SEL_R = crate::BitReader;
#[doc = "Field `SARADC_SAR_SEL` writer - 0: SAR1 1: SAR2 only work for single SAR mode"]
pub type SARADC_SAR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_SAR_CLK_GATED` reader - "]
pub type SARADC_SAR_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SARADC_SAR_CLK_GATED` writer - "]
pub type SARADC_SAR_CLK_GATED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_SAR_CLK_DIV` reader - SAR clock divider"]
pub type SARADC_SAR_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SARADC_SAR_CLK_DIV` writer - SAR clock divider"]
pub type SARADC_SAR_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SARADC_SAR1_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SARADC_SAR1_PATT_LEN_R = crate::FieldReader;
#[doc = "Field `SARADC_SAR1_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SARADC_SAR1_PATT_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SARADC_SAR2_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SARADC_SAR2_PATT_LEN_R = crate::FieldReader;
#[doc = "Field `SARADC_SAR2_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SARADC_SAR2_PATT_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SARADC_SAR1_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SARADC_SAR1_PATT_P_CLEAR_R = crate::BitReader;
#[doc = "Field `SARADC_SAR1_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SARADC_SAR1_PATT_P_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_SAR2_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC2 CTRL"]
pub type SARADC_SAR2_PATT_P_CLEAR_R = crate::BitReader;
#[doc = "Field `SARADC_SAR2_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC2 CTRL"]
pub type SARADC_SAR2_PATT_P_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_DATA_SAR_SEL` reader - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
pub type SARADC_DATA_SAR_SEL_R = crate::BitReader;
#[doc = "Field `SARADC_DATA_SAR_SEL` writer - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
pub type SARADC_DATA_SAR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARADC_DATA_TO_I2S` reader - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
pub type SARADC_DATA_TO_I2S_R = crate::BitReader;
#[doc = "Field `SARADC_DATA_TO_I2S` writer - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
pub type SARADC_DATA_TO_I2S_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn saradc_start_force(&self) -> SARADC_START_FORCE_R {
        SARADC_START_FORCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn saradc_start(&self) -> SARADC_START_R {
        SARADC_START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
    #[inline(always)]
    pub fn saradc_sar2_mux(&self) -> SARADC_SAR2_MUX_R {
        SARADC_SAR2_MUX_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 0: single mode 1: double mode 2: alternate mode"]
    #[inline(always)]
    pub fn saradc_work_mode(&self) -> SARADC_WORK_MODE_R {
        SARADC_WORK_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 0: SAR1 1: SAR2 only work for single SAR mode"]
    #[inline(always)]
    pub fn saradc_sar_sel(&self) -> SARADC_SAR_SEL_R {
        SARADC_SAR_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn saradc_sar_clk_gated(&self) -> SARADC_SAR_CLK_GATED_R {
        SARADC_SAR_CLK_GATED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn saradc_sar_clk_div(&self) -> SARADC_SAR_CLK_DIV_R {
        SARADC_SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar1_patt_len(&self) -> SARADC_SAR1_PATT_LEN_R {
        SARADC_SAR1_PATT_LEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn saradc_sar2_patt_len(&self) -> SARADC_SAR2_PATT_LEN_R {
        SARADC_SAR2_PATT_LEN_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn saradc_sar1_patt_p_clear(&self) -> SARADC_SAR1_PATT_P_CLEAR_R {
        SARADC_SAR1_PATT_P_CLEAR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    pub fn saradc_sar2_patt_p_clear(&self) -> SARADC_SAR2_PATT_P_CLEAR_R {
        SARADC_SAR2_PATT_P_CLEAR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn saradc_data_sar_sel(&self) -> SARADC_DATA_SAR_SEL_R {
        SARADC_DATA_SAR_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn saradc_data_to_i2s(&self) -> SARADC_DATA_TO_I2S_R {
        SARADC_DATA_TO_I2S_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC_CTRL")
            .field("saradc_start_force", &self.saradc_start_force())
            .field("saradc_start", &self.saradc_start())
            .field("saradc_sar2_mux", &self.saradc_sar2_mux())
            .field("saradc_work_mode", &self.saradc_work_mode())
            .field("saradc_sar_sel", &self.saradc_sar_sel())
            .field("saradc_sar_clk_gated", &self.saradc_sar_clk_gated())
            .field("saradc_sar_clk_div", &self.saradc_sar_clk_div())
            .field("saradc_sar1_patt_len", &self.saradc_sar1_patt_len())
            .field("saradc_sar2_patt_len", &self.saradc_sar2_patt_len())
            .field("saradc_sar1_patt_p_clear", &self.saradc_sar1_patt_p_clear())
            .field("saradc_sar2_patt_p_clear", &self.saradc_sar2_patt_p_clear())
            .field("saradc_data_sar_sel", &self.saradc_data_sar_sel())
            .field("saradc_data_to_i2s", &self.saradc_data_to_i2s())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_start_force(&mut self) -> SARADC_START_FORCE_W<APB_SARADC_CTRL_SPEC> {
        SARADC_START_FORCE_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_start(&mut self) -> SARADC_START_W<APB_SARADC_CTRL_SPEC> {
        SARADC_START_W::new(self, 1)
    }
    #[doc = "Bit 2 - 1: SAR ADC2 is controlled by DIG ADC2 CTRL 0: SAR ADC2 is controlled by PWDET CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar2_mux(&mut self) -> SARADC_SAR2_MUX_W<APB_SARADC_CTRL_SPEC> {
        SARADC_SAR2_MUX_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - 0: single mode 1: double mode 2: alternate mode"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_work_mode(&mut self) -> SARADC_WORK_MODE_W<APB_SARADC_CTRL_SPEC> {
        SARADC_WORK_MODE_W::new(self, 3)
    }
    #[doc = "Bit 5 - 0: SAR1 1: SAR2 only work for single SAR mode"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_sel(&mut self) -> SARADC_SAR_SEL_W<APB_SARADC_CTRL_SPEC> {
        SARADC_SAR_SEL_W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_clk_gated(&mut self) -> SARADC_SAR_CLK_GATED_W<APB_SARADC_CTRL_SPEC> {
        SARADC_SAR_CLK_GATED_W::new(self, 6)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_clk_div(&mut self) -> SARADC_SAR_CLK_DIV_W<APB_SARADC_CTRL_SPEC> {
        SARADC_SAR_CLK_DIV_W::new(self, 7)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar1_patt_len(&mut self) -> SARADC_SAR1_PATT_LEN_W<APB_SARADC_CTRL_SPEC> {
        SARADC_SAR1_PATT_LEN_W::new(self, 15)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar2_patt_len(&mut self) -> SARADC_SAR2_PATT_LEN_W<APB_SARADC_CTRL_SPEC> {
        SARADC_SAR2_PATT_LEN_W::new(self, 19)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar1_patt_p_clear(&mut self) -> SARADC_SAR1_PATT_P_CLEAR_W<APB_SARADC_CTRL_SPEC> {
        SARADC_SAR1_PATT_P_CLEAR_W::new(self, 23)
    }
    #[doc = "Bit 24 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar2_patt_p_clear(&mut self) -> SARADC_SAR2_PATT_P_CLEAR_W<APB_SARADC_CTRL_SPEC> {
        SARADC_SAR2_PATT_P_CLEAR_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded by the MSB of the 16-bit output data in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    #[must_use]
    pub fn saradc_data_sar_sel(&mut self) -> SARADC_DATA_SAR_SEL_W<APB_SARADC_CTRL_SPEC> {
        SARADC_DATA_SAR_SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA) 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_data_to_i2s(&mut self) -> SARADC_DATA_TO_I2S_W<APB_SARADC_CTRL_SPEC> {
        SARADC_DATA_TO_I2S_W::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_SARADC_CTRL_SPEC;
impl crate::RegisterSpec for APB_SARADC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_saradc_ctrl::R`](R) reader structure"]
impl crate::Readable for APB_SARADC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_saradc_ctrl::W`](W) writer structure"]
impl crate::Writable for APB_SARADC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_SARADC_CTRL to value 0x007f_8240"]
impl crate::Resettable for APB_SARADC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x007f_8240;
}
