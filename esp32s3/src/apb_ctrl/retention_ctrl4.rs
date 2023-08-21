#[doc = "Register `RETENTION_CTRL4` reader"]
pub type R = crate::R<RETENTION_CTRL4_SPEC>;
#[doc = "Register `RETENTION_CTRL4` writer"]
pub type W = crate::W<RETENTION_CTRL4_SPEC>;
#[doc = "Field `RETENTION_INV_CFG` reader - ******* Description ***********"]
pub type RETENTION_INV_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `RETENTION_INV_CFG` writer - ******* Description ***********"]
pub type RETENTION_INV_CFG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn retention_inv_cfg(&self) -> RETENTION_INV_CFG_R {
        RETENTION_INV_CFG_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RETENTION_CTRL4")
            .field(
                "retention_inv_cfg",
                &format_args!("{}", self.retention_inv_cfg().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RETENTION_CTRL4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    #[must_use]
    pub fn retention_inv_cfg(&mut self) -> RETENTION_INV_CFG_W<RETENTION_CTRL4_SPEC, 0> {
        RETENTION_INV_CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`retention_ctrl4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`retention_ctrl4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RETENTION_CTRL4_SPEC;
impl crate::RegisterSpec for RETENTION_CTRL4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`retention_ctrl4::R`](R) reader structure"]
impl crate::Readable for RETENTION_CTRL4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`retention_ctrl4::W`](W) writer structure"]
impl crate::Writable for RETENTION_CTRL4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RETENTION_CTRL4 to value 0xffff_ffff"]
impl crate::Resettable for RETENTION_CTRL4_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
