#[doc = "Register `GMAC0_PAD_BIST_INT_ENA` reader"]
pub type R = crate::R<GMAC0_PAD_BIST_INT_ENA_SPEC>;
#[doc = "Register `GMAC0_PAD_BIST_INT_ENA` writer"]
pub type W = crate::W<GMAC0_PAD_BIST_INT_ENA_SPEC>;
#[doc = "Field `GMAC0_PAD_BIST_OK_INT_ENA` reader - Write 1 to enable gmac0 pad bist ok interrupt"]
pub type GMAC0_PAD_BIST_OK_INT_ENA_R = crate::BitReader;
#[doc = "Field `GMAC0_PAD_BIST_OK_INT_ENA` writer - Write 1 to enable gmac0 pad bist ok interrupt"]
pub type GMAC0_PAD_BIST_OK_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMAC0_PAD_BIST_FAIL_INT_ENA` reader - Write 1 to enable gmac0 pad bist fail interrupt"]
pub type GMAC0_PAD_BIST_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `GMAC0_PAD_BIST_FAIL_INT_ENA` writer - Write 1 to enable gmac0 pad bist fail interrupt"]
pub type GMAC0_PAD_BIST_FAIL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable gmac0 pad bist ok interrupt"]
    #[inline(always)]
    pub fn gmac0_pad_bist_ok_int_ena(&self) -> GMAC0_PAD_BIST_OK_INT_ENA_R {
        GMAC0_PAD_BIST_OK_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable gmac0 pad bist fail interrupt"]
    #[inline(always)]
    pub fn gmac0_pad_bist_fail_int_ena(&self) -> GMAC0_PAD_BIST_FAIL_INT_ENA_R {
        GMAC0_PAD_BIST_FAIL_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMAC0_PAD_BIST_INT_ENA")
            .field(
                "gmac0_pad_bist_ok_int_ena",
                &self.gmac0_pad_bist_ok_int_ena(),
            )
            .field(
                "gmac0_pad_bist_fail_int_ena",
                &self.gmac0_pad_bist_fail_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable gmac0 pad bist ok interrupt"]
    #[inline(always)]
    pub fn gmac0_pad_bist_ok_int_ena(
        &mut self,
    ) -> GMAC0_PAD_BIST_OK_INT_ENA_W<'_, GMAC0_PAD_BIST_INT_ENA_SPEC> {
        GMAC0_PAD_BIST_OK_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable gmac0 pad bist fail interrupt"]
    #[inline(always)]
    pub fn gmac0_pad_bist_fail_int_ena(
        &mut self,
    ) -> GMAC0_PAD_BIST_FAIL_INT_ENA_W<'_, GMAC0_PAD_BIST_INT_ENA_SPEC> {
        GMAC0_PAD_BIST_FAIL_INT_ENA_W::new(self, 1)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac0_pad_bist_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac0_pad_bist_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC0_PAD_BIST_INT_ENA_SPEC;
impl crate::RegisterSpec for GMAC0_PAD_BIST_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac0_pad_bist_int_ena::R`](R) reader structure"]
impl crate::Readable for GMAC0_PAD_BIST_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac0_pad_bist_int_ena::W`](W) writer structure"]
impl crate::Writable for GMAC0_PAD_BIST_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMAC0_PAD_BIST_INT_ENA to value 0"]
impl crate::Resettable for GMAC0_PAD_BIST_INT_ENA_SPEC {}
