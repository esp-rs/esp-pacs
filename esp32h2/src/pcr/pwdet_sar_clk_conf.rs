#[doc = "Register `PWDET_SAR_CLK_CONF` reader"]
pub type R = crate::R<PWDET_SAR_CLK_CONF_SPEC>;
#[doc = "Register `PWDET_SAR_CLK_CONF` writer"]
pub type W = crate::W<PWDET_SAR_CLK_CONF_SPEC>;
#[doc = "Field `PWDET_SAR_CLK_DIV_NUM` reader - xxxx"]
pub type PWDET_SAR_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PWDET_SAR_CLK_DIV_NUM` writer - xxxx"]
pub type PWDET_SAR_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PWDET_SAR_READER_EN` reader - xxxx"]
pub type PWDET_SAR_READER_EN_R = crate::BitReader;
#[doc = "Field `PWDET_SAR_READER_EN` writer - xxxx"]
pub type PWDET_SAR_READER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
            .field("pwdet_sar_clk_div_num", &self.pwdet_sar_clk_div_num())
            .field("pwdet_sar_reader_en", &self.pwdet_sar_reader_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    pub fn pwdet_sar_clk_div_num(&mut self) -> PWDET_SAR_CLK_DIV_NUM_W<PWDET_SAR_CLK_CONF_SPEC> {
        PWDET_SAR_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - xxxx"]
    #[inline(always)]
    pub fn pwdet_sar_reader_en(&mut self) -> PWDET_SAR_READER_EN_W<PWDET_SAR_CLK_CONF_SPEC> {
        PWDET_SAR_READER_EN_W::new(self, 8)
    }
}
#[doc = "xxxx\n\nYou can [`read`](crate::Reg::read) this register and get [`pwdet_sar_clk_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwdet_sar_clk_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWDET_SAR_CLK_CONF_SPEC;
impl crate::RegisterSpec for PWDET_SAR_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwdet_sar_clk_conf::R`](R) reader structure"]
impl crate::Readable for PWDET_SAR_CLK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pwdet_sar_clk_conf::W`](W) writer structure"]
impl crate::Writable for PWDET_SAR_CLK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PWDET_SAR_CLK_CONF to value 0x0107"]
impl crate::Resettable for PWDET_SAR_CLK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0107;
}
