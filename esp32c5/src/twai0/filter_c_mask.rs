#[doc = "Register `FILTER_C_MASK` reader"]
pub type R = crate::R<FILTER_C_MASK_SPEC>;
#[doc = "Register `FILTER_C_MASK` writer"]
pub type W = crate::W<FILTER_C_MASK_SPEC>;
#[doc = "Field `BIT_MASK_C_VAL` reader - Filter C mask. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If filter A is not present, writes to this register have no effect and read will return all zeroes."]
pub type BIT_MASK_C_VAL_R = crate::FieldReader<u32>;
#[doc = "Field `BIT_MASK_C_VAL` writer - Filter C mask. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If filter A is not present, writes to this register have no effect and read will return all zeroes."]
pub type BIT_MASK_C_VAL_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28 - Filter C mask. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If filter A is not present, writes to this register have no effect and read will return all zeroes."]
    #[inline(always)]
    pub fn bit_mask_c_val(&self) -> BIT_MASK_C_VAL_R {
        BIT_MASK_C_VAL_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FILTER_C_MASK")
            .field("bit_mask_c_val", &self.bit_mask_c_val())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28 - Filter C mask. The identifier format is the same as in IDENTIFIER_W of TXT buffer or RX buffer. If filter A is not present, writes to this register have no effect and read will return all zeroes."]
    #[inline(always)]
    pub fn bit_mask_c_val(&mut self) -> BIT_MASK_C_VAL_W<FILTER_C_MASK_SPEC> {
        BIT_MASK_C_VAL_W::new(self, 0)
    }
}
#[doc = "TWAI FD filter C mask value register\n\nYou can [`read`](crate::Reg::read) this register and get [`filter_c_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filter_c_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FILTER_C_MASK_SPEC;
impl crate::RegisterSpec for FILTER_C_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filter_c_mask::R`](R) reader structure"]
impl crate::Readable for FILTER_C_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`filter_c_mask::W`](W) writer structure"]
impl crate::Writable for FILTER_C_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FILTER_C_MASK to value 0"]
impl crate::Resettable for FILTER_C_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
