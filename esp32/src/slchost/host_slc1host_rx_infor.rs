#[doc = "Register `HOST_SLC1HOST_RX_INFOR` reader"]
pub struct R(crate::R<HOST_SLC1HOST_RX_INFOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOST_SLC1HOST_RX_INFOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOST_SLC1HOST_RX_INFOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOST_SLC1HOST_RX_INFOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HOST_SLC1HOST_RX_INFOR` writer"]
pub struct W(crate::W<HOST_SLC1HOST_RX_INFOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HOST_SLC1HOST_RX_INFOR_SPEC>;
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
impl From<crate::W<HOST_SLC1HOST_RX_INFOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HOST_SLC1HOST_RX_INFOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HOST_SLC1HOST_RX_INFOR` reader - "]
pub type HOST_SLC1HOST_RX_INFOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HOST_SLC1HOST_RX_INFOR` writer - "]
pub type HOST_SLC1HOST_RX_INFOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, HOST_SLC1HOST_RX_INFOR_SPEC, 20, O, u32, u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn host_slc1host_rx_infor(&self) -> HOST_SLC1HOST_RX_INFOR_R {
        HOST_SLC1HOST_RX_INFOR_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLC1HOST_RX_INFOR")
            .field(
                "host_slc1host_rx_infor",
                &format_args!("{}", self.host_slc1host_rx_infor().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLC1HOST_RX_INFOR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn host_slc1host_rx_infor(&mut self) -> HOST_SLC1HOST_RX_INFOR_W<0> {
        HOST_SLC1HOST_RX_INFOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [host_slc1host_rx_infor](index.html) module"]
pub struct HOST_SLC1HOST_RX_INFOR_SPEC;
impl crate::RegisterSpec for HOST_SLC1HOST_RX_INFOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [host_slc1host_rx_infor::R](R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_RX_INFOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [host_slc1host_rx_infor::W](W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_RX_INFOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLC1HOST_RX_INFOR to value 0"]
impl crate::Resettable for HOST_SLC1HOST_RX_INFOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
