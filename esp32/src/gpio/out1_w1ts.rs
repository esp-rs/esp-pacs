#[doc = "Register `OUT1_W1TS` reader"]
pub type R = crate::R<OUT1_W1TS_SPEC>;
#[doc = "Register `OUT1_W1TS` writer"]
pub type W = crate::W<OUT1_W1TS_SPEC>;
#[doc = "Field `OUT1_DATA_W1TS` reader - GPIO32~39 output value write 1 to set"]
pub type OUT1_DATA_W1TS_R = crate::FieldReader;
#[doc = "Field `OUT1_DATA_W1TS` writer - GPIO32~39 output value write 1 to set"]
pub type OUT1_DATA_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 output value write 1 to set"]
    #[inline(always)]
    pub fn out1_data_w1ts(&self) -> OUT1_DATA_W1TS_R {
        OUT1_DATA_W1TS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT1_W1TS")
            .field("out1_data_w1ts", &self.out1_data_w1ts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 output value write 1 to set"]
    #[inline(always)]
    #[must_use]
    pub fn out1_data_w1ts(&mut self) -> OUT1_DATA_W1TS_W<OUT1_W1TS_SPEC> {
        OUT1_DATA_W1TS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out1_w1ts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT1_W1TS_SPEC;
impl crate::RegisterSpec for OUT1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out1_w1ts::R`](R) reader structure"]
impl crate::Readable for OUT1_W1TS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out1_w1ts::W`](W) writer structure"]
impl crate::Writable for OUT1_W1TS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT1_W1TS to value 0"]
impl crate::Resettable for OUT1_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
