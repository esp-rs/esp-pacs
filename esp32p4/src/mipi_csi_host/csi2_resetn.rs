#[doc = "Register `CSI2_RESETN` reader"]
pub type R = crate::R<CSI2_RESETN_SPEC>;
#[doc = "Register `CSI2_RESETN` writer"]
pub type W = crate::W<CSI2_RESETN_SPEC>;
#[doc = "Field `CSI2_RESETN` reader - NA"]
pub type CSI2_RESETN_R = crate::BitReader;
#[doc = "Field `CSI2_RESETN` writer - NA"]
pub type CSI2_RESETN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn csi2_resetn(&self) -> CSI2_RESETN_R {
        CSI2_RESETN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSI2_RESETN")
            .field("csi2_resetn", &format_args!("{}", self.csi2_resetn().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CSI2_RESETN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn csi2_resetn(&mut self) -> CSI2_RESETN_W<CSI2_RESETN_SPEC> {
        CSI2_RESETN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csi2_resetn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csi2_resetn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSI2_RESETN_SPEC;
impl crate::RegisterSpec for CSI2_RESETN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csi2_resetn::R`](R) reader structure"]
impl crate::Readable for CSI2_RESETN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`csi2_resetn::W`](W) writer structure"]
impl crate::Writable for CSI2_RESETN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSI2_RESETN to value 0"]
impl crate::Resettable for CSI2_RESETN_SPEC {
    const RESET_VALUE: u32 = 0;
}
