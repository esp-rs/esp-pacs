///Register `SWD_WPROTECT` reader
pub type R = crate::R<SWD_WPROTECT_SPEC>;
///Register `SWD_WPROTECT` writer
pub type W = crate::W<SWD_WPROTECT_SPEC>;
///Field `SWD_WKEY` reader - need_des
pub type SWD_WKEY_R = crate::FieldReader<u32>;
///Field `SWD_WKEY` writer - need_des
pub type SWD_WKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn swd_wkey(&self) -> SWD_WKEY_R {
        SWD_WKEY_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWD_WPROTECT")
            .field("swd_wkey", &self.swd_wkey())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn swd_wkey(&mut self) -> SWD_WKEY_W<SWD_WPROTECT_SPEC> {
        SWD_WKEY_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`swd_wprotect::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swd_wprotect::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SWD_WPROTECT_SPEC;
impl crate::RegisterSpec for SWD_WPROTECT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`swd_wprotect::R`](R) reader structure
impl crate::Readable for SWD_WPROTECT_SPEC {}
///`write(|w| ..)` method takes [`swd_wprotect::W`](W) writer structure
impl crate::Writable for SWD_WPROTECT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SWD_WPROTECT to value 0
impl crate::Resettable for SWD_WPROTECT_SPEC {
    const RESET_VALUE: u32 = 0;
}
