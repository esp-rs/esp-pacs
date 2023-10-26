#[doc = "Register `SAR_READER1_CTRL` reader"]
pub type R = crate::R<SAR_READER1_CTRL_SPEC>;
#[doc = "Register `SAR_READER1_CTRL` writer"]
pub type W = crate::W<SAR_READER1_CTRL_SPEC>;
#[doc = "Field `SAR1_CLK_DIV` reader - Clock divider."]
pub type SAR1_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `SAR1_CLK_DIV` writer - Clock divider."]
pub type SAR1_CLK_DIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SAR1_CLK_GATED` reader - "]
pub type SAR1_CLK_GATED_R = crate::BitReader;
#[doc = "Field `SAR1_CLK_GATED` writer - "]
pub type SAR1_CLK_GATED_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR1_SAMPLE_NUM` reader - "]
pub type SAR1_SAMPLE_NUM_R = crate::FieldReader;
#[doc = "Field `SAR1_SAMPLE_NUM` writer - "]
pub type SAR1_SAMPLE_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `SAR1_DATA_INV` reader - Invert SAR ADC1 data."]
pub type SAR1_DATA_INV_R = crate::BitReader;
#[doc = "Field `SAR1_DATA_INV` writer - Invert SAR ADC1 data."]
pub type SAR1_DATA_INV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `SAR1_INT_EN` reader - Enable SAR ADC1 to send out interrupt."]
pub type SAR1_INT_EN_R = crate::BitReader;
#[doc = "Field `SAR1_INT_EN` writer - Enable SAR ADC1 to send out interrupt."]
pub type SAR1_INT_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
            .field(
                "sar1_clk_div",
                &format_args!("{}", self.sar1_clk_div().bits()),
            )
            .field(
                "sar1_clk_gated",
                &format_args!("{}", self.sar1_clk_gated().bit()),
            )
            .field(
                "sar1_sample_num",
                &format_args!("{}", self.sar1_sample_num().bits()),
            )
            .field(
                "sar1_data_inv",
                &format_args!("{}", self.sar1_data_inv().bit()),
            )
            .field("sar1_int_en", &format_args!("{}", self.sar1_int_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_READER1_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divider."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_clk_div(&mut self) -> SAR1_CLK_DIV_W<SAR_READER1_CTRL_SPEC, 0> {
        SAR1_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_clk_gated(&mut self) -> SAR1_CLK_GATED_W<SAR_READER1_CTRL_SPEC, 18> {
        SAR1_CLK_GATED_W::new(self)
    }
    #[doc = "Bits 19:26"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_sample_num(&mut self) -> SAR1_SAMPLE_NUM_W<SAR_READER1_CTRL_SPEC, 19> {
        SAR1_SAMPLE_NUM_W::new(self)
    }
    #[doc = "Bit 28 - Invert SAR ADC1 data."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_data_inv(&mut self) -> SAR1_DATA_INV_W<SAR_READER1_CTRL_SPEC, 28> {
        SAR1_DATA_INV_W::new(self)
    }
    #[doc = "Bit 29 - Enable SAR ADC1 to send out interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sar1_int_en(&mut self) -> SAR1_INT_EN_W<SAR_READER1_CTRL_SPEC, 29> {
        SAR1_INT_EN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RTC ADC1 data and sampling control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_reader1_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_reader1_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_READER1_CTRL_SPEC;
impl crate::RegisterSpec for SAR_READER1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_reader1_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_READER1_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_reader1_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_READER1_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_READER1_CTRL to value 0x2004_0002"]
impl crate::Resettable for SAR_READER1_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x2004_0002;
}
