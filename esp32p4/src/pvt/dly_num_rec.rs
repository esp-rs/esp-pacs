#[doc = "Register `DLY_NUM_REC%s` reader"]
pub type R = crate::R<DLY_NUM_REC_SPEC>;
#[doc = "Register `DLY_NUM_REC%s` writer"]
pub type W = crate::W<DLY_NUM_REC_SPEC>;
#[doc = "Field `VAL` reader - "]
pub type VAL_R = crate::FieldReader<u32>;
#[doc = "Field `VAL` writer - "]
pub type VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLY_NUM_REC")
            .field("val", &self.val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn val(&mut self) -> VAL_W<'_, DLY_NUM_REC_SPEC> {
        VAL_W::new(self, 0)
    }
}
#[doc = "Delay-chain capture %s\n\nYou can [`read`](crate::Reg::read) this register and get [`dly_num_rec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly_num_rec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLY_NUM_REC_SPEC;
impl crate::RegisterSpec for DLY_NUM_REC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dly_num_rec::R`](R) reader structure"]
impl crate::Readable for DLY_NUM_REC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dly_num_rec::W`](W) writer structure"]
impl crate::Writable for DLY_NUM_REC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLY_NUM_REC%s to value 0"]
impl crate::Resettable for DLY_NUM_REC_SPEC {}
