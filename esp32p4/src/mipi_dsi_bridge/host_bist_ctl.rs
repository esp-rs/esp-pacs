#[doc = "Register `HOST_BIST_CTL` reader"]
pub type R = crate::R<HOST_BIST_CTL_SPEC>;
#[doc = "Register `HOST_BIST_CTL` writer"]
pub type W = crate::W<HOST_BIST_CTL_SPEC>;
#[doc = "Field `BISTOK` reader - bistok"]
pub type BISTOK_R = crate::BitReader;
#[doc = "Field `BISTON` reader - biston"]
pub type BISTON_R = crate::BitReader;
#[doc = "Field `BISTON` writer - biston"]
pub type BISTON_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - bistok"]
    #[inline(always)]
    pub fn bistok(&self) -> BISTOK_R {
        BISTOK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - biston"]
    #[inline(always)]
    pub fn biston(&self) -> BISTON_R {
        BISTON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_BIST_CTL")
            .field("bistok", &self.bistok().bit())
            .field("biston", &self.biston().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_BIST_CTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - biston"]
    #[inline(always)]
    #[must_use]
    pub fn biston(&mut self) -> BISTON_W<HOST_BIST_CTL_SPEC> {
        BISTON_W::new(self, 1)
    }
}
#[doc = "dsi_bridge host bist control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_bist_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_bist_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_BIST_CTL_SPEC;
impl crate::RegisterSpec for HOST_BIST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_bist_ctl::R`](R) reader structure"]
impl crate::Readable for HOST_BIST_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_bist_ctl::W`](W) writer structure"]
impl crate::Writable for HOST_BIST_CTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_BIST_CTL to value 0"]
impl crate::Resettable for HOST_BIST_CTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
