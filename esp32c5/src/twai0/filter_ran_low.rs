#[doc = "Register `FILTER_RAN_LOW` reader"]
pub type R = crate::R<FILTER_RAN_LOW_SPEC>;
#[doc = "Register `FILTER_RAN_LOW` writer"]
pub type W = crate::W<FILTER_RAN_LOW_SPEC>;
#[doc = "Field `BIT_RAN_LOW_VAL` reader - Filter Range Low threshold. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If Range filter is not supported, writes to this register have no effect and read will return all Zeroes."]
pub type BIT_RAN_LOW_VAL_R = crate::FieldReader<u32>;
#[doc = "Field `BIT_RAN_LOW_VAL` writer - Filter Range Low threshold. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If Range filter is not supported, writes to this register have no effect and read will return all Zeroes."]
pub type BIT_RAN_LOW_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Filter Range Low threshold. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If Range filter is not supported, writes to this register have no effect and read will return all Zeroes."]
    #[inline(always)]
    pub fn bit_ran_low_val(&self) -> BIT_RAN_LOW_VAL_R {
        BIT_RAN_LOW_VAL_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_RAN_LOW")
            .field("bit_ran_low_val", &self.bit_ran_low_val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - Filter Range Low threshold. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If Range filter is not supported, writes to this register have no effect and read will return all Zeroes."]
    #[inline(always)]
    pub fn bit_ran_low_val(&mut self) -> BIT_RAN_LOW_VAL_W<FILTER_RAN_LOW_SPEC> {
        BIT_RAN_LOW_VAL_W::new(self, 0)
    }
}
#[doc = "TWAI FD filter range low value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_ran_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_ran_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_RAN_LOW_SPEC;
impl crate::RegisterSpec for FILTER_RAN_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_ran_low::R`](R) reader structure"]
impl crate::Readable for FILTER_RAN_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_ran_low::W`](W) writer structure"]
impl crate::Writable for FILTER_RAN_LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_RAN_LOW to value 0"]
impl crate::Resettable for FILTER_RAN_LOW_SPEC {}
