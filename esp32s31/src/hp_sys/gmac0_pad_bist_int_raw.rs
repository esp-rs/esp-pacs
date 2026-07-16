#[doc = "Register `GMAC0_PAD_BIST_INT_RAW` reader"]
pub type R = crate::R<GMAC0_PAD_BIST_INT_RAW_SPEC>;
#[doc = "Register `GMAC0_PAD_BIST_INT_RAW` writer"]
pub type W = crate::W<GMAC0_PAD_BIST_INT_RAW_SPEC>;
#[doc = "Field `GMAC0_PAD_BIST_OK_INT_RAW` reader - intr triggered when bist finish and status is bist ok"]
pub type GMAC0_PAD_BIST_OK_INT_RAW_R = crate::BitReader;
#[doc = "Field `GMAC0_PAD_BIST_OK_INT_RAW` writer - intr triggered when bist finish and status is bist ok"]
pub type GMAC0_PAD_BIST_OK_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GMAC0_PAD_BIST_FAIL_INT_RAW` reader - intr triggered when bist finish and status is bist fail"]
pub type GMAC0_PAD_BIST_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `GMAC0_PAD_BIST_FAIL_INT_RAW` writer - intr triggered when bist finish and status is bist fail"]
pub type GMAC0_PAD_BIST_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - intr triggered when bist finish and status is bist ok"]
    #[inline(always)]
    pub fn gmac0_pad_bist_ok_int_raw(&self) -> GMAC0_PAD_BIST_OK_INT_RAW_R {
        GMAC0_PAD_BIST_OK_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - intr triggered when bist finish and status is bist fail"]
    #[inline(always)]
    pub fn gmac0_pad_bist_fail_int_raw(&self) -> GMAC0_PAD_BIST_FAIL_INT_RAW_R {
        GMAC0_PAD_BIST_FAIL_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GMAC0_PAD_BIST_INT_RAW")
            .field(
                "gmac0_pad_bist_ok_int_raw",
                &self.gmac0_pad_bist_ok_int_raw(),
            )
            .field(
                "gmac0_pad_bist_fail_int_raw",
                &self.gmac0_pad_bist_fail_int_raw(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - intr triggered when bist finish and status is bist ok"]
    #[inline(always)]
    pub fn gmac0_pad_bist_ok_int_raw(
        &mut self,
    ) -> GMAC0_PAD_BIST_OK_INT_RAW_W<'_, GMAC0_PAD_BIST_INT_RAW_SPEC> {
        GMAC0_PAD_BIST_OK_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - intr triggered when bist finish and status is bist fail"]
    #[inline(always)]
    pub fn gmac0_pad_bist_fail_int_raw(
        &mut self,
    ) -> GMAC0_PAD_BIST_FAIL_INT_RAW_W<'_, GMAC0_PAD_BIST_INT_RAW_SPEC> {
        GMAC0_PAD_BIST_FAIL_INT_RAW_W::new(self, 1)
    }
}
#[doc = "gmac0 pad bist interupt raw register\n\nYou can [`read`](crate::Reg::read) this register and get [`gmac0_pad_bist_int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gmac0_pad_bist_int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GMAC0_PAD_BIST_INT_RAW_SPEC;
impl crate::RegisterSpec for GMAC0_PAD_BIST_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac0_pad_bist_int_raw::R`](R) reader structure"]
impl crate::Readable for GMAC0_PAD_BIST_INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gmac0_pad_bist_int_raw::W`](W) writer structure"]
impl crate::Writable for GMAC0_PAD_BIST_INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GMAC0_PAD_BIST_INT_RAW to value 0"]
impl crate::Resettable for GMAC0_PAD_BIST_INT_RAW_SPEC {}
