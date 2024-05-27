///Register `QUICK_SENT` reader
pub type R = crate::R<QUICK_SENT_SPEC>;
///Register `QUICK_SENT` writer
pub type W = crate::W<QUICK_SENT_SPEC>;
///Field `SINGLE_SEND_NUM` reader - This register is used to specify the single_send register.
pub type SINGLE_SEND_NUM_R = crate::FieldReader;
///Field `SINGLE_SEND_NUM` writer - This register is used to specify the single_send register.
pub type SINGLE_SEND_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `SINGLE_SEND_EN` reader - Set this bit to enable single_send mode to send short packet.
pub type SINGLE_SEND_EN_R = crate::BitReader;
///Field `SINGLE_SEND_EN` writer - Set this bit to enable single_send mode to send short packet.
pub type SINGLE_SEND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALWAYS_SEND_NUM` reader - This register is used to specify the always_send register.
pub type ALWAYS_SEND_NUM_R = crate::FieldReader;
///Field `ALWAYS_SEND_NUM` writer - This register is used to specify the always_send register.
pub type ALWAYS_SEND_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ALWAYS_SEND_EN` reader - Set this bit to enable always_send mode to send short packet.
pub type ALWAYS_SEND_EN_R = crate::BitReader;
///Field `ALWAYS_SEND_EN` writer - Set this bit to enable always_send mode to send short packet.
pub type ALWAYS_SEND_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - This register is used to specify the single_send register.
    #[inline(always)]
    pub fn single_send_num(&self) -> SINGLE_SEND_NUM_R {
        SINGLE_SEND_NUM_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Set this bit to enable single_send mode to send short packet.
    #[inline(always)]
    pub fn single_send_en(&self) -> SINGLE_SEND_EN_R {
        SINGLE_SEND_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:6 - This register is used to specify the always_send register.
    #[inline(always)]
    pub fn always_send_num(&self) -> ALWAYS_SEND_NUM_R {
        ALWAYS_SEND_NUM_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bit 7 - Set this bit to enable always_send mode to send short packet.
    #[inline(always)]
    pub fn always_send_en(&self) -> ALWAYS_SEND_EN_R {
        ALWAYS_SEND_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("QUICK_SENT")
            .field("single_send_num", &self.single_send_num())
            .field("single_send_en", &self.single_send_en())
            .field("always_send_num", &self.always_send_num())
            .field("always_send_en", &self.always_send_en())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - This register is used to specify the single_send register.
    #[inline(always)]
    #[must_use]
    pub fn single_send_num(&mut self) -> SINGLE_SEND_NUM_W<QUICK_SENT_SPEC> {
        SINGLE_SEND_NUM_W::new(self, 0)
    }
    ///Bit 3 - Set this bit to enable single_send mode to send short packet.
    #[inline(always)]
    #[must_use]
    pub fn single_send_en(&mut self) -> SINGLE_SEND_EN_W<QUICK_SENT_SPEC> {
        SINGLE_SEND_EN_W::new(self, 3)
    }
    ///Bits 4:6 - This register is used to specify the always_send register.
    #[inline(always)]
    #[must_use]
    pub fn always_send_num(&mut self) -> ALWAYS_SEND_NUM_W<QUICK_SENT_SPEC> {
        ALWAYS_SEND_NUM_W::new(self, 4)
    }
    ///Bit 7 - Set this bit to enable always_send mode to send short packet.
    #[inline(always)]
    #[must_use]
    pub fn always_send_en(&mut self) -> ALWAYS_SEND_EN_W<QUICK_SENT_SPEC> {
        ALWAYS_SEND_EN_W::new(self, 7)
    }
}
/**UHCI quick send configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`quick_sent::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quick_sent::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct QUICK_SENT_SPEC;
impl crate::RegisterSpec for QUICK_SENT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`quick_sent::R`](R) reader structure
impl crate::Readable for QUICK_SENT_SPEC {}
///`write(|w| ..)` method takes [`quick_sent::W`](W) writer structure
impl crate::Writable for QUICK_SENT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets QUICK_SENT to value 0
impl crate::Resettable for QUICK_SENT_SPEC {
    const RESET_VALUE: u32 = 0;
}
