#[doc = "Register `FILTER_B_VAL` reader"]
pub type R = crate::R<FILTER_B_VAL_SPEC>;
#[doc = "Register `FILTER_B_VAL` writer"]
pub type W = crate::W<FILTER_B_VAL_SPEC>;
#[doc = "Field `BIT_VAL_B_VAL` reader - Filter B value. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If filter A is not present, writes to this register have no effect and read will return all zeroes."]
pub type BIT_VAL_B_VAL_R = crate::FieldReader<u32>;
#[doc = "Field `BIT_VAL_B_VAL` writer - Filter B value. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If filter A is not present, writes to this register have no effect and read will return all zeroes."]
pub type BIT_VAL_B_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Filter B value. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If filter A is not present, writes to this register have no effect and read will return all zeroes."]
    #[inline(always)]
    pub fn bit_val_b_val(&self) -> BIT_VAL_B_VAL_R {
        BIT_VAL_B_VAL_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_B_VAL")
            .field("bit_val_b_val", &self.bit_val_b_val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - Filter B value. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If filter A is not present, writes to this register have no effect and read will return all zeroes."]
    #[inline(always)]
    pub fn bit_val_b_val(&mut self) -> BIT_VAL_B_VAL_W<'_, FILTER_B_VAL_SPEC> {
        BIT_VAL_B_VAL_W::new(self, 0)
    }
}
#[doc = "TWAI FD filter B bit value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_b_val::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_b_val::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_B_VAL_SPEC;
impl crate::RegisterSpec for FILTER_B_VAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_b_val::R`](R) reader structure"]
impl crate::Readable for FILTER_B_VAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_b_val::W`](W) writer structure"]
impl crate::Writable for FILTER_B_VAL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTER_B_VAL to value 0"]
impl crate::Resettable for FILTER_B_VAL_SPEC {}
