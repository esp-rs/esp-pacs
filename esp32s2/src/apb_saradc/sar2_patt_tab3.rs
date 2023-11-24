#[doc = "Register `SAR2_PATT_TAB3` reader"]
pub type R = crate::R<SAR2_PATT_TAB3_SPEC>;
#[doc = "Register `SAR2_PATT_TAB3` writer"]
pub type W = crate::W<SAR2_PATT_TAB3_SPEC>;
#[doc = "Field `SAR2_PATT_TAB3` reader - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
pub type SAR2_PATT_TAB3_R = crate::FieldReader<u32>;
#[doc = "Field `SAR2_PATT_TAB3` writer - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
pub type SAR2_PATT_TAB3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab3(&self) -> SAR2_PATT_TAB3_R {
        SAR2_PATT_TAB3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2_PATT_TAB3")
            .field(
                "sar2_patt_tab3",
                &format_args!("{}", self.sar2_patt_tab3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR2_PATT_TAB3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 8 ~ 11 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_patt_tab3(&mut self) -> SAR2_PATT_TAB3_W<SAR2_PATT_TAB3_SPEC> {
        SAR2_PATT_TAB3_W::new(self, 0)
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
#[doc = "Item 8 ~ 11 for pattern table 2 (each item one byte)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar2_patt_tab3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar2_patt_tab3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR2_PATT_TAB3_SPEC;
impl crate::RegisterSpec for SAR2_PATT_TAB3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_patt_tab3::R`](R) reader structure"]
impl crate::Readable for SAR2_PATT_TAB3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar2_patt_tab3::W`](W) writer structure"]
impl crate::Writable for SAR2_PATT_TAB3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB3 to value 0x0f0f_0f0f"]
impl crate::Resettable for SAR2_PATT_TAB3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}
