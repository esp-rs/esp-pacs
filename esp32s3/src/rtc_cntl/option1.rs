///Register `OPTION1` reader
pub type R = crate::R<OPTION1_SPEC>;
///Register `OPTION1` writer
pub type W = crate::W<OPTION1_SPEC>;
///Field `FORCE_DOWNLOAD_BOOT` reader - force chip entry download boot by sw
pub type FORCE_DOWNLOAD_BOOT_R = crate::BitReader;
///Field `FORCE_DOWNLOAD_BOOT` writer - force chip entry download boot by sw
pub type FORCE_DOWNLOAD_BOOT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - force chip entry download boot by sw
    #[inline(always)]
    pub fn force_download_boot(&self) -> FORCE_DOWNLOAD_BOOT_R {
        FORCE_DOWNLOAD_BOOT_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OPTION1")
            .field("force_download_boot", &self.force_download_boot())
            .finish()
    }
}
impl W {
    ///Bit 0 - force chip entry download boot by sw
    #[inline(always)]
    #[must_use]
    pub fn force_download_boot(&mut self) -> FORCE_DOWNLOAD_BOOT_W<OPTION1_SPEC> {
        FORCE_DOWNLOAD_BOOT_W::new(self, 0)
    }
}
/**rtc common configure

You can [`read`](crate::generic::Reg::read) this register and get [`option1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`option1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OPTION1_SPEC;
impl crate::RegisterSpec for OPTION1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`option1::R`](R) reader structure
impl crate::Readable for OPTION1_SPEC {}
///`write(|w| ..)` method takes [`option1::W`](W) writer structure
impl crate::Writable for OPTION1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OPTION1 to value 0
impl crate::Resettable for OPTION1_SPEC {
    const RESET_VALUE: u32 = 0;
}
