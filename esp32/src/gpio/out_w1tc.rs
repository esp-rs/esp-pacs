#[doc = "Register `OUT_W1TC` reader"]
pub type R = crate::R<OUT_W1TC_SPEC>;
#[doc = "Register `OUT_W1TC` writer"]
pub type W = crate::W<OUT_W1TC_SPEC>;
#[doc = "Field `OUT_DATA_W1TC` reader - GPIO0~31 output value write 1 to clear"]
pub type OUT_DATA_W1TC_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DATA_W1TC` writer - GPIO0~31 output value write 1 to clear"]
pub type OUT_DATA_W1TC_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - GPIO0~31 output value write 1 to clear"]
    #[inline(always)]
    pub fn out_data_w1tc(&self) -> OUT_DATA_W1TC_R {
        OUT_DATA_W1TC_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_W1TC")
            .field("out_data_w1tc", &self.out_data_w1tc())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - GPIO0~31 output value write 1 to clear"]
    #[inline(always)]
    #[must_use]
    pub fn out_data_w1tc(&mut self) -> OUT_DATA_W1TC_W<OUT_W1TC_SPEC> {
        OUT_DATA_W1TC_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_w1tc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_W1TC_SPEC;
impl crate::RegisterSpec for OUT_W1TC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_w1tc::R`](R) reader structure"]
impl crate::Readable for OUT_W1TC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_w1tc::W`](W) writer structure"]
impl crate::Writable for OUT_W1TC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_W1TC to value 0"]
impl crate::Resettable for OUT_W1TC_SPEC {
    const RESET_VALUE: u32 = 0;
}
