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
            .field("bistok", &self.bistok())
            .field("biston", &self.biston())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - biston"]
    #[inline(always)]
    pub fn biston(&mut self) -> BISTON_W<HOST_BIST_CTL_SPEC> {
        BISTON_W::new(self, 1)
    }
}
#[doc = "dsi_bridge host bist control register\n\nYou can [`read`](crate::Reg::read) this register and get [`host_bist_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`host_bist_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_BIST_CTL_SPEC;
impl crate::RegisterSpec for HOST_BIST_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_bist_ctl::R`](R) reader structure"]
impl crate::Readable for HOST_BIST_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_bist_ctl::W`](W) writer structure"]
impl crate::Writable for HOST_BIST_CTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HOST_BIST_CTL to value 0"]
impl crate::Resettable for HOST_BIST_CTL_SPEC {}
