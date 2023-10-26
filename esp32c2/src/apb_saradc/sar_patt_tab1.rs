#[doc = "Register `SAR_PATT_TAB1` reader"]
pub type R = crate::R<SAR_PATT_TAB1_SPEC>;
#[doc = "Register `SAR_PATT_TAB1` writer"]
pub type W = crate::W<SAR_PATT_TAB1_SPEC>;
#[doc = "Field `SARADC_SAR_PATT_TAB1` reader - item 0 ~ 3 for pattern table 1 (each item one byte)"]
pub type SARADC_SAR_PATT_TAB1_R = crate::FieldReader<u32>;
#[doc = "Field `SARADC_SAR_PATT_TAB1` writer - item 0 ~ 3 for pattern table 1 (each item one byte)"]
pub type SARADC_SAR_PATT_TAB1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - item 0 ~ 3 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar_patt_tab1(&self) -> SARADC_SAR_PATT_TAB1_R {
        SARADC_SAR_PATT_TAB1_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_PATT_TAB1")
            .field(
                "saradc_sar_patt_tab1",
                &format_args!("{}", self.saradc_sar_patt_tab1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_PATT_TAB1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - item 0 ~ 3 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar_patt_tab1(&mut self) -> SARADC_SAR_PATT_TAB1_W<SAR_PATT_TAB1_SPEC, 0> {
        SARADC_SAR_PATT_TAB1_W::new(self)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_patt_tab1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_patt_tab1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_PATT_TAB1_SPEC;
impl crate::RegisterSpec for SAR_PATT_TAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_patt_tab1::R`](R) reader structure"]
impl crate::Readable for SAR_PATT_TAB1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_patt_tab1::W`](W) writer structure"]
impl crate::Writable for SAR_PATT_TAB1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_PATT_TAB1 to value 0x00ff_ffff"]
impl crate::Resettable for SAR_PATT_TAB1_SPEC {
    const RESET_VALUE: Self::Ux = 0x00ff_ffff;
}
