///Register `LOAD_HI` reader
pub type R = crate::R<LOAD_HI_SPEC>;
///Register `LOAD_HI` writer
pub type W = crate::W<LOAD_HI_SPEC>;
///Field `LOAD_HI` reader - The value to be loaded into system timer, high 32 bits.
pub type LOAD_HI_R = crate::FieldReader<u32>;
///Field `LOAD_HI` writer - The value to be loaded into system timer, high 32 bits.
pub type LOAD_HI_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32, crate::Safe>;
impl R {
    ///Bits 0:31 - The value to be loaded into system timer, high 32 bits.
    #[inline(always)]
    pub fn load_hi(&self) -> LOAD_HI_R {
        LOAD_HI_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOAD_HI")
            .field("load_hi", &self.load_hi())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The value to be loaded into system timer, high 32 bits.
    #[inline(always)]
    #[must_use]
    pub fn load_hi(&mut self) -> LOAD_HI_W<LOAD_HI_SPEC> {
        LOAD_HI_W::new(self, 0)
    }
}
/**High 32 bits to be loaded to system timer

You can [`read`](crate::generic::Reg::read) this register and get [`load_hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`load_hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOAD_HI_SPEC;
impl crate::RegisterSpec for LOAD_HI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`load_hi::R`](R) reader structure
impl crate::Readable for LOAD_HI_SPEC {}
///`write(|w| ..)` method takes [`load_hi::W`](W) writer structure
impl crate::Writable for LOAD_HI_SPEC {
    type Safety = crate::Safe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOAD_HI to value 0
impl crate::Resettable for LOAD_HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
