#[doc = "Register `HOST_SLC1HOST_RX_INFOR` reader"]
pub type R = crate::R<HOST_SLC1HOST_RX_INFOR_SPEC>;
#[doc = "Register `HOST_SLC1HOST_RX_INFOR` writer"]
pub type W = crate::W<HOST_SLC1HOST_RX_INFOR_SPEC>;
#[doc = "Field `HOST_SLC1HOST_RX_INFOR` reader - "]
pub type HOST_SLC1HOST_RX_INFOR_R = crate::FieldReader<u32>;
#[doc = "Field `HOST_SLC1HOST_RX_INFOR` writer - "]
pub type HOST_SLC1HOST_RX_INFOR_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 20, O, u32>;
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
    pub fn host_slc1host_rx_infor(
        &mut self,
    ) -> HOST_SLC1HOST_RX_INFOR_W<HOST_SLC1HOST_RX_INFOR_SPEC, 0> {
        HOST_SLC1HOST_RX_INFOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slc1host_rx_infor::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slc1host_rx_infor::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLC1HOST_RX_INFOR_SPEC;
impl crate::RegisterSpec for HOST_SLC1HOST_RX_INFOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slc1host_rx_infor::R`](R) reader structure"]
impl crate::Readable for HOST_SLC1HOST_RX_INFOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slc1host_rx_infor::W`](W) writer structure"]
impl crate::Writable for HOST_SLC1HOST_RX_INFOR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLC1HOST_RX_INFOR to value 0"]
impl crate::Resettable for HOST_SLC1HOST_RX_INFOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
