#[doc = "Register `SAR1_PATT_TAB4` reader"]
pub type R = crate::R<SAR1_PATT_TAB4_SPEC>;
#[doc = "Register `SAR1_PATT_TAB4` writer"]
pub type W = crate::W<SAR1_PATT_TAB4_SPEC>;
#[doc = "Field `SAR1_PATT_TAB4` reader - Item 12 ~ 15 for pattern table 1 (each item one byte)"]
pub type SAR1_PATT_TAB4_R = crate::FieldReader<u32>;
#[doc = "Field `SAR1_PATT_TAB4` writer - Item 12 ~ 15 for pattern table 1 (each item one byte)"]
pub type SAR1_PATT_TAB4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Item 12 ~ 15 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn sar1_patt_tab4(&self) -> SAR1_PATT_TAB4_R {
        SAR1_PATT_TAB4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_PATT_TAB4")
            .field("sar1_patt_tab4", &self.sar1_patt_tab4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 12 ~ 15 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn sar1_patt_tab4(&mut self) -> SAR1_PATT_TAB4_W<SAR1_PATT_TAB4_SPEC> {
        SAR1_PATT_TAB4_W::new(self, 0)
    }
}
#[doc = "Item 12 ~ 15 for pattern table 1 (each item one byte)\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR1_PATT_TAB4_SPEC;
impl crate::RegisterSpec for SAR1_PATT_TAB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_patt_tab4::R`](R) reader structure"]
impl crate::Readable for SAR1_PATT_TAB4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar1_patt_tab4::W`](W) writer structure"]
impl crate::Writable for SAR1_PATT_TAB4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR1_PATT_TAB4 to value 0x0f0f_0f0f"]
impl crate::Resettable for SAR1_PATT_TAB4_SPEC {
    const RESET_VALUE: u32 = 0x0f0f_0f0f;
}
