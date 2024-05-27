///Register `OUT_ENABLE` reader
pub type R = crate::R<OUT_ENABLE_SPEC>;
///Register `OUT_ENABLE` writer
pub type W = crate::W<OUT_ENABLE_SPEC>;
///Field `ENABLE` reader - set lp gpio output data
pub type ENABLE_R = crate::FieldReader;
///Field `ENABLE` writer - set lp gpio output data
pub type ENABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - set lp gpio output data
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_ENABLE")
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - set lp gpio output data
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<OUT_ENABLE_SPEC> {
        ENABLE_W::new(self, 0)
    }
}
/**need des

You can [`read`](crate::generic::Reg::read) this register and get [`out_enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_ENABLE_SPEC;
impl crate::RegisterSpec for OUT_ENABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_enable::R`](R) reader structure
impl crate::Readable for OUT_ENABLE_SPEC {}
///`write(|w| ..)` method takes [`out_enable::W`](W) writer structure
impl crate::Writable for OUT_ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUT_ENABLE to value 0
impl crate::Resettable for OUT_ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
