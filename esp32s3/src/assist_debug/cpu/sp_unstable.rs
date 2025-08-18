#[doc = "Register `SP_UNSTABLE` reader"]
pub type R = crate::R<SP_UNSTABLE_SPEC>;
#[doc = "Register `SP_UNSTABLE` writer"]
pub type W = crate::W<SP_UNSTABLE_SPEC>;
#[doc = "Field `SP_UNSTABLE` reader - unstable period when window change,during this period no check stackpointer"]
pub type SP_UNSTABLE_R = crate::FieldReader;
#[doc = "Field `SP_UNSTABLE` writer - unstable period when window change,during this period no check stackpointer"]
pub type SP_UNSTABLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - unstable period when window change,during this period no check stackpointer"]
    #[inline(always)]
    pub fn sp_unstable(&self) -> SP_UNSTABLE_R {
        SP_UNSTABLE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SP_UNSTABLE")
            .field("sp_unstable", &self.sp_unstable())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - unstable period when window change,during this period no check stackpointer"]
    #[inline(always)]
    pub fn sp_unstable(&mut self) -> SP_UNSTABLE_W<'_, SP_UNSTABLE_SPEC> {
        SP_UNSTABLE_W::new(self, 0)
    }
}
#[doc = "core0 sp unstable configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sp_unstable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sp_unstable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SP_UNSTABLE_SPEC;
impl crate::RegisterSpec for SP_UNSTABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sp_unstable::R`](R) reader structure"]
impl crate::Readable for SP_UNSTABLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sp_unstable::W`](W) writer structure"]
impl crate::Writable for SP_UNSTABLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SP_UNSTABLE to value 0x0b"]
impl crate::Resettable for SP_UNSTABLE_SPEC {
    const RESET_VALUE: u32 = 0x0b;
}
