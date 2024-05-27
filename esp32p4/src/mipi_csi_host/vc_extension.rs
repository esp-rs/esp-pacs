///Register `VC_EXTENSION` reader
pub type R = crate::R<VC_EXTENSION_SPEC>;
///Register `VC_EXTENSION` writer
pub type W = crate::W<VC_EXTENSION_SPEC>;
///Field `VCX` reader - NA
pub type VCX_R = crate::BitReader;
///Field `VCX` writer - NA
pub type VCX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn vcx(&self) -> VCX_R {
        VCX_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VC_EXTENSION")
            .field("vcx", &self.vcx())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn vcx(&mut self) -> VCX_W<VC_EXTENSION_SPEC> {
        VCX_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`vc_extension::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vc_extension::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct VC_EXTENSION_SPEC;
impl crate::RegisterSpec for VC_EXTENSION_SPEC {
    type Ux = u32;
}
///`read()` method returns [`vc_extension::R`](R) reader structure
impl crate::Readable for VC_EXTENSION_SPEC {}
///`write(|w| ..)` method takes [`vc_extension::W`](W) writer structure
impl crate::Writable for VC_EXTENSION_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets VC_EXTENSION to value 0
impl crate::Resettable for VC_EXTENSION_SPEC {
    const RESET_VALUE: u32 = 0;
}
