#[doc = "Register `APB_SARADC_SAR1_PATT_TAB%s` reader"]
pub type R = crate::R<APB_SARADC_SAR1_PATT_TAB_SPEC>;
#[doc = "Register `APB_SARADC_SAR1_PATT_TAB%s` writer"]
pub type W = crate::W<APB_SARADC_SAR1_PATT_TAB_SPEC>;
#[doc = "Field `SARADC_SAR1_PATT_TAB1` reader - item 0 ~ 3 for pattern table 1 (each item one byte)"]
pub type SARADC_SAR1_PATT_TAB1_R = crate::FieldReader<u32>;
#[doc = "Field `SARADC_SAR1_PATT_TAB1` writer - item 0 ~ 3 for pattern table 1 (each item one byte)"]
pub type SARADC_SAR1_PATT_TAB1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - item 0 ~ 3 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab1(&self) -> SARADC_SAR1_PATT_TAB1_R {
        SARADC_SAR1_PATT_TAB1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC_SAR1_PATT_TAB")
            .field("saradc_sar1_patt_tab1", &self.saradc_sar1_patt_tab1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - item 0 ~ 3 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar1_patt_tab1(
        &mut self,
    ) -> SARADC_SAR1_PATT_TAB1_W<APB_SARADC_SAR1_PATT_TAB_SPEC> {
        SARADC_SAR1_PATT_TAB1_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_sar1_patt_tab::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_sar1_patt_tab::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_SARADC_SAR1_PATT_TAB_SPEC;
impl crate::RegisterSpec for APB_SARADC_SAR1_PATT_TAB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_saradc_sar1_patt_tab::R`](R) reader structure"]
impl crate::Readable for APB_SARADC_SAR1_PATT_TAB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_saradc_sar1_patt_tab::W`](W) writer structure"]
impl crate::Writable for APB_SARADC_SAR1_PATT_TAB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_SARADC_SAR1_PATT_TAB%s to value 0x0f0f_0f0f"]
impl crate::Resettable for APB_SARADC_SAR1_PATT_TAB_SPEC {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
