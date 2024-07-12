#[doc = "Register `IBI_SIR_REQ_PAYLOAD` reader"]
pub type R = crate::R<IBI_SIR_REQ_PAYLOAD_SPEC>;
#[doc = "Register `IBI_SIR_REQ_PAYLOAD` writer"]
pub type W = crate::W<IBI_SIR_REQ_PAYLOAD_SPEC>;
#[doc = "Field `REG_SIR_REQ_PAYLOAD` reader - NA"]
pub type REG_SIR_REQ_PAYLOAD_R = crate::FieldReader<u32>;
#[doc = "Field `REG_SIR_REQ_PAYLOAD` writer - NA"]
pub type REG_SIR_REQ_PAYLOAD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn reg_sir_req_payload(&self) -> REG_SIR_REQ_PAYLOAD_R {
        REG_SIR_REQ_PAYLOAD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBI_SIR_REQ_PAYLOAD")
            .field("reg_sir_req_payload", &self.reg_sir_req_payload())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sir_req_payload(&mut self) -> REG_SIR_REQ_PAYLOAD_W<IBI_SIR_REQ_PAYLOAD_SPEC> {
        REG_SIR_REQ_PAYLOAD_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`ibi_sir_req_payload::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ibi_sir_req_payload::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBI_SIR_REQ_PAYLOAD_SPEC;
impl crate::RegisterSpec for IBI_SIR_REQ_PAYLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibi_sir_req_payload::R`](R) reader structure"]
impl crate::Readable for IBI_SIR_REQ_PAYLOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibi_sir_req_payload::W`](W) writer structure"]
impl crate::Writable for IBI_SIR_REQ_PAYLOAD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBI_SIR_REQ_PAYLOAD to value 0"]
impl crate::Resettable for IBI_SIR_REQ_PAYLOAD_SPEC {
    const RESET_VALUE: u32 = 0;
}
