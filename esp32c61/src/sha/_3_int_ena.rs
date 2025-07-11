#[doc = "Register `_3_INT_ENA` reader"]
pub type R = crate::R<_3_INT_ENA_SPEC>;
#[doc = "Register `_3_INT_ENA` writer"]
pub type W = crate::W<_3_INT_ENA_SPEC>;
#[doc = "Field `_3_INT_ENA` reader - Sha3 interrupt enable register. 1'b0: disable(default). 1'b1:enable"]
pub type _3_INT_ENA_R = crate::BitReader;
#[doc = "Field `_3_INT_ENA` writer - Sha3 interrupt enable register. 1'b0: disable(default). 1'b1:enable"]
pub type _3_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Sha3 interrupt enable register. 1'b0: disable(default). 1'b1:enable"]
    #[inline(always)]
    pub fn _3_int_ena(&self) -> _3_INT_ENA_R {
        _3_INT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_3_INT_ENA")
            .field("_3_int_ena", &self._3_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Sha3 interrupt enable register. 1'b0: disable(default). 1'b1:enable"]
    #[inline(always)]
    pub fn _3_int_ena(&mut self) -> _3_INT_ENA_W<_3_INT_ENA_SPEC> {
        _3_INT_ENA_W::new(self, 0)
    }
}
#[doc = "Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_INT_ENA_SPEC;
impl crate::RegisterSpec for _3_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_int_ena::R`](R) reader structure"]
impl crate::Readable for _3_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_int_ena::W`](W) writer structure"]
impl crate::Writable for _3_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _3_INT_ENA to value 0"]
impl crate::Resettable for _3_INT_ENA_SPEC {}
