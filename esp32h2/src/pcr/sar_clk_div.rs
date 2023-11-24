#[doc = "Register `SAR_CLK_DIV` reader"]
pub type R = crate::R<SAR_CLK_DIV_SPEC>;
#[doc = "Register `SAR_CLK_DIV` writer"]
pub type W = crate::W<SAR_CLK_DIV_SPEC>;
#[doc = "Field `SAR2_CLK_DIV_NUM` reader - xxxx"]
pub type SAR2_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SAR2_CLK_DIV_NUM` writer - xxxx"]
pub type SAR2_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SAR1_CLK_DIV_NUM` reader - xxxx"]
pub type SAR1_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SAR1_CLK_DIV_NUM` writer - xxxx"]
pub type SAR1_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    pub fn sar2_clk_div_num(&self) -> SAR2_CLK_DIV_NUM_R {
        SAR2_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - xxxx"]
    #[inline(always)]
    pub fn sar1_clk_div_num(&self) -> SAR1_CLK_DIV_NUM_R {
        SAR1_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_CLK_DIV")
            .field(
                "sar2_clk_div_num",
                &format_args!("{}", self.sar2_clk_div_num().bits()),
            )
            .field(
                "sar1_clk_div_num",
                &format_args!("{}", self.sar1_clk_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_CLK_DIV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_clk_div_num(&mut self) -> SAR2_CLK_DIV_NUM_W<SAR_CLK_DIV_SPEC> {
        SAR2_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_clk_div_num(&mut self) -> SAR1_CLK_DIV_NUM_W<SAR_CLK_DIV_SPEC> {
        SAR1_CLK_DIV_NUM_W::new(self, 8)
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
#[doc = "xxxx\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_clk_div::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_clk_div::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_CLK_DIV_SPEC;
impl crate::RegisterSpec for SAR_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_clk_div::R`](R) reader structure"]
impl crate::Readable for SAR_CLK_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_clk_div::W`](W) writer structure"]
impl crate::Writable for SAR_CLK_DIV_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_CLK_DIV to value 0x0404"]
impl crate::Resettable for SAR_CLK_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0404;
}
