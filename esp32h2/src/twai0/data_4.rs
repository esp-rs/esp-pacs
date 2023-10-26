#[doc = "Register `DATA_4` reader"]
pub type R = crate::R<DATA_4_SPEC>;
#[doc = "Register `DATA_4` writer"]
pub type W = crate::W<DATA_4_SPEC>;
#[doc = "Field `DATA_4` reader - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4."]
pub type DATA_4_R = crate::FieldReader;
#[doc = "Field `DATA_4` writer - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4."]
pub type DATA_4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4."]
    #[inline(always)]
    pub fn data_4(&self) -> DATA_4_R {
        DATA_4_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_4")
            .field("data_4", &format_args!("{}", self.data_4().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 0 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 4 and when software initiate read operation, it is rx data register 4."]
    #[inline(always)]
    #[must_use]
    pub fn data_4(&mut self) -> DATA_4_W<DATA_4_SPEC, 0> {
        DATA_4_W::new(self)
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
#[doc = "Data register 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_4_SPEC;
impl crate::RegisterSpec for DATA_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_4::R`](R) reader structure"]
impl crate::Readable for DATA_4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_4::W`](W) writer structure"]
impl crate::Writable for DATA_4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_4 to value 0"]
impl crate::Resettable for DATA_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
