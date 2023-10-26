#[doc = "Register `DATA_7` reader"]
pub type R = crate::R<DATA_7_SPEC>;
#[doc = "Register `DATA_7` writer"]
pub type W = crate::W<DATA_7_SPEC>;
#[doc = "Field `TX_BYTE_7` reader - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, it stores the 7th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_7_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_7` writer - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, it stores the 7th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, it stores the 7th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte_7(&self) -> TX_BYTE_7_R {
        TX_BYTE_7_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_7")
            .field("tx_byte_7", &format_args!("{}", self.tx_byte_7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 3 with R/W Permission. In operation mode, it stores the 7th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_7(&mut self) -> TX_BYTE_7_W<DATA_7_SPEC, 0> {
        TX_BYTE_7_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Data register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_7_SPEC;
impl crate::RegisterSpec for DATA_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_7::R`](R) reader structure"]
impl crate::Readable for DATA_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_7::W`](W) writer structure"]
impl crate::Writable for DATA_7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_7 to value 0"]
impl crate::Resettable for DATA_7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
