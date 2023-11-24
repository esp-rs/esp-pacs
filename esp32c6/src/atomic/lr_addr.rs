#[doc = "Register `LR_ADDR` reader"]
pub type R = crate::R<LR_ADDR_SPEC>;
#[doc = "Register `LR_ADDR` writer"]
pub type W = crate::W<LR_ADDR_SPEC>;
#[doc = "Field `GLOABLE_LR_ADDR` reader - backup gloable address"]
pub type GLOABLE_LR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `GLOABLE_LR_ADDR` writer - backup gloable address"]
pub type GLOABLE_LR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - backup gloable address"]
    #[inline(always)]
    pub fn gloable_lr_addr(&self) -> GLOABLE_LR_ADDR_R {
        GLOABLE_LR_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LR_ADDR")
            .field(
                "gloable_lr_addr",
                &format_args!("{}", self.gloable_lr_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LR_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - backup gloable address"]
    #[inline(always)]
    #[must_use]
    pub fn gloable_lr_addr(&mut self) -> GLOABLE_LR_ADDR_W<LR_ADDR_SPEC> {
        GLOABLE_LR_ADDR_W::new(self, 0)
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
#[doc = "gloable lr address regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lr_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lr_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LR_ADDR_SPEC;
impl crate::RegisterSpec for LR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lr_addr::R`](R) reader structure"]
impl crate::Readable for LR_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lr_addr::W`](W) writer structure"]
impl crate::Writable for LR_ADDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LR_ADDR to value 0"]
impl crate::Resettable for LR_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
