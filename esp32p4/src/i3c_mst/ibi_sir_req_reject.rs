#[doc = "Register `IBI_SIR_REQ_REJECT` reader"]
pub type R = crate::R<IBI_SIR_REQ_REJECT_SPEC>;
#[doc = "Register `IBI_SIR_REQ_REJECT` writer"]
pub type W = crate::W<IBI_SIR_REQ_REJECT_SPEC>;
#[doc = "Field `REG_SIR_REQ_REJECT` reader - The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC"]
pub type REG_SIR_REQ_REJECT_R = crate::FieldReader<u32>;
#[doc = "Field `REG_SIR_REQ_REJECT` writer - The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC"]
pub type REG_SIR_REQ_REJECT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC"]
    #[inline(always)]
    pub fn reg_sir_req_reject(&self) -> REG_SIR_REQ_REJECT_R {
        REG_SIR_REQ_REJECT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBI_SIR_REQ_REJECT")
            .field(
                "reg_sir_req_reject",
                &format_args!("{}", self.reg_sir_req_reject().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBI_SIR_REQ_REJECT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application of controller can decide whether to send ACK or NACK for Slave request received from any I3C device. A device specific response control bit is provided to select the response option, Master will ACK/NACK the Master Request based on programming of control bit, corresponding to the interrupting device. 0:ACK the SIR Request 1:NACK and send direct auto disable CCC"]
    #[inline(always)]
    #[must_use]
    pub fn reg_sir_req_reject(&mut self) -> REG_SIR_REQ_REJECT_W<IBI_SIR_REQ_REJECT_SPEC> {
        REG_SIR_REQ_REJECT_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibi_sir_req_reject::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibi_sir_req_reject::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBI_SIR_REQ_REJECT_SPEC;
impl crate::RegisterSpec for IBI_SIR_REQ_REJECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibi_sir_req_reject::R`](R) reader structure"]
impl crate::Readable for IBI_SIR_REQ_REJECT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibi_sir_req_reject::W`](W) writer structure"]
impl crate::Writable for IBI_SIR_REQ_REJECT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IBI_SIR_REQ_REJECT to value 0"]
impl crate::Resettable for IBI_SIR_REQ_REJECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
