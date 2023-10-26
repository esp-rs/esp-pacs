#[doc = "Register `LOG_DATA_MASK` reader"]
pub type R = crate::R<LOG_DATA_MASK_SPEC>;
#[doc = "Register `LOG_DATA_MASK` writer"]
pub type W = crate::W<LOG_DATA_MASK_SPEC>;
#[doc = "Field `LOG_DATA_MASK` reader - byte mask enable, BIT0 mask the first byte of MEM_MONITOR_LOG_CHECK_DATA, and BIT1 mask second byte, and so on."]
pub type LOG_DATA_MASK_R = crate::FieldReader;
#[doc = "Field `LOG_DATA_MASK` writer - byte mask enable, BIT0 mask the first byte of MEM_MONITOR_LOG_CHECK_DATA, and BIT1 mask second byte, and so on."]
pub type LOG_DATA_MASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:3 - byte mask enable, BIT0 mask the first byte of MEM_MONITOR_LOG_CHECK_DATA, and BIT1 mask second byte, and so on."]
    #[inline(always)]
    pub fn log_data_mask(&self) -> LOG_DATA_MASK_R {
        LOG_DATA_MASK_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_DATA_MASK")
            .field(
                "log_data_mask",
                &format_args!("{}", self.log_data_mask().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOG_DATA_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - byte mask enable, BIT0 mask the first byte of MEM_MONITOR_LOG_CHECK_DATA, and BIT1 mask second byte, and so on."]
    #[inline(always)]
    #[must_use]
    pub fn log_data_mask(&mut self) -> LOG_DATA_MASK_W<LOG_DATA_MASK_SPEC, 0> {
        LOG_DATA_MASK_W::new(self)
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
#[doc = "check data mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`log_data_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_data_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_DATA_MASK_SPEC;
impl crate::RegisterSpec for LOG_DATA_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log_data_mask::R`](R) reader structure"]
impl crate::Readable for LOG_DATA_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`log_data_mask::W`](W) writer structure"]
impl crate::Writable for LOG_DATA_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOG_DATA_MASK to value 0"]
impl crate::Resettable for LOG_DATA_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
