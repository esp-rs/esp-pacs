#[doc = "Register `_3_SHAKE_LENGTH` reader"]
pub type R = crate::R<_3_SHAKE_LENGTH_SPEC>;
#[doc = "Register `_3_SHAKE_LENGTH` writer"]
pub type W = crate::W<_3_SHAKE_LENGTH_SPEC>;
#[doc = "Field `_3_SHAKE_LENGTH` reader - SHAKE output hash word length"]
pub type _3_SHAKE_LENGTH_R = crate::FieldReader<u32>;
#[doc = "Field `_3_SHAKE_LENGTH` writer - SHAKE output hash word length"]
pub type _3_SHAKE_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:20 - SHAKE output hash word length"]
    #[inline(always)]
    pub fn _3_shake_length(&self) -> _3_SHAKE_LENGTH_R {
        _3_SHAKE_LENGTH_R::new(self.bits & 0x001f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_3_SHAKE_LENGTH")
            .field("_3_shake_length", &self._3_shake_length())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:20 - SHAKE output hash word length"]
    #[inline(always)]
    pub fn _3_shake_length(&mut self) -> _3_SHAKE_LENGTH_W<'_, _3_SHAKE_LENGTH_SPEC> {
        _3_SHAKE_LENGTH_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_shake_length::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_shake_length::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_SHAKE_LENGTH_SPEC;
impl crate::RegisterSpec for _3_SHAKE_LENGTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_shake_length::R`](R) reader structure"]
impl crate::Readable for _3_SHAKE_LENGTH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_shake_length::W`](W) writer structure"]
impl crate::Writable for _3_SHAKE_LENGTH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _3_SHAKE_LENGTH to value 0x32"]
impl crate::Resettable for _3_SHAKE_LENGTH_SPEC {
    const RESET_VALUE: u32 = 0x32;
}
