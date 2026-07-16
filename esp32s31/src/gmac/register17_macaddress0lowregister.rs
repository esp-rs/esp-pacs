#[doc = "Register `REGISTER17_MACADDRESS0LOWREGISTER` reader"]
pub type R = crate::R<REGISTER17_MACADDRESS0LOWREGISTER_SPEC>;
#[doc = "Register `REGISTER17_MACADDRESS0LOWREGISTER` writer"]
pub type W = crate::W<REGISTER17_MACADDRESS0LOWREGISTER_SPEC>;
#[doc = "Field `ADDRLO_0` reader - MAC Address0 \\[31:0\\] This field contains the lower 32 bits of the first 6byte MAC address This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
pub type ADDRLO_0_R = crate::FieldReader<u32>;
#[doc = "Field `ADDRLO_0` writer - MAC Address0 \\[31:0\\] This field contains the lower 32 bits of the first 6byte MAC address This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
pub type ADDRLO_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\] This field contains the lower 32 bits of the first 6byte MAC address This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
    #[inline(always)]
    pub fn addrlo_0(&self) -> ADDRLO_0_R {
        ADDRLO_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER17_MACADDRESS0LOWREGISTER")
            .field("addrlo_0", &self.addrlo_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Address0 \\[31:0\\] This field contains the lower 32 bits of the first 6byte MAC address This is used by the MAC for filtering the received frames and inserting the MAC address in the Transmit Flow Control _Pause_ Frames"]
    #[inline(always)]
    pub fn addrlo_0(&mut self) -> ADDRLO_0_W<'_, REGISTER17_MACADDRESS0LOWREGISTER_SPEC> {
        ADDRLO_0_W::new(self, 0)
    }
}
#[doc = "Contains the lower 32 bits of the first MAC address\n\nYou can [`read`](crate::Reg::read) this register and get [`register17_macaddress0lowregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register17_macaddress0lowregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER17_MACADDRESS0LOWREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER17_MACADDRESS0LOWREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register17_macaddress0lowregister::R`](R) reader structure"]
impl crate::Readable for REGISTER17_MACADDRESS0LOWREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register17_macaddress0lowregister::W`](W) writer structure"]
impl crate::Writable for REGISTER17_MACADDRESS0LOWREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER17_MACADDRESS0LOWREGISTER to value 0xffff_ffff"]
impl crate::Resettable for REGISTER17_MACADDRESS0LOWREGISTER_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
