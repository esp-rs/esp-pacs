#[doc = "Register `DCACHE_OCCUPY_CTRL` reader"]
pub type R = crate::R<DCACHE_OCCUPY_CTRL_SPEC>;
#[doc = "Register `DCACHE_OCCUPY_CTRL` writer"]
pub type W = crate::W<DCACHE_OCCUPY_CTRL_SPEC>;
#[doc = "Field `DCACHE_OCCUPY_ENA` reader - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
pub type DCACHE_OCCUPY_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_OCCUPY_ENA` writer - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
pub type DCACHE_OCCUPY_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `DCACHE_OCCUPY_DONE` reader - The bit is used to indicate occupy operation is finished."]
pub type DCACHE_OCCUPY_DONE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
    #[inline(always)]
    pub fn dcache_occupy_ena(&self) -> DCACHE_OCCUPY_ENA_R {
        DCACHE_OCCUPY_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate occupy operation is finished."]
    #[inline(always)]
    pub fn dcache_occupy_done(&self) -> DCACHE_OCCUPY_DONE_R {
        DCACHE_OCCUPY_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_OCCUPY_CTRL")
            .field(
                "dcache_occupy_ena",
                &format_args!("{}", self.dcache_occupy_ena().bit()),
            )
            .field(
                "dcache_occupy_done",
                &format_args!("{}", self.dcache_occupy_done().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_OCCUPY_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable occupy operation. It will be cleared by hardware after issuing Auot-Invalidate Operation."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_occupy_ena(&mut self) -> DCACHE_OCCUPY_ENA_W<DCACHE_OCCUPY_CTRL_SPEC, 0> {
        DCACHE_OCCUPY_ENA_W::new(self)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_occupy_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_occupy_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_OCCUPY_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_OCCUPY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_occupy_ctrl::R`](R) reader structure"]
impl crate::Readable for DCACHE_OCCUPY_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_occupy_ctrl::W`](W) writer structure"]
impl crate::Writable for DCACHE_OCCUPY_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_OCCUPY_CTRL to value 0x02"]
impl crate::Resettable for DCACHE_OCCUPY_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
