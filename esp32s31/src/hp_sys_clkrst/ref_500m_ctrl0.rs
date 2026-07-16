#[doc = "Register `REF_500M_CTRL0` reader"]
pub type R = crate::R<REF_500M_CTRL0_SPEC>;
#[doc = "Register `REF_500M_CTRL0` writer"]
pub type W = crate::W<REF_500M_CTRL0_SPEC>;
#[doc = "Field `REF_500M_SEL` reader - need_des"]
pub type REF_500M_SEL_R = crate::BitReader;
#[doc = "Field `REF_500M_SEL` writer - need_des"]
pub type REF_500M_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ref_500m_sel(&self) -> REF_500M_SEL_R {
        REF_500M_SEL_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REF_500M_CTRL0")
            .field("ref_500m_sel", &self.ref_500m_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ref_500m_sel(&mut self) -> REF_500M_SEL_W<'_, REF_500M_CTRL0_SPEC> {
        REF_500M_SEL_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ref_500m_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ref_500m_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_500M_CTRL0_SPEC;
impl crate::RegisterSpec for REF_500M_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ref_500m_ctrl0::R`](R) reader structure"]
impl crate::Readable for REF_500M_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ref_500m_ctrl0::W`](W) writer structure"]
impl crate::Writable for REF_500M_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REF_500M_CTRL0 to value 0"]
impl crate::Resettable for REF_500M_CTRL0_SPEC {}
