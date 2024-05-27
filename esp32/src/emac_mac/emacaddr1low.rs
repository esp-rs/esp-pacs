///Register `EMACADDR1LOW` reader
pub type R = crate::R<EMACADDR1LOW_SPEC>;
///Register `EMACADDR1LOW` writer
pub type W = crate::W<EMACADDR1LOW_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
/**This field contains the lower 32 bits of the second 6-byte MAC address.The content of this field is undefined so the register needs to be configured after the initialization Process.

You can [`read`](crate::generic::Reg::read) this register and get [`emacaddr1low::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emacaddr1low::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EMACADDR1LOW_SPEC;
impl crate::RegisterSpec for EMACADDR1LOW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`emacaddr1low::R`](R) reader structure
impl crate::Readable for EMACADDR1LOW_SPEC {}
///`write(|w| ..)` method takes [`emacaddr1low::W`](W) writer structure
impl crate::Writable for EMACADDR1LOW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EMACADDR1LOW to value 0
impl crate::Resettable for EMACADDR1LOW_SPEC {
    const RESET_VALUE: u32 = 0;
}
