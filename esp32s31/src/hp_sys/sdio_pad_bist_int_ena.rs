#[doc = "Register `SDIO_PAD_BIST_INT_ENA` reader"]
pub type R = crate::R<SDIO_PAD_BIST_INT_ENA_SPEC>;
#[doc = "Register `SDIO_PAD_BIST_INT_ENA` writer"]
pub type W = crate::W<SDIO_PAD_BIST_INT_ENA_SPEC>;
#[doc = "Field `SDIO_PAD_BIST_OK_INT_ENA` reader - Write 1 to enable sdio pad bist ok interrupt"]
pub type SDIO_PAD_BIST_OK_INT_ENA_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_OK_INT_ENA` writer - Write 1 to enable sdio pad bist ok interrupt"]
pub type SDIO_PAD_BIST_OK_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_PAD_BIST_FAIL_INT_ENA` reader - Write 1 to enable sdio pad bist fail interrupt"]
pub type SDIO_PAD_BIST_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `SDIO_PAD_BIST_FAIL_INT_ENA` writer - Write 1 to enable sdio pad bist fail interrupt"]
pub type SDIO_PAD_BIST_FAIL_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable sdio pad bist ok interrupt"]
    #[inline(always)]
    pub fn sdio_pad_bist_ok_int_ena(&self) -> SDIO_PAD_BIST_OK_INT_ENA_R {
        SDIO_PAD_BIST_OK_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable sdio pad bist fail interrupt"]
    #[inline(always)]
    pub fn sdio_pad_bist_fail_int_ena(&self) -> SDIO_PAD_BIST_FAIL_INT_ENA_R {
        SDIO_PAD_BIST_FAIL_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_PAD_BIST_INT_ENA")
            .field("sdio_pad_bist_ok_int_ena", &self.sdio_pad_bist_ok_int_ena())
            .field(
                "sdio_pad_bist_fail_int_ena",
                &self.sdio_pad_bist_fail_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable sdio pad bist ok interrupt"]
    #[inline(always)]
    pub fn sdio_pad_bist_ok_int_ena(
        &mut self,
    ) -> SDIO_PAD_BIST_OK_INT_ENA_W<'_, SDIO_PAD_BIST_INT_ENA_SPEC> {
        SDIO_PAD_BIST_OK_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable sdio pad bist fail interrupt"]
    #[inline(always)]
    pub fn sdio_pad_bist_fail_int_ena(
        &mut self,
    ) -> SDIO_PAD_BIST_FAIL_INT_ENA_W<'_, SDIO_PAD_BIST_INT_ENA_SPEC> {
        SDIO_PAD_BIST_FAIL_INT_ENA_W::new(self, 1)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_pad_bist_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_pad_bist_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_PAD_BIST_INT_ENA_SPEC;
impl crate::RegisterSpec for SDIO_PAD_BIST_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_pad_bist_int_ena::R`](R) reader structure"]
impl crate::Readable for SDIO_PAD_BIST_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_pad_bist_int_ena::W`](W) writer structure"]
impl crate::Writable for SDIO_PAD_BIST_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDIO_PAD_BIST_INT_ENA to value 0"]
impl crate::Resettable for SDIO_PAD_BIST_INT_ENA_SPEC {}
