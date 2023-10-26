#[doc = "Register `CK8M_TICK_CONF` reader"]
pub type R = crate::R<CK8M_TICK_CONF_SPEC>;
#[doc = "Register `CK8M_TICK_CONF` writer"]
pub type W = crate::W<CK8M_TICK_CONF_SPEC>;
#[doc = "Field `CK8M_TICK_NUM` reader - "]
pub type CK8M_TICK_NUM_R = crate::FieldReader;
#[doc = "Field `CK8M_TICK_NUM` writer - "]
pub type CK8M_TICK_NUM_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn ck8m_tick_num(&self) -> CK8M_TICK_NUM_R {
        CK8M_TICK_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CK8M_TICK_CONF")
            .field(
                "ck8m_tick_num",
                &format_args!("{}", self.ck8m_tick_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CK8M_TICK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn ck8m_tick_num(&mut self) -> CK8M_TICK_NUM_W<CK8M_TICK_CONF_SPEC, 0> {
        CK8M_TICK_NUM_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ck8m_tick_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ck8m_tick_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CK8M_TICK_CONF_SPEC;
impl crate::RegisterSpec for CK8M_TICK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ck8m_tick_conf::R`](R) reader structure"]
impl crate::Readable for CK8M_TICK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ck8m_tick_conf::W`](W) writer structure"]
impl crate::Writable for CK8M_TICK_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CK8M_TICK_CONF to value 0x0b"]
impl crate::Resettable for CK8M_TICK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0b;
}
