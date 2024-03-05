#[doc = "Register `SAR2_PATT_TAB4` reader"]
pub type R = crate::R<SAR2_PATT_TAB4_SPEC>;
#[doc = "Register `SAR2_PATT_TAB4` writer"]
pub type W = crate::W<SAR2_PATT_TAB4_SPEC>;
#[doc = "Field `SAR2_PATT_TAB4` reader - Item 12 ~ 15 for pattern table 2 (each item one byte)"]
pub type SAR2_PATT_TAB4_R = crate::FieldReader<u32>;
#[doc = "Field `SAR2_PATT_TAB4` writer - Item 12 ~ 15 for pattern table 2 (each item one byte)"]
pub type SAR2_PATT_TAB4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Item 12 ~ 15 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab4(&self) -> SAR2_PATT_TAB4_R {
        SAR2_PATT_TAB4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2_PATT_TAB4")
            .field(
                "sar2_patt_tab4",
                &format_args!("{}", self.sar2_patt_tab4().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR2_PATT_TAB4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 12 ~ 15 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_patt_tab4(&mut self) -> SAR2_PATT_TAB4_W<SAR2_PATT_TAB4_SPEC> {
        SAR2_PATT_TAB4_W::new(self, 0)
    }
}
#[doc = "Item 12 ~ 15 for pattern table 2 (each item one byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_patt_tab4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar2_patt_tab4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR2_PATT_TAB4_SPEC;
impl crate::RegisterSpec for SAR2_PATT_TAB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_patt_tab4::R`](R) reader structure"]
impl crate::Readable for SAR2_PATT_TAB4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar2_patt_tab4::W`](W) writer structure"]
impl crate::Writable for SAR2_PATT_TAB4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB4 to value 0x0f0f_0f0f"]
impl crate::Resettable for SAR2_PATT_TAB4_SPEC {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
