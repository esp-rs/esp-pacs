#[doc = "Register `EDMA_PMS_I2S0` reader"]
pub type R = crate::R<EDMA_PMS_I2S0_SPEC>;
#[doc = "Register `EDMA_PMS_I2S0` writer"]
pub type W = crate::W<EDMA_PMS_I2S0_SPEC>;
#[doc = "Field `ATTR1` reader - This field is used to configure the permission of I2S0 accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR1_R = crate::FieldReader;
#[doc = "Field `ATTR1` writer - This field is used to configure the permission of I2S0 accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `ATTR2` reader - This field is used to configure the permission of I2S0 accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR2_R = crate::FieldReader;
#[doc = "Field `ATTR2` writer - This field is used to configure the permission of I2S0 accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
pub type ATTR2_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:1 - This field is used to configure the permission of I2S0 accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr1(&self) -> ATTR1_R {
        ATTR1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of I2S0 accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    pub fn attr2(&self) -> ATTR2_R {
        ATTR2_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EDMA_PMS_I2S0")
            .field("attr1", &format_args!("{}", self.attr1().bits()))
            .field("attr2", &format_args!("{}", self.attr2().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EDMA_PMS_I2S0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to configure the permission of I2S0 accessing address, which is larger than boundary 0 and less than boundary 1, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    #[must_use]
    pub fn attr1(&mut self) -> ATTR1_W<EDMA_PMS_I2S0_SPEC, 0> {
        ATTR1_W::new(self)
    }
    #[doc = "Bits 2:3 - This field is used to configure the permission of I2S0 accessing address, which is larger than boundary 1 and less than boundary 2, through EDMA. Bit 0: set this bit to enable read permission. Bit 1: set this bit to enable write permission."]
    #[inline(always)]
    #[must_use]
    pub fn attr2(&mut self) -> ATTR2_W<EDMA_PMS_I2S0_SPEC, 2> {
        ATTR2_W::new(self)
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
#[doc = "EDMA-I2S0 permission control register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`edma_pms_i2s0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`edma_pms_i2s0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EDMA_PMS_I2S0_SPEC;
impl crate::RegisterSpec for EDMA_PMS_I2S0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`edma_pms_i2s0::R`](R) reader structure"]
impl crate::Readable for EDMA_PMS_I2S0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`edma_pms_i2s0::W`](W) writer structure"]
impl crate::Writable for EDMA_PMS_I2S0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EDMA_PMS_I2S0 to value 0x0f"]
impl crate::Resettable for EDMA_PMS_I2S0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
