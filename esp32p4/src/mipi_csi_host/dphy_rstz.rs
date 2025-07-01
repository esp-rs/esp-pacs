#[doc = "Register `DPHY_RSTZ` reader"]
pub type R = crate::R<DPHY_RSTZ_SPEC>;
#[doc = "Register `DPHY_RSTZ` writer"]
pub type W = crate::W<DPHY_RSTZ_SPEC>;
#[doc = "Field `DPHY_RSTZ` reader - NA"]
pub type DPHY_RSTZ_R = crate::BitReader;
#[doc = "Field `DPHY_RSTZ` writer - NA"]
pub type DPHY_RSTZ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dphy_rstz(&self) -> DPHY_RSTZ_R {
        DPHY_RSTZ_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPHY_RSTZ")
            .field("dphy_rstz", &self.dphy_rstz())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn dphy_rstz(&mut self) -> DPHY_RSTZ_W<DPHY_RSTZ_SPEC> {
        DPHY_RSTZ_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dphy_rstz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dphy_rstz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPHY_RSTZ_SPEC;
impl crate::RegisterSpec for DPHY_RSTZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dphy_rstz::R`](R) reader structure"]
impl crate::Readable for DPHY_RSTZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dphy_rstz::W`](W) writer structure"]
impl crate::Writable for DPHY_RSTZ_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DPHY_RSTZ to value 0"]
impl crate::Resettable for DPHY_RSTZ_SPEC {}
