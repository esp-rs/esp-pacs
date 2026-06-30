#[doc = "Register `DBIAS_CMD4` reader"]
pub type R = crate::R<DBIAS_CMD4_SPEC>;
#[doc = "Register `DBIAS_CMD4` writer"]
pub type W = crate::W<DBIAS_CMD4_SPEC>;
#[doc = "Field `DBIAS_CMD4` reader - needs field desc"]
pub type DBIAS_CMD4_R = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CMD4` writer - needs field desc"]
pub type DBIAS_CMD4_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd4(&self) -> DBIAS_CMD4_R {
        DBIAS_CMD4_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_CMD4")
            .field("dbias_cmd4", &self.dbias_cmd4())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd4(&mut self) -> DBIAS_CMD4_W<'_, DBIAS_CMD4_SPEC> {
        DBIAS_CMD4_W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_cmd4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_cmd4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBIAS_CMD4_SPEC;
impl crate::RegisterSpec for DBIAS_CMD4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_cmd4::R`](R) reader structure"]
impl crate::Readable for DBIAS_CMD4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbias_cmd4::W`](W) writer structure"]
impl crate::Writable for DBIAS_CMD4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CMD4 to value 0"]
impl crate::Resettable for DBIAS_CMD4_SPEC {}
