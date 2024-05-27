///Register `SAR_DAC_CTRL1` reader
pub type R = crate::R<SAR_DAC_CTRL1_SPEC>;
///Register `SAR_DAC_CTRL1` writer
pub type W = crate::W<SAR_DAC_CTRL1_SPEC>;
///Field `SW_FSTEP` reader - Frequency step for CW generator can be used to adjust the frequency.
pub type SW_FSTEP_R = crate::FieldReader<u16>;
///Field `SW_FSTEP` writer - Frequency step for CW generator can be used to adjust the frequency.
pub type SW_FSTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `SW_TONE_EN` reader - 0: disable CW generator. 1: enable CW generator.
pub type SW_TONE_EN_R = crate::BitReader;
///Field `SW_TONE_EN` writer - 0: disable CW generator. 1: enable CW generator.
pub type SW_TONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEBUG_BIT_SEL` reader -
pub type DEBUG_BIT_SEL_R = crate::FieldReader;
///Field `DEBUG_BIT_SEL` writer -
pub type DEBUG_BIT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `DAC_DIG_FORCE` reader - 0: DAC1 and DAC2 do not use DMA. 1: DAC1 and DAC2 use DMA.
pub type DAC_DIG_FORCE_R = crate::BitReader;
///Field `DAC_DIG_FORCE` writer - 0: DAC1 and DAC2 do not use DMA. 1: DAC1 and DAC2 use DMA.
pub type DAC_DIG_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CLK_FORCE_LOW` reader - 1: force PDAC_CLK to low
pub type DAC_CLK_FORCE_LOW_R = crate::BitReader;
///Field `DAC_CLK_FORCE_LOW` writer - 1: force PDAC_CLK to low
pub type DAC_CLK_FORCE_LOW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CLK_FORCE_HIGH` reader - 1: force PDAC_CLK to high
pub type DAC_CLK_FORCE_HIGH_R = crate::BitReader;
///Field `DAC_CLK_FORCE_HIGH` writer - 1: force PDAC_CLK to high
pub type DAC_CLK_FORCE_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CLK_INV` reader - 1: invert PDAC_CLK.
pub type DAC_CLK_INV_R = crate::BitReader;
///Field `DAC_CLK_INV` writer - 1: invert PDAC_CLK.
pub type DAC_CLK_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_RESET` reader - Reset DAC by software.
pub type DAC_RESET_R = crate::BitReader;
///Field `DAC_RESET` writer - Reset DAC by software.
pub type DAC_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CLKGATE_EN` reader - DAC clock gate enable bit.
pub type DAC_CLKGATE_EN_R = crate::BitReader;
///Field `DAC_CLKGATE_EN` writer - DAC clock gate enable bit.
pub type DAC_CLKGATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:15 - Frequency step for CW generator can be used to adjust the frequency.
    #[inline(always)]
    pub fn sw_fstep(&self) -> SW_FSTEP_R {
        SW_FSTEP_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 16 - 0: disable CW generator. 1: enable CW generator.
    #[inline(always)]
    pub fn sw_tone_en(&self) -> SW_TONE_EN_R {
        SW_TONE_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:21
    #[inline(always)]
    pub fn debug_bit_sel(&self) -> DEBUG_BIT_SEL_R {
        DEBUG_BIT_SEL_R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    ///Bit 22 - 0: DAC1 and DAC2 do not use DMA. 1: DAC1 and DAC2 use DMA.
    #[inline(always)]
    pub fn dac_dig_force(&self) -> DAC_DIG_FORCE_R {
        DAC_DIG_FORCE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - 1: force PDAC_CLK to low
    #[inline(always)]
    pub fn dac_clk_force_low(&self) -> DAC_CLK_FORCE_LOW_R {
        DAC_CLK_FORCE_LOW_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - 1: force PDAC_CLK to high
    #[inline(always)]
    pub fn dac_clk_force_high(&self) -> DAC_CLK_FORCE_HIGH_R {
        DAC_CLK_FORCE_HIGH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - 1: invert PDAC_CLK.
    #[inline(always)]
    pub fn dac_clk_inv(&self) -> DAC_CLK_INV_R {
        DAC_CLK_INV_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Reset DAC by software.
    #[inline(always)]
    pub fn dac_reset(&self) -> DAC_RESET_R {
        DAC_RESET_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DAC clock gate enable bit.
    #[inline(always)]
    pub fn dac_clkgate_en(&self) -> DAC_CLKGATE_EN_R {
        DAC_CLKGATE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_DAC_CTRL1")
            .field("sw_fstep", &self.sw_fstep())
            .field("sw_tone_en", &self.sw_tone_en())
            .field("debug_bit_sel", &self.debug_bit_sel())
            .field("dac_dig_force", &self.dac_dig_force())
            .field("dac_clk_force_low", &self.dac_clk_force_low())
            .field("dac_clk_force_high", &self.dac_clk_force_high())
            .field("dac_clk_inv", &self.dac_clk_inv())
            .field("dac_reset", &self.dac_reset())
            .field("dac_clkgate_en", &self.dac_clkgate_en())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Frequency step for CW generator can be used to adjust the frequency.
    #[inline(always)]
    #[must_use]
    pub fn sw_fstep(&mut self) -> SW_FSTEP_W<SAR_DAC_CTRL1_SPEC> {
        SW_FSTEP_W::new(self, 0)
    }
    ///Bit 16 - 0: disable CW generator. 1: enable CW generator.
    #[inline(always)]
    #[must_use]
    pub fn sw_tone_en(&mut self) -> SW_TONE_EN_W<SAR_DAC_CTRL1_SPEC> {
        SW_TONE_EN_W::new(self, 16)
    }
    ///Bits 17:21
    #[inline(always)]
    #[must_use]
    pub fn debug_bit_sel(&mut self) -> DEBUG_BIT_SEL_W<SAR_DAC_CTRL1_SPEC> {
        DEBUG_BIT_SEL_W::new(self, 17)
    }
    ///Bit 22 - 0: DAC1 and DAC2 do not use DMA. 1: DAC1 and DAC2 use DMA.
    #[inline(always)]
    #[must_use]
    pub fn dac_dig_force(&mut self) -> DAC_DIG_FORCE_W<SAR_DAC_CTRL1_SPEC> {
        DAC_DIG_FORCE_W::new(self, 22)
    }
    ///Bit 23 - 1: force PDAC_CLK to low
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_force_low(&mut self) -> DAC_CLK_FORCE_LOW_W<SAR_DAC_CTRL1_SPEC> {
        DAC_CLK_FORCE_LOW_W::new(self, 23)
    }
    ///Bit 24 - 1: force PDAC_CLK to high
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_force_high(&mut self) -> DAC_CLK_FORCE_HIGH_W<SAR_DAC_CTRL1_SPEC> {
        DAC_CLK_FORCE_HIGH_W::new(self, 24)
    }
    ///Bit 25 - 1: invert PDAC_CLK.
    #[inline(always)]
    #[must_use]
    pub fn dac_clk_inv(&mut self) -> DAC_CLK_INV_W<SAR_DAC_CTRL1_SPEC> {
        DAC_CLK_INV_W::new(self, 25)
    }
    ///Bit 26 - Reset DAC by software.
    #[inline(always)]
    #[must_use]
    pub fn dac_reset(&mut self) -> DAC_RESET_W<SAR_DAC_CTRL1_SPEC> {
        DAC_RESET_W::new(self, 26)
    }
    ///Bit 27 - DAC clock gate enable bit.
    #[inline(always)]
    #[must_use]
    pub fn dac_clkgate_en(&mut self) -> DAC_CLKGATE_EN_W<SAR_DAC_CTRL1_SPEC> {
        DAC_CLKGATE_EN_W::new(self, 27)
    }
}
/**DAC control

You can [`read`](crate::generic::Reg::read) this register and get [`sar_dac_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_dac_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_DAC_CTRL1_SPEC;
impl crate::RegisterSpec for SAR_DAC_CTRL1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_dac_ctrl1::R`](R) reader structure
impl crate::Readable for SAR_DAC_CTRL1_SPEC {}
///`write(|w| ..)` method takes [`sar_dac_ctrl1::W`](W) writer structure
impl crate::Writable for SAR_DAC_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_DAC_CTRL1 to value 0
impl crate::Resettable for SAR_DAC_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
