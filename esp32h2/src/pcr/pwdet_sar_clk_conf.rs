#[doc = "Register `PWDET_SAR_CLK_CONF` reader"]
pub type R = crate::R<PWDET_SAR_CLK_CONF_SPEC>;
#[doc = "Register `PWDET_SAR_CLK_CONF` writer"]
pub type W = crate::W<PWDET_SAR_CLK_CONF_SPEC>;
#[doc = "Field `PWDET_SAR_CLK_DIV_NUM` reader - xxxx"]
pub type PWDET_SAR_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PWDET_SAR_CLK_DIV_NUM` writer - xxxx"]
pub type PWDET_SAR_CLK_DIV_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `PWDET_SAR_READER_EN` reader - xxxx"]
pub type PWDET_SAR_READER_EN_R = crate::BitReader;
#[doc = "Field `PWDET_SAR_READER_EN` writer - xxxx"]
pub type PWDET_SAR_READER_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    pub fn pwdet_sar_clk_div_num(&self) -> PWDET_SAR_CLK_DIV_NUM_R {
        PWDET_SAR_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - xxxx"]
    #[inline(always)]
    pub fn pwdet_sar_reader_en(&self) -> PWDET_SAR_READER_EN_R {
        PWDET_SAR_READER_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDET_SAR_CLK_CONF")
            .field(
                "pwdet_sar_clk_div_num",
                &format_args!("{}", self.pwdet_sar_clk_div_num().bits()),
            )
            .field(
                "pwdet_sar_reader_en",
                &format_args!("{}", self.pwdet_sar_reader_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PWDET_SAR_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn pwdet_sar_clk_div_num(&mut self) -> PWDET_SAR_CLK_DIV_NUM_W<PWDET_SAR_CLK_CONF_SPEC, 0> {
        PWDET_SAR_CLK_DIV_NUM_W::new(self)
    }
    #[doc = "Bit 8 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn pwdet_sar_reader_en(&mut self) -> PWDET_SAR_READER_EN_W<PWDET_SAR_CLK_CONF_SPEC, 8> {
        PWDET_SAR_READER_EN_W::new(self)
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
#[doc = "xxxx\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwdet_sar_clk_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwdet_sar_clk_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWDET_SAR_CLK_CONF_SPEC;
impl crate::RegisterSpec for PWDET_SAR_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwdet_sar_clk_conf::R`](R) reader structure"]
impl crate::Readable for PWDET_SAR_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwdet_sar_clk_conf::W`](W) writer structure"]
impl crate::Writable for PWDET_SAR_CLK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWDET_SAR_CLK_CONF to value 0x0107"]
impl crate::Resettable for PWDET_SAR_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0107;
}
