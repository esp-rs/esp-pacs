#[doc = "Register `EXT3` reader"]
pub type R = crate::R<EXT3_SPEC>;
#[doc = "Register `EXT3` writer"]
pub type W = crate::W<EXT3_SPEC>;
#[doc = "Field `INT_HOLD_ENA` reader - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at ¡°idle¡± phase 2: hold at ¡°prepare¡± phase."]
pub type INT_HOLD_ENA_R = crate::FieldReader;
#[doc = "Field `INT_HOLD_ENA` writer - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at ¡°idle¡± phase 2: hold at ¡°prepare¡± phase."]
pub type INT_HOLD_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at ¡°idle¡± phase 2: hold at ¡°prepare¡± phase."]
    #[inline(always)]
    pub fn int_hold_ena(&self) -> INT_HOLD_ENA_R {
        INT_HOLD_ENA_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT3")
            .field("int_hold_ena", &self.int_hold_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set if the other SPI is busy the SPI will be hold. 1(3): hold at ¡°idle¡± phase 2: hold at ¡°prepare¡± phase."]
    #[inline(always)]
    pub fn int_hold_ena(&mut self) -> INT_HOLD_ENA_W<EXT3_SPEC> {
        INT_HOLD_ENA_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`ext3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT3_SPEC;
impl crate::RegisterSpec for EXT3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext3::R`](R) reader structure"]
impl crate::Readable for EXT3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext3::W`](W) writer structure"]
impl crate::Writable for EXT3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT3 to value 0"]
impl crate::Resettable for EXT3_SPEC {
    const RESET_VALUE: u32 = 0;
}
