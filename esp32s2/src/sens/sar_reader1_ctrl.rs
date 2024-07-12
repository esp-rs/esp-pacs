#[doc = "Register `SAR_READER1_CTRL` reader"]
pub type R = crate::R<SAR_READER1_CTRL_SPEC>;
#[doc = "Register `SAR_READER1_CTRL` writer"]
pub type W = crate::W<SAR_READER1_CTRL_SPEC>;
#[doc = "Field `SAR1_CLK_DIV` reader - Clock divider."]
pub type SAR1_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SAR1_CLK_DIV` writer - Clock divider."]
pub type SAR1_CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_CLK_GATED` reader - "]
pub type SAR1_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SAR1_CLK_GATED` writer - "]
pub type SAR1_CLK_GATED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_SAMPLE_NUM` reader - "]
pub type SAR1_SAMPLE_NUM_R = crate::FieldReader;
#[doc = "Field `SAR1_SAMPLE_NUM` writer - "]
pub type SAR1_SAMPLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_DATA_INV` reader - Invert SAR ADC1 data."]
pub type SAR1_DATA_INV_R = crate::BitReader;
#[doc = "Field `SAR1_DATA_INV` writer - Invert SAR ADC1 data."]
pub type SAR1_DATA_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_INT_EN` reader - Enable SAR ADC1 to send out interrupt."]
pub type SAR1_INT_EN_R = crate::BitReader;
#[doc = "Field `SAR1_INT_EN` writer - Enable SAR ADC1 to send out interrupt."]
pub type SAR1_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Clock divider."]
    #[inline(always)]
    pub fn sar1_clk_div(&self) -> SAR1_CLK_DIV_R {
        SAR1_CLK_DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn sar1_clk_gated(&self) -> SAR1_CLK_GATED_R {
        SAR1_CLK_GATED_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    pub fn sar1_sample_num(&self) -> SAR1_SAMPLE_NUM_R {
        SAR1_SAMPLE_NUM_R::new(((self.bits >> 19) & 0xff) as u8)
    }
    #[doc = "Bit 28 - Invert SAR ADC1 data."]
    #[inline(always)]
    pub fn sar1_data_inv(&self) -> SAR1_DATA_INV_R {
        SAR1_DATA_INV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable SAR ADC1 to send out interrupt."]
    #[inline(always)]
    pub fn sar1_int_en(&self) -> SAR1_INT_EN_R {
        SAR1_INT_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_READER1_CTRL")
            .field("sar1_clk_div", &self.sar1_clk_div())
            .field("sar1_clk_gated", &self.sar1_clk_gated())
            .field("sar1_sample_num", &self.sar1_sample_num())
            .field("sar1_data_inv", &self.sar1_data_inv())
            .field("sar1_int_en", &self.sar1_int_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_clk_div(&mut self) -> SAR1_CLK_DIV_W<SAR_READER1_CTRL_SPEC> {
        SAR1_CLK_DIV_W::new(self, 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_clk_gated(&mut self) -> SAR1_CLK_GATED_W<SAR_READER1_CTRL_SPEC> {
        SAR1_CLK_GATED_W::new(self, 18)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_sample_num(&mut self) -> SAR1_SAMPLE_NUM_W<SAR_READER1_CTRL_SPEC> {
        SAR1_SAMPLE_NUM_W::new(self, 19)
    }
    #[doc = "Bit 28 - Invert SAR ADC1 data."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_data_inv(&mut self) -> SAR1_DATA_INV_W<SAR_READER1_CTRL_SPEC> {
        SAR1_DATA_INV_W::new(self, 28)
    }
    #[doc = "Bit 29 - Enable SAR ADC1 to send out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_int_en(&mut self) -> SAR1_INT_EN_W<SAR_READER1_CTRL_SPEC> {
        SAR1_INT_EN_W::new(self, 29)
    }
}
#[doc = "RTC ADC1 data and sampling control\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_reader1_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_reader1_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_READER1_CTRL_SPEC;
impl crate::RegisterSpec for SAR_READER1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_reader1_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_READER1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_reader1_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_READER1_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_READER1_CTRL to value 0x2004_0002"]
impl crate::Resettable for SAR_READER1_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2004_0002;
}
