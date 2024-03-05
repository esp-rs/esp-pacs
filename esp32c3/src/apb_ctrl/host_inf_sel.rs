#[doc = "Register `HOST_INF_SEL` reader"]
pub type R = crate::R<HOST_INF_SEL_SPEC>;
#[doc = "Register `HOST_INF_SEL` writer"]
pub type W = crate::W<HOST_INF_SEL_SPEC>;
#[doc = "Field `PERI_IO_SWAP` reader - reg_peri_io_swap"]
pub type PERI_IO_SWAP_R = crate::FieldReader;
#[doc = "Field `PERI_IO_SWAP` writer - reg_peri_io_swap"]
pub type PERI_IO_SWAP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - reg_peri_io_swap"]
    #[inline(always)]
    pub fn peri_io_swap(&self) -> PERI_IO_SWAP_R {
        PERI_IO_SWAP_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_INF_SEL")
            .field(
                "peri_io_swap",
                &format_args!("{}", self.peri_io_swap().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_INF_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - reg_peri_io_swap"]
    #[inline(always)]
    #[must_use]
    pub fn peri_io_swap(&mut self) -> PERI_IO_SWAP_W<HOST_INF_SEL_SPEC> {
        PERI_IO_SWAP_W::new(self, 0)
    }
}
#[doc = "APB_CTRL_HOST_INF_SEL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_inf_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_inf_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_INF_SEL_SPEC;
impl crate::RegisterSpec for HOST_INF_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_inf_sel::R`](R) reader structure"]
impl crate::Readable for HOST_INF_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_inf_sel::W`](W) writer structure"]
impl crate::Writable for HOST_INF_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_INF_SEL to value 0"]
impl crate::Resettable for HOST_INF_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
