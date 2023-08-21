#[doc = "Register `PRO_DPORT_APB_MASK0` reader"]
pub type R = crate::R<PRO_DPORT_APB_MASK0_SPEC>;
#[doc = "Register `PRO_DPORT_APB_MASK0` writer"]
pub type W = crate::W<PRO_DPORT_APB_MASK0_SPEC>;
#[doc = "Field `PRODPORT_APB_MASK0` reader - "]
pub type PRODPORT_APB_MASK0_R = crate::FieldReader<u32>;
#[doc = "Field `PRODPORT_APB_MASK0` writer - "]
pub type PRODPORT_APB_MASK0_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn prodport_apb_mask0(&self) -> PRODPORT_APB_MASK0_R {
        PRODPORT_APB_MASK0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_APB_MASK0")
            .field(
                "prodport_apb_mask0",
                &format_args!("{}", self.prodport_apb_mask0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_APB_MASK0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn prodport_apb_mask0(&mut self) -> PRODPORT_APB_MASK0_W<PRO_DPORT_APB_MASK0_SPEC, 0> {
        PRODPORT_APB_MASK0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dport_apb_mask0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_dport_apb_mask0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DPORT_APB_MASK0_SPEC;
impl crate::RegisterSpec for PRO_DPORT_APB_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dport_apb_mask0::R`](R) reader structure"]
impl crate::Readable for PRO_DPORT_APB_MASK0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_dport_apb_mask0::W`](W) writer structure"]
impl crate::Writable for PRO_DPORT_APB_MASK0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_APB_MASK0 to value 0"]
impl crate::Resettable for PRO_DPORT_APB_MASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
