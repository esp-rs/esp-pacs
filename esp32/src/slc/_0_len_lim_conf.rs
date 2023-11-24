#[doc = "Register `_0_LEN_LIM_CONF` reader"]
pub type R = crate::R<_0_LEN_LIM_CONF_SPEC>;
#[doc = "Register `_0_LEN_LIM_CONF` writer"]
pub type W = crate::W<_0_LEN_LIM_CONF_SPEC>;
#[doc = "Field `SLC0_LEN_LIM` reader - "]
pub type SLC0_LEN_LIM_R = crate::FieldReader<u32>;
#[doc = "Field `SLC0_LEN_LIM` writer - "]
pub type SLC0_LEN_LIM_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn slc0_len_lim(&self) -> SLC0_LEN_LIM_R {
        SLC0_LEN_LIM_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_LEN_LIM_CONF")
            .field(
                "slc0_len_lim",
                &format_args!("{}", self.slc0_len_lim().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<_0_LEN_LIM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_len_lim(&mut self) -> SLC0_LEN_LIM_W<_0_LEN_LIM_CONF_SPEC> {
        SLC0_LEN_LIM_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_0_len_lim_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_0_len_lim_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_LEN_LIM_CONF_SPEC;
impl crate::RegisterSpec for _0_LEN_LIM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_len_lim_conf::R`](R) reader structure"]
impl crate::Readable for _0_LEN_LIM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0_len_lim_conf::W`](W) writer structure"]
impl crate::Writable for _0_LEN_LIM_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets _0_LEN_LIM_CONF to value 0x5400"]
impl crate::Resettable for _0_LEN_LIM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x5400;
}
