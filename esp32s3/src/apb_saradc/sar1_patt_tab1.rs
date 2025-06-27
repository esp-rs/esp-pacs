#[doc = "Register `SAR1_PATT_TAB1` reader"]
pub type R = crate::R<SAR1_PATT_TAB1_SPEC>;
#[doc = "Register `SAR1_PATT_TAB1` writer"]
pub type W = crate::W<SAR1_PATT_TAB1_SPEC>;
#[doc = "Field `SAR1_PATT_TAB1` reader - item 0 ~ 3 for pattern table 1 (each item 6bit)"]
pub type SAR1_PATT_TAB1_R = crate::FieldReader<u32>;
#[doc = "Field `SAR1_PATT_TAB1` writer - item 0 ~ 3 for pattern table 1 (each item 6bit)"]
pub type SAR1_PATT_TAB1_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - item 0 ~ 3 for pattern table 1 (each item 6bit)"]
    #[inline(always)]
    pub fn sar1_patt_tab1(&self) -> SAR1_PATT_TAB1_R {
        SAR1_PATT_TAB1_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR1_PATT_TAB1")
            .field("sar1_patt_tab1", &self.sar1_patt_tab1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - item 0 ~ 3 for pattern table 1 (each item 6bit)"]
    #[inline(always)]
    pub fn sar1_patt_tab1(&mut self) -> SAR1_PATT_TAB1_W<SAR1_PATT_TAB1_SPEC> {
        SAR1_PATT_TAB1_W::new(self, 0)
    }
}
#[doc = "configure apb saradc pattern table\n\nYou can [`read`](crate::Reg::read) this register and get [`sar1_patt_tab1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar1_patt_tab1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR1_PATT_TAB1_SPEC;
impl crate::RegisterSpec for SAR1_PATT_TAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar1_patt_tab1::R`](R) reader structure"]
impl crate::Readable for SAR1_PATT_TAB1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar1_patt_tab1::W`](W) writer structure"]
impl crate::Writable for SAR1_PATT_TAB1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR1_PATT_TAB1 to value 0"]
impl crate::Resettable for SAR1_PATT_TAB1_SPEC {}
