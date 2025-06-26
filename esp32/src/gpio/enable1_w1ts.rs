#[doc = "Register `ENABLE1_W1TS` reader"]
pub type R = crate::R<ENABLE1_W1TS_SPEC>;
#[doc = "Register `ENABLE1_W1TS` writer"]
pub type W = crate::W<ENABLE1_W1TS_SPEC>;
#[doc = "Field `ENABLE1_DATA_W1TS` reader - GPIO32~39 output enable write 1 to set"]
pub type ENABLE1_DATA_W1TS_R = crate::FieldReader;
#[doc = "Field `ENABLE1_DATA_W1TS` writer - GPIO32~39 output enable write 1 to set"]
pub type ENABLE1_DATA_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 output enable write 1 to set"]
    #[inline(always)]
    pub fn enable1_data_w1ts(&self) -> ENABLE1_DATA_W1TS_R {
        ENABLE1_DATA_W1TS_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENABLE1_W1TS")
            .field("enable1_data_w1ts", &self.enable1_data_w1ts())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 output enable write 1 to set"]
    #[inline(always)]
    pub fn enable1_data_w1ts(&mut self) -> ENABLE1_DATA_W1TS_W<ENABLE1_W1TS_SPEC> {
        ENABLE1_DATA_W1TS_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`enable1_w1ts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable1_w1ts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENABLE1_W1TS_SPEC;
impl crate::RegisterSpec for ENABLE1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable1_w1ts::R`](R) reader structure"]
impl crate::Readable for ENABLE1_W1TS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`enable1_w1ts::W`](W) writer structure"]
impl crate::Writable for ENABLE1_W1TS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENABLE1_W1TS to value 0"]
impl crate::Resettable for ENABLE1_W1TS_SPEC {}
