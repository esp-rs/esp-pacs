#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `START_FORCE` reader - enable start saradc by sw"]
pub type START_FORCE_R = crate::BitReader;
#[doc = "Field `START_FORCE` writer - enable start saradc by sw"]
pub type START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START` reader - start saradc by sw"]
pub type START_R = crate::BitReader;
#[doc = "Field `START` writer - start saradc by sw"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WORK_MODE` reader - 0: single mode, 1: double mode, 2: alternate mode"]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - 0: single mode, 1: double mode, 2: alternate mode"]
pub type WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR_SEL` reader - 0: SAR1, 1: SAR2, only work for single SAR mode"]
pub type SAR_SEL_R = crate::BitReader;
#[doc = "Field `SAR_SEL` writer - 0: SAR1, 1: SAR2, only work for single SAR mode"]
pub type SAR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_CLK_GATED` reader - enable SAR CLK gate when saradc idle"]
pub type SAR_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SAR_CLK_GATED` writer - enable SAR CLK gate when saradc idle"]
pub type SAR_CLK_GATED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_CLK_DIV` reader - SAR clock divider"]
pub type SAR_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SAR_CLK_DIV` writer - SAR clock divider"]
pub type SAR_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SAR1_PATT_LEN_R = crate::FieldReader;
#[doc = "Field `SAR1_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SAR1_PATT_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR2_PATT_LEN` reader - 0 ~ 15 means length 1 ~ 16"]
pub type SAR2_PATT_LEN_R = crate::FieldReader;
#[doc = "Field `SAR2_PATT_LEN` writer - 0 ~ 15 means length 1 ~ 16"]
pub type SAR2_PATT_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SAR1_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SAR1_PATT_P_CLEAR_R = crate::BitReader;
#[doc = "Field `SAR1_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC1 CTRL"]
pub type SAR1_PATT_P_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_PATT_P_CLEAR` reader - clear the pointer of pattern table for DIG ADC2 CTRL"]
pub type SAR2_PATT_P_CLEAR_R = crate::BitReader;
#[doc = "Field `SAR2_PATT_P_CLEAR` writer - clear the pointer of pattern table for DIG ADC2 CTRL"]
pub type SAR2_PATT_P_CLEAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_SAR_SEL` reader - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type DATA_SAR_SEL_R = crate::BitReader;
#[doc = "Field `DATA_SAR_SEL` writer - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
pub type DATA_SAR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TO_I2S` reader - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type DATA_TO_I2S_R = crate::BitReader;
#[doc = "Field `DATA_TO_I2S` writer - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
pub type DATA_TO_I2S_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_SAR_FORCE` reader - force option to xpd sar blocks"]
pub type XPD_SAR_FORCE_R = crate::FieldReader;
#[doc = "Field `XPD_SAR_FORCE` writer - force option to xpd sar blocks"]
pub type XPD_SAR_FORCE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WAIT_ARB_CYCLE` reader - wait arbit signal stable after sar_done"]
pub type WAIT_ARB_CYCLE_R = crate::FieldReader;
#[doc = "Field `WAIT_ARB_CYCLE` writer - wait arbit signal stable after sar_done"]
pub type WAIT_ARB_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - enable start saradc by sw"]
    #[inline(always)]
    pub fn start_force(&self) -> START_FORCE_R {
        START_FORCE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - start saradc by sw"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - 0: single mode, 1: double mode, 2: alternate mode"]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - 0: SAR1, 1: SAR2, only work for single SAR mode"]
    #[inline(always)]
    pub fn sar_sel(&self) -> SAR_SEL_R {
        SAR_SEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - enable SAR CLK gate when saradc idle"]
    #[inline(always)]
    pub fn sar_clk_gated(&self) -> SAR_CLK_GATED_R {
        SAR_CLK_GATED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    pub fn sar_clk_div(&self) -> SAR_CLK_DIV_R {
        SAR_CLK_DIV_R::new(((self.bits >> 7) & 0xff) as u8)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar1_patt_len(&self) -> SAR1_PATT_LEN_R {
        SAR1_PATT_LEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    pub fn sar2_patt_len(&self) -> SAR2_PATT_LEN_R {
        SAR2_PATT_LEN_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    pub fn sar1_patt_p_clear(&self) -> SAR1_PATT_P_CLEAR_R {
        SAR1_PATT_P_CLEAR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_patt_p_clear(&self) -> SAR2_PATT_P_CLEAR_R {
        SAR2_PATT_P_CLEAR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    pub fn data_sar_sel(&self) -> DATA_SAR_SEL_R {
        DATA_SAR_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    pub fn data_to_i2s(&self) -> DATA_TO_I2S_R {
        DATA_TO_I2S_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - force option to xpd sar blocks"]
    #[inline(always)]
    pub fn xpd_sar_force(&self) -> XPD_SAR_FORCE_R {
        XPD_SAR_FORCE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    pub fn wait_arb_cycle(&self) -> WAIT_ARB_CYCLE_R {
        WAIT_ARB_CYCLE_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("start_force", &self.start_force())
            .field("start", &self.start())
            .field("work_mode", &self.work_mode())
            .field("sar_sel", &self.sar_sel())
            .field("sar_clk_gated", &self.sar_clk_gated())
            .field("sar_clk_div", &self.sar_clk_div())
            .field("sar1_patt_len", &self.sar1_patt_len())
            .field("sar2_patt_len", &self.sar2_patt_len())
            .field("sar1_patt_p_clear", &self.sar1_patt_p_clear())
            .field("sar2_patt_p_clear", &self.sar2_patt_p_clear())
            .field("data_sar_sel", &self.data_sar_sel())
            .field("data_to_i2s", &self.data_to_i2s())
            .field("xpd_sar_force", &self.xpd_sar_force())
            .field("wait_arb_cycle", &self.wait_arb_cycle())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - enable start saradc by sw"]
    #[inline(always)]
    #[must_use]
    pub fn start_force(&mut self) -> START_FORCE_W<CTRL_SPEC> {
        START_FORCE_W::new(self, 0)
    }
    #[doc = "Bit 1 - start saradc by sw"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CTRL_SPEC> {
        START_W::new(self, 1)
    }
    #[doc = "Bits 3:4 - 0: single mode, 1: double mode, 2: alternate mode"]
    #[inline(always)]
    #[must_use]
    pub fn work_mode(&mut self) -> WORK_MODE_W<CTRL_SPEC> {
        WORK_MODE_W::new(self, 3)
    }
    #[doc = "Bit 5 - 0: SAR1, 1: SAR2, only work for single SAR mode"]
    #[inline(always)]
    #[must_use]
    pub fn sar_sel(&mut self) -> SAR_SEL_W<CTRL_SPEC> {
        SAR_SEL_W::new(self, 5)
    }
    #[doc = "Bit 6 - enable SAR CLK gate when saradc idle"]
    #[inline(always)]
    #[must_use]
    pub fn sar_clk_gated(&mut self) -> SAR_CLK_GATED_W<CTRL_SPEC> {
        SAR_CLK_GATED_W::new(self, 6)
    }
    #[doc = "Bits 7:14 - SAR clock divider"]
    #[inline(always)]
    #[must_use]
    pub fn sar_clk_div(&mut self) -> SAR_CLK_DIV_W<CTRL_SPEC> {
        SAR_CLK_DIV_W::new(self, 7)
    }
    #[doc = "Bits 15:18 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_patt_len(&mut self) -> SAR1_PATT_LEN_W<CTRL_SPEC> {
        SAR1_PATT_LEN_W::new(self, 15)
    }
    #[doc = "Bits 19:22 - 0 ~ 15 means length 1 ~ 16"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_patt_len(&mut self) -> SAR2_PATT_LEN_W<CTRL_SPEC> {
        SAR2_PATT_LEN_W::new(self, 19)
    }
    #[doc = "Bit 23 - clear the pointer of pattern table for DIG ADC1 CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_patt_p_clear(&mut self) -> SAR1_PATT_P_CLEAR_W<CTRL_SPEC> {
        SAR1_PATT_P_CLEAR_W::new(self, 23)
    }
    #[doc = "Bit 24 - clear the pointer of pattern table for DIG ADC2 CTRL"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_patt_p_clear(&mut self) -> SAR2_PATT_P_CLEAR_W<CTRL_SPEC> {
        SAR2_PATT_P_CLEAR_W::new(self, 24)
    }
    #[doc = "Bit 25 - 1: sar_sel will be coded by the MSB of the 16-bit output data, in this case the resolution should not be larger than 11 bits."]
    #[inline(always)]
    #[must_use]
    pub fn data_sar_sel(&mut self) -> DATA_SAR_SEL_W<CTRL_SPEC> {
        DATA_SAR_SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - 1: I2S input data is from SAR ADC (for DMA), 0: I2S input data is from GPIO matrix"]
    #[inline(always)]
    #[must_use]
    pub fn data_to_i2s(&mut self) -> DATA_TO_I2S_W<CTRL_SPEC> {
        DATA_TO_I2S_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - force option to xpd sar blocks"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_sar_force(&mut self) -> XPD_SAR_FORCE_W<CTRL_SPEC> {
        XPD_SAR_FORCE_W::new(self, 27)
    }
    #[doc = "Bits 30:31 - wait arbit signal stable after sar_done"]
    #[inline(always)]
    #[must_use]
    pub fn wait_arb_cycle(&mut self) -> WAIT_ARB_CYCLE_W<CTRL_SPEC> {
        WAIT_ARB_CYCLE_W::new(self, 30)
    }
}
#[doc = "configure apb saradc controller\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x407f_8240"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0x407f_8240;
}
