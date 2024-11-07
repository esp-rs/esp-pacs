#[doc = "Register `HAINTMSK` reader"]
pub type R = crate::R<HAINTMSK_SPEC>;
#[doc = "Register `HAINTMSK` writer"]
pub type W = crate::W<HAINTMSK_SPEC>;
#[doc = "Field `HAINTMSK` reader - "]
pub type HAINTMSK_R = crate::FieldReader;
#[doc = "Field `HAINTMSK` writer - "]
pub type HAINTMSK_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn haintmsk(&self) -> HAINTMSK_R {
        HAINTMSK_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HAINTMSK")
            .field("haintmsk", &self.haintmsk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn haintmsk(&mut self) -> HAINTMSK_W<HAINTMSK_SPEC> {
        HAINTMSK_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`haintmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HAINTMSK_SPEC;
impl crate::RegisterSpec for HAINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`haintmsk::R`](R) reader structure"]
impl crate::Readable for HAINTMSK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`haintmsk::W`](W) writer structure"]
impl crate::Writable for HAINTMSK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HAINTMSK to value 0"]
impl crate::Resettable for HAINTMSK_SPEC {
    const RESET_VALUE: u32 = 0;
}
