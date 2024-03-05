#[doc = "Register `ANA_CONF0` reader"]
pub type R = crate::R<ANA_CONF0_SPEC>;
#[doc = "Register `ANA_CONF0` writer"]
pub type W = crate::W<ANA_CONF0_SPEC>;
#[doc = "Field `ANA_CONF0` reader - need des"]
pub type ANA_CONF0_R = crate::FieldReader<u32>;
#[doc = "Field `ANA_CONF0` writer - need des"]
pub type ANA_CONF0_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `ANA_STATUS0` reader - need des"]
pub type ANA_STATUS0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    pub fn ana_conf0(&self) -> ANA_CONF0_R {
        ANA_CONF0_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - need des"]
    #[inline(always)]
    pub fn ana_status0(&self) -> ANA_STATUS0_R {
        ANA_STATUS0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ANA_CONF0")
            .field("ana_conf0", &format_args!("{}", self.ana_conf0().bits()))
            .field(
                "ana_status0",
                &format_args!("{}", self.ana_status0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ANA_CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:23 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn ana_conf0(&mut self) -> ANA_CONF0_W<ANA_CONF0_SPEC> {
        ANA_CONF0_W::new(self, 0)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ana_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ana_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ANA_CONF0_SPEC;
impl crate::RegisterSpec for ANA_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ana_conf0::R`](R) reader structure"]
impl crate::Readable for ANA_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ana_conf0::W`](W) writer structure"]
impl crate::Writable for ANA_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ANA_CONF0 to value 0"]
impl crate::Resettable for ANA_CONF0_SPEC {
    const RESET_VALUE: u32 = 0;
}
