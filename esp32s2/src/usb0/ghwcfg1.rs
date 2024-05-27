///Register `GHWCFG1` reader
pub type R = crate::R<GHWCFG1_SPEC>;
///Field `EPDIR` reader -
pub type EPDIR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn epdir(&self) -> EPDIR_R {
        EPDIR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHWCFG1")
            .field("epdir", &self.epdir())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ghwcfg1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GHWCFG1_SPEC;
impl crate::RegisterSpec for GHWCFG1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ghwcfg1::R`](R) reader structure
impl crate::Readable for GHWCFG1_SPEC {}
///`reset()` method sets GHWCFG1 to value 0
impl crate::Resettable for GHWCFG1_SPEC {
    const RESET_VALUE: u32 = 0;
}
