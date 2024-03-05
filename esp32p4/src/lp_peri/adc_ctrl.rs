#[doc = "Register `ADC_CTRL` reader"]
pub type R = crate::R<ADC_CTRL_SPEC>;
#[doc = "Register `ADC_CTRL` writer"]
pub type W = crate::W<ADC_CTRL_SPEC>;
#[doc = "Field `SAR2_CLK_FORCE_ON` reader - need_des"]
pub type SAR2_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `SAR2_CLK_FORCE_ON` writer - need_des"]
pub type SAR2_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR1_CLK_FORCE_ON` reader - need_des"]
pub type SAR1_CLK_FORCE_ON_R = crate::BitReader;
#[doc = "Field `SAR1_CLK_FORCE_ON` writer - need_des"]
pub type SAR1_CLK_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPADC_FUNC_DIV_NUM` reader - need_des"]
pub type LPADC_FUNC_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LPADC_FUNC_DIV_NUM` writer - need_des"]
pub type LPADC_FUNC_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPADC_SAR2_DIV_NUM` reader - need_des"]
pub type LPADC_SAR2_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LPADC_SAR2_DIV_NUM` writer - need_des"]
pub type LPADC_SAR2_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPADC_SAR1_DIV_NUM` reader - need_des"]
pub type LPADC_SAR1_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `LPADC_SAR1_DIV_NUM` writer - need_des"]
pub type LPADC_SAR1_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn sar2_clk_force_on(&self) -> SAR2_CLK_FORCE_ON_R {
        SAR2_CLK_FORCE_ON_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn sar1_clk_force_on(&self) -> SAR1_CLK_FORCE_ON_R {
        SAR1_CLK_FORCE_ON_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn lpadc_func_div_num(&self) -> LPADC_FUNC_DIV_NUM_R {
        LPADC_FUNC_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn lpadc_sar2_div_num(&self) -> LPADC_SAR2_DIV_NUM_R {
        LPADC_SAR2_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn lpadc_sar1_div_num(&self) -> LPADC_SAR1_DIV_NUM_R {
        LPADC_SAR1_DIV_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADC_CTRL")
            .field(
                "sar2_clk_force_on",
                &format_args!("{}", self.sar2_clk_force_on().bit()),
            )
            .field(
                "sar1_clk_force_on",
                &format_args!("{}", self.sar1_clk_force_on().bit()),
            )
            .field(
                "lpadc_func_div_num",
                &format_args!("{}", self.lpadc_func_div_num().bits()),
            )
            .field(
                "lpadc_sar2_div_num",
                &format_args!("{}", self.lpadc_sar2_div_num().bits()),
            )
            .field(
                "lpadc_sar1_div_num",
                &format_args!("{}", self.lpadc_sar1_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ADC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_clk_force_on(&mut self) -> SAR2_CLK_FORCE_ON_W<ADC_CTRL_SPEC> {
        SAR2_CLK_FORCE_ON_W::new(self, 6)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_clk_force_on(&mut self) -> SAR1_CLK_FORCE_ON_W<ADC_CTRL_SPEC> {
        SAR1_CLK_FORCE_ON_W::new(self, 7)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lpadc_func_div_num(&mut self) -> LPADC_FUNC_DIV_NUM_W<ADC_CTRL_SPEC> {
        LPADC_FUNC_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lpadc_sar2_div_num(&mut self) -> LPADC_SAR2_DIV_NUM_W<ADC_CTRL_SPEC> {
        LPADC_SAR2_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lpadc_sar1_div_num(&mut self) -> LPADC_SAR1_DIV_NUM_W<ADC_CTRL_SPEC> {
        LPADC_SAR1_DIV_NUM_W::new(self, 24)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADC_CTRL_SPEC;
impl crate::RegisterSpec for ADC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adc_ctrl::R`](R) reader structure"]
impl crate::Readable for ADC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adc_ctrl::W`](W) writer structure"]
impl crate::Writable for ADC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADC_CTRL to value 0x0404_0400"]
impl crate::Resettable for ADC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0404_0400;
}
