#[doc = "Register `PMT_RWUFFR` reader"]
pub type R = crate::R<PMT_RWUFFR_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PMT_RWUFFR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "The MSB (31st bit) must be zero.Bit j\\[30:0\\] is the byte mask. If Bit 1/2/3/4 (byte number) of the byte mask is set the CRC block processes the Filter 1/2/3/4 Offset + j of the incoming packet(PWKPTR is 0/1/2/3).RWKPTR is 0:Filter 0 Byte Mask .RWKPTR is 1:Filter 1 Byte Mask RWKPTR is 2:Filter 2 Byte Mask RWKPTR is 3:Filter 3 Byte Mask RWKPTR is 4:Bit 3/11/19/27 specifies the address type defining the destination address type of the pattern.When the bit is set the pattern applies to only multicast packets\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt_rwuffr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PMT_RWUFFR_SPEC;
impl crate::RegisterSpec for PMT_RWUFFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmt_rwuffr::R`](R) reader structure"]
impl crate::Readable for PMT_RWUFFR_SPEC {}
#[doc = "`reset()` method sets PMT_RWUFFR to value 0"]
impl crate::Resettable for PMT_RWUFFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
