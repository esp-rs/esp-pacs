#[doc = "Register `SAR_READER2_CTRL` reader"]
pub type R = crate::R<SAR_READER2_CTRL_SPEC>;
#[doc = "Register `SAR_READER2_CTRL` writer"]
pub type W = crate::W<SAR_READER2_CTRL_SPEC>;
#[doc = "Field `SAR_SAR2_CLK_DIV` reader - clock divider"]
pub type SAR_SAR2_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_CLK_DIV` writer - clock divider"]
pub type SAR_SAR2_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR_SAR2_WAIT_ARB_CYCLE` reader - wait arbit stable after sar_done"]
pub type SAR_SAR2_WAIT_ARB_CYCLE_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_WAIT_ARB_CYCLE` writer - wait arbit stable after sar_done"]
pub type SAR_SAR2_WAIT_ARB_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAR_SAR2_CLK_GATED` reader - ******* Description ***********"]
pub type SAR_SAR2_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_CLK_GATED` writer - ******* Description ***********"]
pub type SAR_SAR2_CLK_GATED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_SAR2_SAMPLE_NUM` reader - ******* Description ***********"]
pub type SAR_SAR2_SAMPLE_NUM_R = crate::FieldReader;
#[doc = "Field `SAR_SAR2_SAMPLE_NUM` writer - ******* Description ***********"]
pub type SAR_SAR2_SAMPLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR_SAR2_DATA_INV` reader - Invert SAR ADC2 data"]
pub type SAR_SAR2_DATA_INV_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_DATA_INV` writer - Invert SAR ADC2 data"]
pub type SAR_SAR2_DATA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_SAR2_INT_EN` reader - enable saradc2 to send out interrupt"]
pub type SAR_SAR2_INT_EN_R = crate::BitReader;
#[doc = "Field `SAR_SAR2_INT_EN` writer - enable saradc2 to send out interrupt"]
pub type SAR_SAR2_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar_sar2_clk_div(&self) -> SAR_SAR2_CLK_DIV_R {
        SAR_SAR2_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - wait arbit stable after sar_done"]
    #[inline(always)]
    pub fn sar_sar2_wait_arb_cycle(&self) -> SAR_SAR2_WAIT_ARB_CYCLE_R {
        SAR_SAR2_WAIT_ARB_CYCLE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - ******* Description ***********"]
    #[inline(always)]
    pub fn sar_sar2_clk_gated(&self) -> SAR_SAR2_CLK_GATED_R {
        SAR_SAR2_CLK_GATED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn sar_sar2_sample_num(&self) -> SAR_SAR2_SAMPLE_NUM_R {
        SAR_SAR2_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar_sar2_data_inv(&self) -> SAR_SAR2_DATA_INV_R {
        SAR_SAR2_DATA_INV_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - enable saradc2 to send out interrupt"]
    #[inline(always)]
    pub fn sar_sar2_int_en(&self) -> SAR_SAR2_INT_EN_R {
        SAR_SAR2_INT_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READER2_CTRL")
            .field("sar_sar2_clk_div", &self.sar_sar2_clk_div())
            .field("sar_sar2_wait_arb_cycle", &self.sar_sar2_wait_arb_cycle())
            .field("sar_sar2_clk_gated", &self.sar_sar2_clk_gated())
            .field("sar_sar2_sample_num", &self.sar_sar2_sample_num())
            .field("sar_sar2_data_inv", &self.sar_sar2_data_inv())
            .field("sar_sar2_int_en", &self.sar_sar2_int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - clock divider"]
    #[inline(always)]
    pub fn sar_sar2_clk_div(&mut self) -> SAR_SAR2_CLK_DIV_W<SAR_READER2_CTRL_SPEC> {
        SAR_SAR2_CLK_DIV_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - wait arbit stable after sar_done"]
    #[inline(always)]
    pub fn sar_sar2_wait_arb_cycle(&mut self) -> SAR_SAR2_WAIT_ARB_CYCLE_W<SAR_READER2_CTRL_SPEC> {
        SAR_SAR2_WAIT_ARB_CYCLE_W::new(self, 16)
    }
    #[doc = "Bit 18 - ******* Description ***********"]
    #[inline(always)]
    pub fn sar_sar2_clk_gated(&mut self) -> SAR_SAR2_CLK_GATED_W<SAR_READER2_CTRL_SPEC> {
        SAR_SAR2_CLK_GATED_W::new(self, 18)
    }
    #[doc = "Bits 19:26 - ******* Description ***********"]
    #[inline(always)]
    pub fn sar_sar2_sample_num(&mut self) -> SAR_SAR2_SAMPLE_NUM_W<SAR_READER2_CTRL_SPEC> {
        SAR_SAR2_SAMPLE_NUM_W::new(self, 19)
    }
    #[doc = "Bit 29 - Invert SAR ADC2 data"]
    #[inline(always)]
    pub fn sar_sar2_data_inv(&mut self) -> SAR_SAR2_DATA_INV_W<SAR_READER2_CTRL_SPEC> {
        SAR_SAR2_DATA_INV_W::new(self, 29)
    }
    #[doc = "Bit 30 - enable saradc2 to send out interrupt"]
    #[inline(always)]
    pub fn sar_sar2_int_en(&mut self) -> SAR_SAR2_INT_EN_W<SAR_READER2_CTRL_SPEC> {
        SAR_SAR2_INT_EN_W::new(self, 30)
    }
}
#[doc = "configure saradc2 reader\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_reader2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_READER2_CTRL_SPEC;
impl crate::RegisterSpec for SAR_READER2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_reader2_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_READER2_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_reader2_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_READER2_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_READER2_CTRL to value 0x4005_0002"]
impl crate::Resettable for SAR_READER2_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4005_0002;
}
