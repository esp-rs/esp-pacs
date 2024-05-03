#[doc = "Register `IBI_NOTIFY_CTRL` reader"]
pub type R = crate::R<IBI_NOTIFY_CTRL_SPEC>;
#[doc = "Register `IBI_NOTIFY_CTRL` writer"]
pub type W = crate::W<IBI_NOTIFY_CTRL_SPEC>;
#[doc = "Field `REG_NOTIFY_SIR_REJECTED` reader - Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl."]
pub type REG_NOTIFY_SIR_REJECTED_R = crate::BitReader;
#[doc = "Field `REG_NOTIFY_SIR_REJECTED` writer - Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl."]
pub type REG_NOTIFY_SIR_REJECTED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl."]
    #[inline(always)]
    pub fn reg_notify_sir_rejected(&self) -> REG_NOTIFY_SIR_REJECTED_R {
        REG_NOTIFY_SIR_REJECTED_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IBI_NOTIFY_CTRL")
            .field(
                "reg_notify_sir_rejected",
                &self.reg_notify_sir_rejected().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IBI_NOTIFY_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 2 - Notify Rejected Slave Interrupt Request Control. This bit is used to suppress reporting to the application about Slave Interrupt Request. 0:Suppress passing the IBI Status to the IBI FIFO(hence not notifying the application) when a SIR request is NACKed and auto-disabled base on the IBI_SIR_REQ_REJECT register. 1: Writes IBI Status to the IBI FIFO(hence notifying the application) when SIR request is NACKed and auto-disabled based on the IBI_SIR_REQ_REJECT registerl."]
    #[inline(always)]
    #[must_use]
    pub fn reg_notify_sir_rejected(&mut self) -> REG_NOTIFY_SIR_REJECTED_W<IBI_NOTIFY_CTRL_SPEC> {
        REG_NOTIFY_SIR_REJECTED_W::new(self, 2)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ibi_notify_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ibi_notify_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IBI_NOTIFY_CTRL_SPEC;
impl crate::RegisterSpec for IBI_NOTIFY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ibi_notify_ctrl::R`](R) reader structure"]
impl crate::Readable for IBI_NOTIFY_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ibi_notify_ctrl::W`](W) writer structure"]
impl crate::Writable for IBI_NOTIFY_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IBI_NOTIFY_CTRL to value 0"]
impl crate::Resettable for IBI_NOTIFY_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
