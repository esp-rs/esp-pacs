#[doc = "Register `READER2_CTRL` reader"]
pub type R = crate::R<READER2_CTRL_SPEC>;
#[doc = "Register `READER2_CTRL` writer"]
pub type W = crate::W<READER2_CTRL_SPEC>;
#[doc = "Field `SAR2_CLK_DIV` reader - Clock divider."]
pub type SAR2_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SAR2_CLK_DIV` writer - Clock divider."]
pub type SAR2_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR2_WAIT_ARB_CYCLE` reader - Wait arbit stable after sar_done."]
pub type SAR2_WAIT_ARB_CYCLE_R = crate::FieldReader;
#[doc = "Field `SAR2_WAIT_ARB_CYCLE` writer - Wait arbit stable after sar_done."]
pub type SAR2_WAIT_ARB_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR2_CLK_GATED` reader - N/A"]
pub type SAR2_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SAR2_CLK_GATED` writer - N/A"]
pub type SAR2_CLK_GATED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_SAMPLE_NUM` reader - N/A"]
pub type SAR2_SAMPLE_NUM_R = crate::FieldReader;
#[doc = "Field `SAR2_SAMPLE_NUM` writer - N/A"]
pub type SAR2_SAMPLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR2_EN_PAD_FORCE_ENABLE` reader - Force enable adc en_pad to analog circuit 2'b11: force enable ."]
pub type SAR2_EN_PAD_FORCE_ENABLE_R = crate::FieldReader;
#[doc = "Field `SAR2_EN_PAD_FORCE_ENABLE` writer - Force enable adc en_pad to analog circuit 2'b11: force enable ."]
pub type SAR2_EN_PAD_FORCE_ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR2_DATA_INV` reader - Invert SAR ADC2 data."]
pub type SAR2_DATA_INV_R = crate::BitReader;
#[doc = "Field `SAR2_DATA_INV` writer - Invert SAR ADC2 data."]
pub type SAR2_DATA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR2_INT_EN` reader - Enable saradc2 to send out interrupt."]
pub type SAR2_INT_EN_R = crate::BitReader;
#[doc = "Field `SAR2_INT_EN` writer - Enable saradc2 to send out interrupt."]
pub type SAR2_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Clock divider."]
    #[inline(always)]
    pub fn sar2_clk_div(&self) -> SAR2_CLK_DIV_R {
        SAR2_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Wait arbit stable after sar_done."]
    #[inline(always)]
    pub fn sar2_wait_arb_cycle(&self) -> SAR2_WAIT_ARB_CYCLE_R {
        SAR2_WAIT_ARB_CYCLE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn sar2_clk_gated(&self) -> SAR2_CLK_GATED_R {
        SAR2_CLK_GATED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - N/A"]
    #[inline(always)]
    pub fn sar2_sample_num(&self) -> SAR2_SAMPLE_NUM_R {
        SAR2_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bits 27:28 - Force enable adc en_pad to analog circuit 2'b11: force enable ."]
    #[inline(always)]
    pub fn sar2_en_pad_force_enable(&self) -> SAR2_EN_PAD_FORCE_ENABLE_R {
        SAR2_EN_PAD_FORCE_ENABLE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data."]
    #[inline(always)]
    pub fn sar2_data_inv(&self) -> SAR2_DATA_INV_R {
        SAR2_DATA_INV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Enable saradc2 to send out interrupt."]
    #[inline(always)]
    pub fn sar2_int_en(&self) -> SAR2_INT_EN_R {
        SAR2_INT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("READER2_CTRL")
            .field("sar2_clk_div", &self.sar2_clk_div())
            .field("sar2_wait_arb_cycle", &self.sar2_wait_arb_cycle())
            .field("sar2_clk_gated", &self.sar2_clk_gated())
            .field("sar2_sample_num", &self.sar2_sample_num())
            .field("sar2_en_pad_force_enable", &self.sar2_en_pad_force_enable())
            .field("sar2_data_inv", &self.sar2_data_inv())
            .field("sar2_int_en", &self.sar2_int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider."]
    #[inline(always)]
    pub fn sar2_clk_div(&mut self) -> SAR2_CLK_DIV_W<'_, READER2_CTRL_SPEC> {
        SAR2_CLK_DIV_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Wait arbit stable after sar_done."]
    #[inline(always)]
    pub fn sar2_wait_arb_cycle(&mut self) -> SAR2_WAIT_ARB_CYCLE_W<'_, READER2_CTRL_SPEC> {
        SAR2_WAIT_ARB_CYCLE_W::new(self, 16)
    }
    #[doc = "Bit 18 - N/A"]
    #[inline(always)]
    pub fn sar2_clk_gated(&mut self) -> SAR2_CLK_GATED_W<'_, READER2_CTRL_SPEC> {
        SAR2_CLK_GATED_W::new(self, 18)
    }
    #[doc = "Bits 19:26 - N/A"]
    #[inline(always)]
    pub fn sar2_sample_num(&mut self) -> SAR2_SAMPLE_NUM_W<'_, READER2_CTRL_SPEC> {
        SAR2_SAMPLE_NUM_W::new(self, 19)
    }
    #[doc = "Bits 27:28 - Force enable adc en_pad to analog circuit 2'b11: force enable ."]
    #[inline(always)]
    pub fn sar2_en_pad_force_enable(
        &mut self,
    ) -> SAR2_EN_PAD_FORCE_ENABLE_W<'_, READER2_CTRL_SPEC> {
        SAR2_EN_PAD_FORCE_ENABLE_W::new(self, 27)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data."]
    #[inline(always)]
    pub fn sar2_data_inv(&mut self) -> SAR2_DATA_INV_W<'_, READER2_CTRL_SPEC> {
        SAR2_DATA_INV_W::new(self, 29)
    }
    #[doc = "Bit 30 - Enable saradc2 to send out interrupt."]
    #[inline(always)]
    pub fn sar2_int_en(&mut self) -> SAR2_INT_EN_W<'_, READER2_CTRL_SPEC> {
        SAR2_INT_EN_W::new(self, 30)
    }
}
#[doc = "Control the read operation of ADC2.\n\nYou can [`read`](crate::Reg::read) this register and get [`reader2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reader2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct READER2_CTRL_SPEC;
impl crate::RegisterSpec for READER2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reader2_ctrl::R`](R) reader structure"]
impl crate::Readable for READER2_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reader2_ctrl::W`](W) writer structure"]
impl crate::Writable for READER2_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READER2_CTRL to value 0x4005_0002"]
impl crate::Resettable for READER2_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4005_0002;
}
