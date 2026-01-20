#[doc = "Register `BLEBDADDRL` reader"]
pub type R = crate::R<BLEBDADDRL_SPEC>;
#[doc = "Register `BLEBDADDRL` writer"]
pub type W = crate::W<BLEBDADDRL_SPEC>;
#[doc = "Field `BDADDRL` reader - "]
pub type BDADDRL_R = crate::FieldReader<u32>;
#[doc = "Field `BDADDRL` writer - "]
pub type BDADDRL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bdaddrl(&self) -> BDADDRL_R {
        BDADDRL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEBDADDRL")
            .field("bdaddrl", &self.bdaddrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn bdaddrl(&mut self) -> BDADDRL_W<'_, BLEBDADDRL_SPEC> {
        BDADDRL_W::new(self, 0)
    }
}
#[doc = "BLE device address LSB register\n\nYou can [`read`](crate::Reg::read) this register and get [`blebdaddrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blebdaddrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEBDADDRL_SPEC;
impl crate::RegisterSpec for BLEBDADDRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blebdaddrl::R`](R) reader structure"]
impl crate::Readable for BLEBDADDRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blebdaddrl::W`](W) writer structure"]
impl crate::Writable for BLEBDADDRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEBDADDRL to value 0"]
impl crate::Resettable for BLEBDADDRL_SPEC {}
