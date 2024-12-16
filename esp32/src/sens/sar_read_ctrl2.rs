#[doc = "Register `SAR_READ_CTRL2` reader"]
pub type R = crate::R<SAR_READ_CTRL2_SPEC>;
#[doc = "Register `SAR_READ_CTRL2` writer"]
pub type W = crate::W<SAR_READ_CTRL2_SPEC>;
#[doc = "Field `SAR2_CLK_DIV` reader - clock divider"]
pub type SAR2_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SAR2_CLK_DIV` writer - clock divider"]
pub type SAR2_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR2_SAMPLE_CYCLE` reader - sample cycles for SAR ADC2"]
pub type SAR2_SAMPLE_CYCLE_R = crate::FieldReader;
#[doc = "Field `SAR2_SAMPLE_CYCLE` writer - sample cycles for SAR ADC2"]
pub type SAR2_SAMPLE_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR2_SAMPLE_BIT` reader - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
pub type SAR2_SAMPLE_BIT_R = crate::FieldReader;
#[doc = "Field `SAR2_SAMPLE_BIT` writer - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
pub type SAR2_SAMPLE_BIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR2_CLK_GATED` reader - "]
pub type SAR2_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SAR2_CLK_GATED` writer - "]
pub type SAR2_CLK_GATED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_SAMPLE_NUM` reader - "]
pub type SAR2_SAMPLE_NUM_R = crate::FieldReader;
#[doc = "Field `SAR2_SAMPLE_NUM` writer - "]
pub type SAR2_SAMPLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR2_PWDET_FORCE` reader - "]
pub type SAR2_PWDET_FORCE_R = crate::BitReader;
#[doc = "Field `SAR2_PWDET_FORCE` writer - "]
pub type SAR2_PWDET_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_DIG_FORCE` reader - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
pub type SAR2_DIG_FORCE_R = crate::BitReader;
#[doc = "Field `SAR2_DIG_FORCE` writer - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
pub type SAR2_DIG_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_DATA_INV` reader - Invert SAR ADC2 data"]
pub type SAR2_DATA_INV_R = crate::BitReader;
#[doc = "Field `SAR2_DATA_INV` writer - Invert SAR ADC2 data"]
pub type SAR2_DATA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar2_clk_div(&self) -> SAR2_CLK_DIV_R {
        SAR2_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC2"]
    #[inline(always)]
    pub fn sar2_sample_cycle(&self) -> SAR2_SAMPLE_CYCLE_R {
        SAR2_SAMPLE_CYCLE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sar2_sample_bit(&self) -> SAR2_SAMPLE_BIT_R {
        SAR2_SAMPLE_BIT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar2_clk_gated(&self) -> SAR2_CLK_GATED_R {
        SAR2_CLK_GATED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar2_sample_num(&self) -> SAR2_SAMPLE_NUM_R {
        SAR2_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sar2_pwdet_force(&self) -> SAR2_PWDET_FORCE_R {
        SAR2_PWDET_FORCE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_dig_force(&self) -> SAR2_DIG_FORCE_R {
        SAR2_DIG_FORCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar2_data_inv(&self) -> SAR2_DATA_INV_R {
        SAR2_DATA_INV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READ_CTRL2")
            .field("sar2_clk_div", &self.sar2_clk_div())
            .field("sar2_sample_cycle", &self.sar2_sample_cycle())
            .field("sar2_sample_bit", &self.sar2_sample_bit())
            .field("sar2_clk_gated", &self.sar2_clk_gated())
            .field("sar2_sample_num", &self.sar2_sample_num())
            .field("sar2_pwdet_force", &self.sar2_pwdet_force())
            .field("sar2_dig_force", &self.sar2_dig_force())
            .field("sar2_data_inv", &self.sar2_data_inv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar2_clk_div(&mut self) -> SAR2_CLK_DIV_W<SAR_READ_CTRL2_SPEC> {
        SAR2_CLK_DIV_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - sample cycles for SAR ADC2"]
    #[inline(always)]
    pub fn sar2_sample_cycle(&mut self) -> SAR2_SAMPLE_CYCLE_W<SAR_READ_CTRL2_SPEC> {
        SAR2_SAMPLE_CYCLE_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - 00: for 9-bit width 01: for 10-bit width 10: for 11-bit width 11: for 12-bit width"]
    #[inline(always)]
    pub fn sar2_sample_bit(&mut self) -> SAR2_SAMPLE_BIT_W<SAR_READ_CTRL2_SPEC> {
        SAR2_SAMPLE_BIT_W::new(self, 16)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar2_clk_gated(&mut self) -> SAR2_CLK_GATED_W<SAR_READ_CTRL2_SPEC> {
        SAR2_CLK_GATED_W::new(self, 18)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar2_sample_num(&mut self) -> SAR2_SAMPLE_NUM_W<SAR_READ_CTRL2_SPEC> {
        SAR2_SAMPLE_NUM_W::new(self, 19)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn sar2_pwdet_force(&mut self) -> SAR2_PWDET_FORCE_W<SAR_READ_CTRL2_SPEC> {
        SAR2_PWDET_FORCE_W::new(self, 27)
    }
    #[doc = "Bit 28 - 1: SAR ADC2 controlled by DIG ADC2 CTRL or PWDET CTRL 0: SAR ADC2 controlled by RTC ADC2 CTRL"]
    #[inline(always)]
    pub fn sar2_dig_force(&mut self) -> SAR2_DIG_FORCE_W<SAR_READ_CTRL2_SPEC> {
        SAR2_DIG_FORCE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar2_data_inv(&mut self) -> SAR2_DATA_INV_W<SAR_READ_CTRL2_SPEC> {
        SAR2_DATA_INV_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_read_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_read_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_READ_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_READ_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_read_ctrl2::R`](R) reader structure"]
impl crate::Readable for SAR_READ_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_read_ctrl2::W`](W) writer structure"]
impl crate::Writable for SAR_READ_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_READ_CTRL2 to value 0x0007_0902"]
impl crate::Resettable for SAR_READ_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x0007_0902;
}
