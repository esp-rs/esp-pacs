///Register `SCRAMBLING` reader
pub type R = crate::R<SCRAMBLING_SPEC>;
///Register `SCRAMBLING` writer
pub type W = crate::W<SCRAMBLING_SPEC>;
///Field `SCRAMBLE_ENABLE` reader - NA
pub type SCRAMBLE_ENABLE_R = crate::BitReader;
///Field `SCRAMBLE_ENABLE` writer - NA
pub type SCRAMBLE_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn scramble_enable(&self) -> SCRAMBLE_ENABLE_R {
        SCRAMBLE_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCRAMBLING")
            .field("scramble_enable", &self.scramble_enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn scramble_enable(&mut self) -> SCRAMBLE_ENABLE_W<SCRAMBLING_SPEC> {
        SCRAMBLE_ENABLE_W::new(self, 0)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`scrambling::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scrambling::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SCRAMBLING_SPEC;
impl crate::RegisterSpec for SCRAMBLING_SPEC {
    type Ux = u32;
}
///`read()` method returns [`scrambling::R`](R) reader structure
impl crate::Readable for SCRAMBLING_SPEC {}
///`write(|w| ..)` method takes [`scrambling::W`](W) writer structure
impl crate::Writable for SCRAMBLING_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SCRAMBLING to value 0
impl crate::Resettable for SCRAMBLING_SPEC {
    const RESET_VALUE: u32 = 0;
}
