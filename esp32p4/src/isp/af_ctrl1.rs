#[doc = "Register `AF_CTRL1` reader"]
pub type R = crate::R<AF_CTRL1_SPEC>;
#[doc = "Register `AF_CTRL1` writer"]
pub type W = crate::W<AF_CTRL1_SPEC>;
#[doc = "Field `AF_THPIXNUM` reader - this field configures pixnum used when calculating the autofocus threshold. Set to 0 to disable threshold calculation"]
pub type AF_THPIXNUM_R = crate::FieldReader<u32>;
#[doc = "Field `AF_THPIXNUM` writer - this field configures pixnum used when calculating the autofocus threshold. Set to 0 to disable threshold calculation"]
pub type AF_THPIXNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
impl R {
    #[doc = "Bits 0:21 - this field configures pixnum used when calculating the autofocus threshold. Set to 0 to disable threshold calculation"]
    #[inline(always)]
    pub fn af_thpixnum(&self) -> AF_THPIXNUM_R {
        AF_THPIXNUM_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_CTRL1")
            .field("af_thpixnum", &self.af_thpixnum().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AF_CTRL1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - this field configures pixnum used when calculating the autofocus threshold. Set to 0 to disable threshold calculation"]
    #[inline(always)]
    #[must_use]
    pub fn af_thpixnum(&mut self) -> AF_THPIXNUM_W<AF_CTRL1_SPEC> {
        AF_THPIXNUM_W::new(self, 0)
    }
}
#[doc = "af control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_CTRL1_SPEC;
impl crate::RegisterSpec for AF_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_ctrl1::R`](R) reader structure"]
impl crate::Readable for AF_CTRL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_ctrl1::W`](W) writer structure"]
impl crate::Writable for AF_CTRL1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF_CTRL1 to value 0"]
impl crate::Resettable for AF_CTRL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
