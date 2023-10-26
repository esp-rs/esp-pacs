#[doc = "Register `IBUS_PMS_TBL_LOCK` reader"]
pub type R = crate::R<IBUS_PMS_TBL_LOCK_SPEC>;
#[doc = "Register `IBUS_PMS_TBL_LOCK` writer"]
pub type W = crate::W<IBUS_PMS_TBL_LOCK_SPEC>;
#[doc = "Field `IBUS_PMS_LOCK` reader - The bit is used to configure the ibus permission control section boundary0"]
pub type IBUS_PMS_LOCK_R = crate::BitReader;
#[doc = "Field `IBUS_PMS_LOCK` writer - The bit is used to configure the ibus permission control section boundary0"]
pub type IBUS_PMS_LOCK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to configure the ibus permission control section boundary0"]
    #[inline(always)]
    pub fn ibus_pms_lock(&self) -> IBUS_PMS_LOCK_R {
        IBUS_PMS_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBUS_PMS_TBL_LOCK")
            .field(
                "ibus_pms_lock",
                &format_args!("{}", self.ibus_pms_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBUS_PMS_TBL_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to configure the ibus permission control section boundary0"]
    #[inline(always)]
    #[must_use]
    pub fn ibus_pms_lock(&mut self) -> IBUS_PMS_LOCK_W<IBUS_PMS_TBL_LOCK_SPEC, 0> {
        IBUS_PMS_LOCK_W::new(self)
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
#[doc = "This description will be updated in the near future.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibus_pms_tbl_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibus_pms_tbl_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBUS_PMS_TBL_LOCK_SPEC;
impl crate::RegisterSpec for IBUS_PMS_TBL_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibus_pms_tbl_lock::R`](R) reader structure"]
impl crate::Readable for IBUS_PMS_TBL_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibus_pms_tbl_lock::W`](W) writer structure"]
impl crate::Writable for IBUS_PMS_TBL_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBUS_PMS_TBL_LOCK to value 0"]
impl crate::Resettable for IBUS_PMS_TBL_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
