#[doc = "Register `CH2_AXI_QOS0` reader"]
pub type R = crate::R<CH2_AXI_QOS0_SPEC>;
#[doc = "Register `CH2_AXI_QOS0` writer"]
pub type W = crate::W<CH2_AXI_QOS0_SPEC>;
#[doc = "Field `CH2_AXI_AWQOS` reader - NA"]
pub type CH2_AXI_AWQOS_R = crate::FieldReader;
#[doc = "Field `CH2_AXI_AWQOS` writer - NA"]
pub type CH2_AXI_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH2_AXI_ARQOS` reader - NA"]
pub type CH2_AXI_ARQOS_R = crate::FieldReader;
#[doc = "Field `CH2_AXI_ARQOS` writer - NA"]
pub type CH2_AXI_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn ch2_axi_awqos(&self) -> CH2_AXI_AWQOS_R {
        CH2_AXI_AWQOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - NA"]
    #[inline(always)]
    pub fn ch2_axi_arqos(&self) -> CH2_AXI_ARQOS_R {
        CH2_AXI_ARQOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_AXI_QOS0")
            .field(
                "ch2_axi_awqos",
                &format_args!("{}", self.ch2_axi_awqos().bits()),
            )
            .field(
                "ch2_axi_arqos",
                &format_args!("{}", self.ch2_axi_arqos().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_AXI_QOS0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_axi_awqos(&mut self) -> CH2_AXI_AWQOS_W<CH2_AXI_QOS0_SPEC> {
        CH2_AXI_AWQOS_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_axi_arqos(&mut self) -> CH2_AXI_ARQOS_W<CH2_AXI_QOS0_SPEC> {
        CH2_AXI_ARQOS_W::new(self, 4)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_axi_qos0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_axi_qos0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_AXI_QOS0_SPEC;
impl crate::RegisterSpec for CH2_AXI_QOS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_axi_qos0::R`](R) reader structure"]
impl crate::Readable for CH2_AXI_QOS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_axi_qos0::W`](W) writer structure"]
impl crate::Writable for CH2_AXI_QOS0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_AXI_QOS0 to value 0"]
impl crate::Resettable for CH2_AXI_QOS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
