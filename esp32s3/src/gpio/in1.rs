///Register `IN1` reader
pub type R = crate::R<IN1_SPEC>;
///Register `IN1` writer
pub type W = crate::W<IN1_SPEC>;
///Field `DATA_NEXT` reader - GPIO input register for GPIO32-53
pub type DATA_NEXT_R = crate::FieldReader<u32>;
///Field `DATA_NEXT` writer - GPIO input register for GPIO32-53
pub type DATA_NEXT_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    ///Bits 0:21 - GPIO input register for GPIO32-53
    #[inline(always)]
    pub fn data_next(&self) -> DATA_NEXT_R {
        DATA_NEXT_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN1")
            .field("data_next", &self.data_next())
            .finish()
    }
}
impl W {
    ///Bits 0:21 - GPIO input register for GPIO32-53
    #[inline(always)]
    #[must_use]
    pub fn data_next(&mut self) -> DATA_NEXT_W<IN1_SPEC> {
        DATA_NEXT_W::new(self, 0)
    }
}
/**GPIO input register for GPIO32-53

You can [`read`](crate::generic::Reg::read) this register and get [`in1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IN1_SPEC;
impl crate::RegisterSpec for IN1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`in1::R`](R) reader structure
impl crate::Readable for IN1_SPEC {}
///`write(|w| ..)` method takes [`in1::W`](W) writer structure
impl crate::Writable for IN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IN1 to value 0
impl crate::Resettable for IN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
