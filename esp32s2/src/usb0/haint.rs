///Register `HAINT` reader
pub type R = crate::R<HAINT_SPEC>;
///Field `HAINT` reader -
pub type HAINT_R = crate::FieldReader;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn haint(&self) -> HAINT_R {
        HAINT_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HAINT")
            .field("haint", &self.haint())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`haint::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HAINT_SPEC;
impl crate::RegisterSpec for HAINT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`haint::R`](R) reader structure
impl crate::Readable for HAINT_SPEC {}
///`reset()` method sets HAINT to value 0
impl crate::Resettable for HAINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
