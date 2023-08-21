#[doc = "Register `SAR1_PATT_TAB4` reader"]
pub type R = crate::R<SAR1_PATT_TAB4_SPEC>;
#[doc = "Register `SAR1_PATT_TAB4` writer"]
pub type W = crate::W<SAR1_PATT_TAB4_SPEC>;
#[doc = "Field `SARADC_SAR1_PATT_TAB4` reader - Item 12 ~ 15 for pattern table 1 (each item 6bit)"]
pub type SARADC_SAR1_PATT_TAB4_R = crate::FieldReader<u32>;
#[doc = "Field `SARADC_SAR1_PATT_TAB4` writer - Item 12 ~ 15 for pattern table 1 (each item 6bit)"]
pub type SARADC_SAR1_PATT_TAB4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 24, O, u32>;
impl R {
    #[doc = "Bits 0:23 - Item 12 ~ 15 for pattern table 1 (each item 6bit)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab4(&self) -> SARADC_SAR1_PATT_TAB4_R {
        SARADC_SAR1_PATT_TAB4_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_PATT_TAB4")
            .field(
                "saradc_sar1_patt_tab4",
                &format_args!("{}", self.saradc_sar1_patt_tab4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR1_PATT_TAB4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:23 - Item 12 ~ 15 for pattern table 1 (each item 6bit)"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar1_patt_tab4(&mut self) -> SARADC_SAR1_PATT_TAB4_W<SAR1_PATT_TAB4_SPEC, 0> {
        SARADC_SAR1_PATT_TAB4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "configure apb saradc pattern table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar1_patt_tab4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar1_patt_tab4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR1_PATT_TAB4_SPEC;
impl crate::RegisterSpec for SAR1_PATT_TAB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_patt_tab4::R`](R) reader structure"]
impl crate::Readable for SAR1_PATT_TAB4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar1_patt_tab4::W`](W) writer structure"]
impl crate::Writable for SAR1_PATT_TAB4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR1_PATT_TAB4 to value 0"]
impl crate::Resettable for SAR1_PATT_TAB4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
