#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `UNDERRUN` reader - the raw interrupt status of dpi_underrun"]
pub type UNDERRUN_R = crate::BitReader;
#[doc = "Field `UNDERRUN` writer - the raw interrupt status of dpi_underrun"]
pub type UNDERRUN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - the raw interrupt status of dpi_underrun"]
    #[inline(always)]
    pub fn underrun(&self) -> UNDERRUN_R {
        UNDERRUN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("underrun", &self.underrun())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - the raw interrupt status of dpi_underrun"]
    #[inline(always)]
    pub fn underrun(&mut self) -> UNDERRUN_W<'_, INT_RAW_SPEC> {
        UNDERRUN_W::new(self, 0)
    }
}
#[doc = "dsi_bridge raw interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {}
