#[doc = "Register `Core_0_NMI_MASK` reader"]
pub type R = crate::R<CORE_0_NMI_MASK_SPEC>;
#[doc = "Register `Core_0_NMI_MASK` writer"]
pub type W = crate::W<CORE_0_NMI_MASK_SPEC>;
#[doc = "Field `CORE_0_NMI_MASK` reader - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
pub type CORE_0_NMI_MASK_R = crate::BitReader;
#[doc = "Field `CORE_0_NMI_MASK` writer - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
pub type CORE_0_NMI_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
    #[inline(always)]
    pub fn core_0_nmi_mask(&self) -> CORE_0_NMI_MASK_R {
        CORE_0_NMI_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_0_NMI_MASK")
            .field("core_0_nmi_mask", &self.core_0_nmi_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
    #[inline(always)]
    pub fn core_0_nmi_mask(&mut self) -> CORE_0_NMI_MASK_W<CORE_0_NMI_MASK_SPEC> {
        CORE_0_NMI_MASK_W::new(self, 0)
    }
}
#[doc = "Core_0 NMI mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_nmi_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`core_0_nmi_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_NMI_MASK_SPEC;
impl crate::RegisterSpec for CORE_0_NMI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_nmi_mask::R`](R) reader structure"]
impl crate::Readable for CORE_0_NMI_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_nmi_mask::W`](W) writer structure"]
impl crate::Writable for CORE_0_NMI_MASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets Core_0_NMI_MASK to value 0"]
impl crate::Resettable for CORE_0_NMI_MASK_SPEC {}
