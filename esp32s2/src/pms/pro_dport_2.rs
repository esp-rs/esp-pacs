#[doc = "Register `PRO_DPORT_2` reader"]
pub struct R(crate::R<PRO_DPORT_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DPORT_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DPORT_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DPORT_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DPORT_2` writer"]
pub struct W(crate::W<PRO_DPORT_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DPORT_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PRO_DPORT_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DPORT_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_0` reader - Configure read-protection address 0."]
pub type PRO_DPORT_RESERVE_FIFO_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_0` writer - Configure read-protection address 0."]
pub type PRO_DPORT_RESERVE_FIFO_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_DPORT_2_SPEC, 18, O, u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Configure read-protection address 0."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_0(&self) -> PRO_DPORT_RESERVE_FIFO_0_R {
        PRO_DPORT_RESERVE_FIFO_0_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_2")
            .field(
                "pro_dport_reserve_fifo_0",
                &format_args!("{}", self.pro_dport_reserve_fifo_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure read-protection address 0."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_reserve_fifo_0(&mut self) -> PRO_DPORT_RESERVE_FIFO_0_W<0> {
        PRO_DPORT_RESERVE_FIFO_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeriBus1 permission control register 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dport_2](index.html) module"]
pub struct PRO_DPORT_2_SPEC;
impl crate::RegisterSpec for PRO_DPORT_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dport_2::R](R) reader structure"]
impl crate::Readable for PRO_DPORT_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dport_2::W](W) writer structure"]
impl crate::Writable for PRO_DPORT_2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_2 to value 0"]
impl crate::Resettable for PRO_DPORT_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
