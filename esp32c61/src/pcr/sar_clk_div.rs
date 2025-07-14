#[doc = "Register `SAR_CLK_DIV` reader"]
pub type R = crate::R<SAR_CLK_DIV_SPEC>;
#[doc = "Register `SAR_CLK_DIV` writer"]
pub type W = crate::W<SAR_CLK_DIV_SPEC>;
#[doc = "Field `SAR2_CLK_DIV_NUM` reader - Configures the divisor for SAR ADC 2 clock to generate ADC analog control signals.\\\\"]
pub type SAR2_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SAR2_CLK_DIV_NUM` writer - Configures the divisor for SAR ADC 2 clock to generate ADC analog control signals.\\\\"]
pub type SAR2_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_CLK_DIV_NUM` reader - Configures the divisor for SAR ADC 1 clock to generate ADC analog control signals.\\\\"]
pub type SAR1_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SAR1_CLK_DIV_NUM` writer - Configures the divisor for SAR ADC 1 clock to generate ADC analog control signals.\\\\"]
pub type SAR1_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the divisor for SAR ADC 2 clock to generate ADC analog control signals.\\\\"]
    #[inline(always)]
    pub fn sar2_clk_div_num(&self) -> SAR2_CLK_DIV_NUM_R {
        SAR2_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the divisor for SAR ADC 1 clock to generate ADC analog control signals.\\\\"]
    #[inline(always)]
    pub fn sar1_clk_div_num(&self) -> SAR1_CLK_DIV_NUM_R {
        SAR1_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_CLK_DIV")
            .field("sar2_clk_div_num", &self.sar2_clk_div_num())
            .field("sar1_clk_div_num", &self.sar1_clk_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the divisor for SAR ADC 2 clock to generate ADC analog control signals.\\\\"]
    #[inline(always)]
    pub fn sar2_clk_div_num(&mut self) -> SAR2_CLK_DIV_NUM_W<SAR_CLK_DIV_SPEC> {
        SAR2_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures the divisor for SAR ADC 1 clock to generate ADC analog control signals.\\\\"]
    #[inline(always)]
    pub fn sar1_clk_div_num(&mut self) -> SAR1_CLK_DIV_NUM_W<SAR_CLK_DIV_SPEC> {
        SAR1_CLK_DIV_NUM_W::new(self, 8)
    }
}
#[doc = "SAR ADC clock divisor configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_clk_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar_clk_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_CLK_DIV_SPEC;
impl crate::RegisterSpec for SAR_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_clk_div::R`](R) reader structure"]
impl crate::Readable for SAR_CLK_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_clk_div::W`](W) writer structure"]
impl crate::Writable for SAR_CLK_DIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_CLK_DIV to value 0x0404"]
impl crate::Resettable for SAR_CLK_DIV_SPEC {
    const RESET_VALUE: u32 = 0x0404;
}
