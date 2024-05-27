///Register `STRAP` reader
pub type R = crate::R<STRAP_SPEC>;
///Field `STRAPPING` reader - GPIO strapping values: bit4 ~ bit2 correspond to stripping pins GPIO45, GPIO0, and GPIO46 respectively.
pub type STRAPPING_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - GPIO strapping values: bit4 ~ bit2 correspond to stripping pins GPIO45, GPIO0, and GPIO46 respectively.
    #[inline(always)]
    pub fn strapping(&self) -> STRAPPING_R {
        STRAPPING_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STRAP")
            .field("strapping", &self.strapping())
            .finish()
    }
}
/**Bootstrap pin value register

You can [`read`](crate::generic::Reg::read) this register and get [`strap::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STRAP_SPEC;
impl crate::RegisterSpec for STRAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`strap::R`](R) reader structure
impl crate::Readable for STRAP_SPEC {}
///`reset()` method sets STRAP to value 0
impl crate::Resettable for STRAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
