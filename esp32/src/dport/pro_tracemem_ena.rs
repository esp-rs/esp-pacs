#[doc = "Register `PRO_TRACEMEM_ENA` reader"]
pub type R = crate::R<PRO_TRACEMEM_ENA_SPEC>;
#[doc = "Register `PRO_TRACEMEM_ENA` writer"]
pub type W = crate::W<PRO_TRACEMEM_ENA_SPEC>;
#[doc = "Field `PRO_TRACEMEM_ENA` reader - "]
pub type PRO_TRACEMEM_ENA_R = crate::BitReader;
#[doc = "Field `PRO_TRACEMEM_ENA` writer - "]
pub type PRO_TRACEMEM_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_tracemem_ena(&self) -> PRO_TRACEMEM_ENA_R {
        PRO_TRACEMEM_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_TRACEMEM_ENA")
            .field(
                "pro_tracemem_ena",
                &format_args!("{}", self.pro_tracemem_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_TRACEMEM_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn pro_tracemem_ena(&mut self) -> PRO_TRACEMEM_ENA_W<PRO_TRACEMEM_ENA_SPEC> {
        PRO_TRACEMEM_ENA_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_tracemem_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_tracemem_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_TRACEMEM_ENA_SPEC;
impl crate::RegisterSpec for PRO_TRACEMEM_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_tracemem_ena::R`](R) reader structure"]
impl crate::Readable for PRO_TRACEMEM_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_tracemem_ena::W`](W) writer structure"]
impl crate::Writable for PRO_TRACEMEM_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_TRACEMEM_ENA to value 0"]
impl crate::Resettable for PRO_TRACEMEM_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
