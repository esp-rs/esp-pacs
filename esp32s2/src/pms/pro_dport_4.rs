#[doc = "Register `PRO_DPORT_4` reader"]
pub struct R(crate::R<PRO_DPORT_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_DPORT_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_DPORT_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_DPORT_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_DPORT_4` writer"]
pub struct W(crate::W<PRO_DPORT_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_DPORT_4_SPEC>;
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
impl From<crate::W<PRO_DPORT_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_DPORT_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_2` reader - Configure read-protection address 2."]
pub type PRO_DPORT_RESERVE_FIFO_2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PRO_DPORT_RESERVE_FIFO_2` writer - Configure read-protection address 2."]
pub type PRO_DPORT_RESERVE_FIFO_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_DPORT_4_SPEC, 18, O, u32, u32>;
impl R {
    #[doc = "Bits 0:17 - Configure read-protection address 2."]
    #[inline(always)]
    pub fn pro_dport_reserve_fifo_2(&self) -> PRO_DPORT_RESERVE_FIFO_2_R {
        PRO_DPORT_RESERVE_FIFO_2_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DPORT_4")
            .field(
                "pro_dport_reserve_fifo_2",
                &format_args!("{}", self.pro_dport_reserve_fifo_2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DPORT_4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:17 - Configure read-protection address 2."]
    #[inline(always)]
    #[must_use]
    pub fn pro_dport_reserve_fifo_2(&mut self) -> PRO_DPORT_RESERVE_FIFO_2_W<0> {
        PRO_DPORT_RESERVE_FIFO_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PeriBus1 permission control register 4.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_dport_4](index.html) module"]
pub struct PRO_DPORT_4_SPEC;
impl crate::RegisterSpec for PRO_DPORT_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_dport_4::R](R) reader structure"]
impl crate::Readable for PRO_DPORT_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_dport_4::W](W) writer structure"]
impl crate::Writable for PRO_DPORT_4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_DPORT_4 to value 0"]
impl crate::Resettable for PRO_DPORT_4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
