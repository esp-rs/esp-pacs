#[doc = "Register `SLC0HOST_LEN_WD` reader"]
pub type R = crate::R<SLC0HOST_LEN_WD_SPEC>;
#[doc = "Register `SLC0HOST_LEN_WD` writer"]
pub type W = crate::W<SLC0HOST_LEN_WD_SPEC>;
#[doc = "Field `SLC0HOST_LEN_WD` reader - *******Description***********"]
pub type SLC0HOST_LEN_WD_R = crate::FieldReader<u32>;
#[doc = "Field `SLC0HOST_LEN_WD` writer - *******Description***********"]
pub type SLC0HOST_LEN_WD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slc0host_len_wd(&self) -> SLC0HOST_LEN_WD_R {
        SLC0HOST_LEN_WD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC0HOST_LEN_WD")
            .field(
                "slc0host_len_wd",
                &format_args!("{}", self.slc0host_len_wd().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC0HOST_LEN_WD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slc0host_len_wd(&mut self) -> SLC0HOST_LEN_WD_W<SLC0HOST_LEN_WD_SPEC> {
        SLC0HOST_LEN_WD_W::new(self, 0)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc0host_len_wd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slc0host_len_wd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC0HOST_LEN_WD_SPEC;
impl crate::RegisterSpec for SLC0HOST_LEN_WD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc0host_len_wd::R`](R) reader structure"]
impl crate::Readable for SLC0HOST_LEN_WD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slc0host_len_wd::W`](W) writer structure"]
impl crate::Writable for SLC0HOST_LEN_WD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLC0HOST_LEN_WD to value 0"]
impl crate::Resettable for SLC0HOST_LEN_WD_SPEC {
    const RESET_VALUE: u32 = 0;
}
