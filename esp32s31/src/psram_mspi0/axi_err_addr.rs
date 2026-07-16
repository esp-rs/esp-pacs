#[doc = "Register `AXI_ERR_ADDR` reader"]
pub type R = crate::R<AXI_ERR_ADDR_SPEC>;
#[doc = "Register `AXI_ERR_ADDR` writer"]
pub type W = crate::W<AXI_ERR_ADDR_SPEC>;
#[doc = "Field `AXI_ERR_ADDR` reader - "]
pub type AXI_ERR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `AXI_ERR_ADDR` writer - "]
pub type AXI_ERR_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn axi_err_addr(&self) -> AXI_ERR_ADDR_R {
        AXI_ERR_ADDR_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_ERR_ADDR")
            .field("axi_err_addr", &self.axi_err_addr())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:28"]
    #[inline(always)]
    pub fn axi_err_addr(&mut self) -> AXI_ERR_ADDR_W<'_, AXI_ERR_ADDR_SPEC> {
        AXI_ERR_ADDR_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_err_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_err_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_ERR_ADDR_SPEC;
impl crate::RegisterSpec for AXI_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_err_addr::R`](R) reader structure"]
impl crate::Readable for AXI_ERR_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_err_addr::W`](W) writer structure"]
impl crate::Writable for AXI_ERR_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_ERR_ADDR to value 0"]
impl crate::Resettable for AXI_ERR_ADDR_SPEC {}
