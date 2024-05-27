///Register `ENABLE` reader
pub type R = crate::R<ENABLE_SPEC>;
///Register `ENABLE` writer
pub type W = crate::W<ENABLE_SPEC>;
///Field `DATA` reader - GPIO output enable register for GPIO0-24
pub type DATA_R = crate::FieldReader<u32>;
///Field `DATA` writer - GPIO output enable register for GPIO0-24
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - GPIO output enable register for GPIO0-24
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE")
            .field("data", &self.data())
            .finish()
    }
}
impl W {
    ///Bits 0:24 - GPIO output enable register for GPIO0-24
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<ENABLE_SPEC> {
        DATA_W::new(self, 0)
    }
}
/**GPIO output enable register

You can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`enable::R`](R) reader structure
impl crate::Readable for ENABLE_SPEC {}
///`write(|w| ..)` method takes [`enable::W`](W) writer structure
impl crate::Writable for ENABLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ENABLE to value 0
impl crate::Resettable for ENABLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
