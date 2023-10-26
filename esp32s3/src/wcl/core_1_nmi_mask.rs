#[doc = "Register `Core_1_NMI_MASK` reader"]
pub type R = crate::R<CORE_1_NMI_MASK_SPEC>;
#[doc = "Register `Core_1_NMI_MASK` writer"]
pub type W = crate::W<CORE_1_NMI_MASK_SPEC>;
#[doc = "Field `CORE_1_NMI_MASK` reader - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
pub type CORE_1_NMI_MASK_R = crate::BitReader;
#[doc = "Field `CORE_1_NMI_MASK` writer - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
pub type CORE_1_NMI_MASK_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
    #[inline(always)]
    pub fn core_1_nmi_mask(&self) -> CORE_1_NMI_MASK_R {
        CORE_1_NMI_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_NMI_MASK")
            .field(
                "core_1_nmi_mask",
                &format_args!("{}", self.core_1_nmi_mask().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_NMI_MASK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_nmi_mask(&mut self) -> CORE_1_NMI_MASK_W<CORE_1_NMI_MASK_SPEC, 0> {
        CORE_1_NMI_MASK_W::new(self)
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
#[doc = "Core_1 NMI mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_nmi_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_nmi_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_NMI_MASK_SPEC;
impl crate::RegisterSpec for CORE_1_NMI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_nmi_mask::R`](R) reader structure"]
impl crate::Readable for CORE_1_NMI_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_nmi_mask::W`](W) writer structure"]
impl crate::Writable for CORE_1_NMI_MASK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_NMI_MASK to value 0"]
impl crate::Resettable for CORE_1_NMI_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
