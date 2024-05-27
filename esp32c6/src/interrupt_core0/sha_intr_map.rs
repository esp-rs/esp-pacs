///Register `SHA_INTR_MAP` reader
pub type R = crate::R<SHA_INTR_MAP_SPEC>;
///Register `SHA_INTR_MAP` writer
pub type W = crate::W<SHA_INTR_MAP_SPEC>;
///Field `SHA_INTR_MAP` reader - Need add description
pub type SHA_INTR_MAP_R = crate::FieldReader;
///Field `SHA_INTR_MAP` writer - Need add description
pub type SHA_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    pub fn sha_intr_map(&self) -> SHA_INTR_MAP_R {
        SHA_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHA_INTR_MAP")
            .field("sha_intr_map", &self.sha_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn sha_intr_map(&mut self) -> SHA_INTR_MAP_W<SHA_INTR_MAP_SPEC> {
        SHA_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`sha_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SHA_INTR_MAP_SPEC;
impl crate::RegisterSpec for SHA_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sha_intr_map::R`](R) reader structure
impl crate::Readable for SHA_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`sha_intr_map::W`](W) writer structure
impl crate::Writable for SHA_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SHA_INTR_MAP to value 0
impl crate::Resettable for SHA_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
