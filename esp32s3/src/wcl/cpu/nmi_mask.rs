#[doc = "Register `NMI_MASK` reader"]
pub type R = crate::R<NMI_MASK_SPEC>;
#[doc = "Register `NMI_MASK` writer"]
pub type W = crate::W<NMI_MASK_SPEC>;
#[doc = "Field `NMI_MASK` reader - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
pub type NMI_MASK_R = crate::BitReader;
#[doc = "Field `NMI_MASK` writer - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
pub type NMI_MASK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
    #[inline(always)]
    pub fn nmi_mask(&self) -> NMI_MASK_R {
        NMI_MASK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NMI_MASK")
            .field("nmi_mask", &self.nmi_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - this bit is used to mask NMI interrupt,it can directly mask NMI interrupt"]
    #[inline(always)]
    pub fn nmi_mask(&mut self) -> NMI_MASK_W<'_, NMI_MASK_SPEC> {
        NMI_MASK_W::new(self, 0)
    }
}
#[doc = "Core_0 NMI mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`nmi_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmi_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NMI_MASK_SPEC;
impl crate::RegisterSpec for NMI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmi_mask::R`](R) reader structure"]
impl crate::Readable for NMI_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`nmi_mask::W`](W) writer structure"]
impl crate::Writable for NMI_MASK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NMI_MASK to value 0"]
impl crate::Resettable for NMI_MASK_SPEC {}
