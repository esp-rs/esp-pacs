#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    gotgctl: GOTGCTL,
    gotgint: GOTGINT,
    gahbcfg: GAHBCFG,
    gusbcfg: GUSBCFG,
    grstctl: GRSTCTL,
    gintsts: GINTSTS,
    gintmsk: GINTMSK,
    grxstsr: GRXSTSR,
    grxstsp: GRXSTSP,
    grxfsiz: GRXFSIZ,
    gnptxfsiz: GNPTXFSIZ,
    gnptxsts: GNPTXSTS,
    _reserved12: [u8; 0x04],
    gpvndctl: GPVNDCTL,
    _reserved13: [u8; 0x08],
    gsnpsid: GSNPSID,
    ghwcfg1: GHWCFG1,
    ghwcfg2: GHWCFG2,
    ghwcfg3: GHWCFG3,
    ghwcfg4: GHWCFG4,
    _reserved18: [u8; 0x08],
    gdfifocfg: GDFIFOCFG,
    _reserved19: [u8; 0x08],
    gintmsk2: GINTMSK2,
    gintsts2: GINTSTS2,
    _reserved21: [u8; 0x90],
    hptxfsiz: HPTXFSIZ,
    dieptxf1: DIEPTXF1,
    dieptxf2: DIEPTXF2,
    dieptxf3: DIEPTXF3,
    dieptxf4: DIEPTXF4,
    dieptxf5: DIEPTXF5,
    dieptxf6: DIEPTXF6,
    dieptxf7: DIEPTXF7,
    _reserved29: [u8; 0x02e0],
    hcfg: HCFG,
    hfir: HFIR,
    hfnum: HFNUM,
    _reserved32: [u8; 0x04],
    hptxsts: HPTXSTS,
    haint: HAINT,
    haintmsk: HAINTMSK,
    hflbaddr: HFLBADDR,
    _reserved36: [u8; 0x20],
    hprt: HPRT,
    _reserved37: [u8; 0xbc],
    hcchar0: HCCHAR0,
    hcsplt0: HCSPLT0,
    hcint0: HCINT0,
    hcintmsk0: HCINTMSK0,
    hctsiz0: HCTSIZ0,
    hcdma0: HCDMA0,
    _reserved43: [u8; 0x04],
    hcdmab0: HCDMAB0,
    hcchar1: HCCHAR1,
    hcsplt1: HCSPLT1,
    hcint1: HCINT1,
    hcintmsk1: HCINTMSK1,
    hctsiz1: HCTSIZ1,
    hcdma1: HCDMA1,
    _reserved50: [u8; 0x04],
    hcdmab1: HCDMAB1,
    hcchar2: HCCHAR2,
    hcsplt2: HCSPLT2,
    hcint2: HCINT2,
    hcintmsk2: HCINTMSK2,
    hctsiz2: HCTSIZ2,
    hcdma2: HCDMA2,
    _reserved57: [u8; 0x04],
    hcdmab2: HCDMAB2,
    hcchar3: HCCHAR3,
    hcsplt3: HCSPLT3,
    hcint3: HCINT3,
    hcintmsk3: HCINTMSK3,
    hctsiz3: HCTSIZ3,
    hcdma3: HCDMA3,
    _reserved64: [u8; 0x04],
    hcdmab3: HCDMAB3,
    hcchar4: HCCHAR4,
    hcsplt4: HCSPLT4,
    hcint4: HCINT4,
    hcintmsk4: HCINTMSK4,
    hctsiz4: HCTSIZ4,
    hcdma4: HCDMA4,
    _reserved71: [u8; 0x04],
    hcdmab4: HCDMAB4,
    hcchar5: HCCHAR5,
    hcsplt5: HCSPLT5,
    hcint5: HCINT5,
    hcintmsk5: HCINTMSK5,
    hctsiz5: HCTSIZ5,
    hcdma5: HCDMA5,
    _reserved78: [u8; 0x04],
    hcdmab5: HCDMAB5,
    hcchar6: HCCHAR6,
    hcsplt6: HCSPLT6,
    hcint6: HCINT6,
    hcintmsk6: HCINTMSK6,
    hctsiz6: HCTSIZ6,
    hcdma6: HCDMA6,
    _reserved85: [u8; 0x04],
    hcdmab6: HCDMAB6,
    hcchar7: HCCHAR7,
    hcsplt7: HCSPLT7,
    hcint7: HCINT7,
    hcintmsk7: HCINTMSK7,
    hctsiz7: HCTSIZ7,
    hcdma7: HCDMA7,
    _reserved92: [u8; 0x04],
    hcdmab7: HCDMAB7,
    hcchar8: HCCHAR8,
    hcsplt8: HCSPLT8,
    hcint8: HCINT8,
    hcintmsk8: HCINTMSK8,
    hctsiz8: HCTSIZ8,
    hcdma8: HCDMA8,
    _reserved99: [u8; 0x04],
    hcdmab8: HCDMAB8,
    hcchar9: HCCHAR9,
    hcsplt9: HCSPLT9,
    hcint9: HCINT9,
    hcintmsk9: HCINTMSK9,
    hctsiz9: HCTSIZ9,
    hcdma9: HCDMA9,
    _reserved106: [u8; 0x04],
    hcdmab9: HCDMAB9,
    hcchar10: HCCHAR10,
    hcsplt10: HCSPLT10,
    hcint10: HCINT10,
    hcintmsk10: HCINTMSK10,
    hctsiz10: HCTSIZ10,
    hcdma10: HCDMA10,
    _reserved113: [u8; 0x04],
    hcdmab10: HCDMAB10,
    hcchar11: HCCHAR11,
    hcsplt11: HCSPLT11,
    hcint11: HCINT11,
    hcintmsk11: HCINTMSK11,
    hctsiz11: HCTSIZ11,
    hcdma11: HCDMA11,
    _reserved120: [u8; 0x04],
    hcdmab11: HCDMAB11,
    hcchar12: HCCHAR12,
    hcsplt12: HCSPLT12,
    hcint12: HCINT12,
    hcintmsk12: HCINTMSK12,
    hctsiz12: HCTSIZ12,
    hcdma12: HCDMA12,
    _reserved127: [u8; 0x04],
    hcdmab12: HCDMAB12,
    hcchar13: HCCHAR13,
    hcsplt13: HCSPLT13,
    hcint13: HCINT13,
    hcintmsk13: HCINTMSK13,
    hctsiz13: HCTSIZ13,
    hcdma13: HCDMA13,
    _reserved134: [u8; 0x04],
    hcdmab13: HCDMAB13,
    hcchar14: HCCHAR14,
    hcsplt14: HCSPLT14,
    hcint14: HCINT14,
    hcintmsk14: HCINTMSK14,
    hctsiz14: HCTSIZ14,
    hcdma14: HCDMA14,
    _reserved141: [u8; 0x04],
    hcdmab14: HCDMAB14,
    hcchar15: HCCHAR15,
    hcsplt15: HCSPLT15,
    hcint15: HCINT15,
    hcintmsk15: HCINTMSK15,
    hctsiz15: HCTSIZ15,
    hcdma15: HCDMA15,
    _reserved148: [u8; 0x04],
    hcdmab15: HCDMAB15,
    _reserved149: [u8; 0x0100],
    dcfg: DCFG,
    dctl: DCTL,
    dsts: DSTS,
    _reserved152: [u8; 0x04],
    diepmsk: DIEPMSK,
    doepmsk: DOEPMSK,
    daint: DAINT,
    daintmsk: DAINTMSK,
    _reserved156: [u8; 0x08],
    dvbusdis: DVBUSDIS,
    dvbuspulse: DVBUSPULSE,
    dthrctl: DTHRCTL,
    diepempmsk: DIEPEMPMSK,
    deachint: DEACHINT,
    deachintmsk: DEACHINTMSK,
    diepeachmsk0: DIEPEACHMSK0,
    diepeachmsk1: DIEPEACHMSK1,
    diepeachmsk2: DIEPEACHMSK2,
    diepeachmsk3: DIEPEACHMSK3,
    diepeachmsk4: DIEPEACHMSK4,
    diepeachmsk5: DIEPEACHMSK5,
    diepeachmsk6: DIEPEACHMSK6,
    diepeachmsk7: DIEPEACHMSK7,
    diepeachmsk8: DIEPEACHMSK8,
    diepeachmsk9: DIEPEACHMSK9,
    diepeachmsk10: DIEPEACHMSK10,
    diepeachmsk11: DIEPEACHMSK11,
    diepeachmsk12: DIEPEACHMSK12,
    diepeachmsk13: DIEPEACHMSK13,
    diepeachmsk14: DIEPEACHMSK14,
    diepeachmsk15: DIEPEACHMSK15,
    doepeachmsk0: DOEPEACHMSK0,
    doepeachmsk1: DOEPEACHMSK1,
    doepeachmsk2: DOEPEACHMSK2,
    doepeachmsk3: DOEPEACHMSK3,
    doepeachmsk4: DOEPEACHMSK4,
    doepeachmsk5: DOEPEACHMSK5,
    doepeachmsk6: DOEPEACHMSK6,
    doepeachmsk7: DOEPEACHMSK7,
    doepeachmsk8: DOEPEACHMSK8,
    doepeachmsk9: DOEPEACHMSK9,
    doepeachmsk10: DOEPEACHMSK10,
    doepeachmsk11: DOEPEACHMSK11,
    doepeachmsk12: DOEPEACHMSK12,
    doepeachmsk13: DOEPEACHMSK13,
    doepeachmsk14: DOEPEACHMSK14,
    doepeachmsk15: DOEPEACHMSK15,
    _reserved194: [u8; 0x40],
    diepctl0: DIEPCTL0,
    _reserved195: [u8; 0x04],
    diepint0: DIEPINT0,
    _reserved196: [u8; 0x04],
    dieptsiz0: DIEPTSIZ0,
    diepdma0: DIEPDMA0,
    dtxfsts0: DTXFSTS0,
    diepdmab0: DIEPDMAB0,
    diepctl1: DIEPCTL1,
    _reserved201: [u8; 0x04],
    diepint1: DIEPINT1,
    _reserved202: [u8; 0x04],
    dieptsiz1: DIEPTSIZ1,
    diepdma1: DIEPDMA1,
    dtxfsts1: DTXFSTS1,
    diepdmab1: DIEPDMAB1,
    diepctl2: DIEPCTL2,
    _reserved207: [u8; 0x04],
    diepint2: DIEPINT2,
    _reserved208: [u8; 0x04],
    dieptsiz2: DIEPTSIZ2,
    diepdma2: DIEPDMA2,
    dtxfsts2: DTXFSTS2,
    diepdmab2: DIEPDMAB2,
    diepctl3: DIEPCTL3,
    _reserved213: [u8; 0x04],
    diepint3: DIEPINT3,
    _reserved214: [u8; 0x04],
    dieptsiz3: DIEPTSIZ3,
    diepdma3: DIEPDMA3,
    dtxfsts3: DTXFSTS3,
    diepdmab3: DIEPDMAB3,
    diepctl4: DIEPCTL4,
    _reserved219: [u8; 0x04],
    diepint4: DIEPINT4,
    _reserved220: [u8; 0x04],
    dieptsiz4: DIEPTSIZ4,
    diepdma4: DIEPDMA4,
    dtxfsts4: DTXFSTS4,
    diepdmab4: DIEPDMAB4,
    diepctl5: DIEPCTL5,
    _reserved225: [u8; 0x04],
    diepint5: DIEPINT5,
    _reserved226: [u8; 0x04],
    dieptsiz5: DIEPTSIZ5,
    diepdma5: DIEPDMA5,
    dtxfsts5: DTXFSTS5,
    diepdmab5: DIEPDMAB5,
    diepctl6: DIEPCTL6,
    _reserved231: [u8; 0x04],
    diepint6: DIEPINT6,
    _reserved232: [u8; 0x04],
    dieptsiz6: DIEPTSIZ6,
    diepdma6: DIEPDMA6,
    dtxfsts6: DTXFSTS6,
    diepdmab6: DIEPDMAB6,
    diepctl7: DIEPCTL7,
    _reserved237: [u8; 0x04],
    diepint7: DIEPINT7,
    _reserved238: [u8; 0x04],
    dieptsiz7: DIEPTSIZ7,
    diepdma7: DIEPDMA7,
    dtxfsts7: DTXFSTS7,
    diepdmab7: DIEPDMAB7,
    diepctl8: DIEPCTL8,
    _reserved243: [u8; 0x04],
    diepint8: DIEPINT8,
    _reserved244: [u8; 0x04],
    dieptsiz8: DIEPTSIZ8,
    diepdma8: DIEPDMA8,
    dtxfsts8: DTXFSTS8,
    diepdmab8: DIEPDMAB8,
    diepctl9: DIEPCTL9,
    _reserved249: [u8; 0x04],
    diepint9: DIEPINT9,
    _reserved250: [u8; 0x04],
    dieptsiz9: DIEPTSIZ9,
    diepdma9: DIEPDMA9,
    dtxfsts9: DTXFSTS9,
    diepdmab9: DIEPDMAB9,
    diepctl10: DIEPCTL10,
    _reserved255: [u8; 0x04],
    diepint10: DIEPINT10,
    _reserved256: [u8; 0x04],
    dieptsiz10: DIEPTSIZ10,
    diepdma10: DIEPDMA10,
    dtxfsts10: DTXFSTS10,
    diepdmab10: DIEPDMAB10,
    diepctl11: DIEPCTL11,
    _reserved261: [u8; 0x04],
    diepint11: DIEPINT11,
    _reserved262: [u8; 0x04],
    dieptsiz11: DIEPTSIZ11,
    diepdma11: DIEPDMA11,
    dtxfsts11: DTXFSTS11,
    diepdmab11: DIEPDMAB11,
    diepctl12: DIEPCTL12,
    _reserved267: [u8; 0x04],
    diepint12: DIEPINT12,
    _reserved268: [u8; 0x04],
    dieptsiz12: DIEPTSIZ12,
    diepdma12: DIEPDMA12,
    dtxfsts12: DTXFSTS12,
    diepdmab12: DIEPDMAB12,
    diepctl13: DIEPCTL13,
    _reserved273: [u8; 0x04],
    diepint13: DIEPINT13,
    _reserved274: [u8; 0x04],
    dieptsiz13: DIEPTSIZ13,
    diepdma13: DIEPDMA13,
    dtxfsts13: DTXFSTS13,
    diepdmab13: DIEPDMAB13,
    diepctl14: DIEPCTL14,
    _reserved279: [u8; 0x04],
    diepint14: DIEPINT14,
    _reserved280: [u8; 0x04],
    dieptsiz14: DIEPTSIZ14,
    diepdma14: DIEPDMA14,
    dtxfsts14: DTXFSTS14,
    diepdmab14: DIEPDMAB14,
    diepctl15: DIEPCTL15,
    _reserved285: [u8; 0x04],
    diepint15: DIEPINT15,
    _reserved286: [u8; 0x04],
    dieptsiz15: DIEPTSIZ15,
    diepdma15: DIEPDMA15,
    dtxfsts15: DTXFSTS15,
    diepdmab15: DIEPDMAB15,
    doepctl0: DOEPCTL0,
    _reserved291: [u8; 0x04],
    doepint0: DOEPINT0,
    _reserved292: [u8; 0x04],
    doeptsiz0: DOEPTSIZ0,
    doepdma0: DOEPDMA0,
    _reserved294: [u8; 0x04],
    doepdmab0: DOEPDMAB0,
    doepctl1: DOEPCTL1,
    _reserved296: [u8; 0x04],
    doepint1: DOEPINT1,
    _reserved297: [u8; 0x04],
    doeptsiz1: DOEPTSIZ1,
    doepdma1: DOEPDMA1,
    _reserved299: [u8; 0x04],
    doepdmab1: DOEPDMAB1,
    doepctl2: DOEPCTL2,
    _reserved301: [u8; 0x04],
    doepint2: DOEPINT2,
    _reserved302: [u8; 0x04],
    doeptsiz2: DOEPTSIZ2,
    doepdma2: DOEPDMA2,
    _reserved304: [u8; 0x04],
    doepdmab2: DOEPDMAB2,
    doepctl3: DOEPCTL3,
    _reserved306: [u8; 0x04],
    doepint3: DOEPINT3,
    _reserved307: [u8; 0x04],
    doeptsiz3: DOEPTSIZ3,
    doepdma3: DOEPDMA3,
    _reserved309: [u8; 0x04],
    doepdmab3: DOEPDMAB3,
    doepctl4: DOEPCTL4,
    _reserved311: [u8; 0x04],
    doepint4: DOEPINT4,
    _reserved312: [u8; 0x04],
    doeptsiz4: DOEPTSIZ4,
    doepdma4: DOEPDMA4,
    _reserved314: [u8; 0x04],
    doepdmab4: DOEPDMAB4,
    doepctl5: DOEPCTL5,
    _reserved316: [u8; 0x04],
    doepint5: DOEPINT5,
    _reserved317: [u8; 0x04],
    doeptsiz5: DOEPTSIZ5,
    doepdma5: DOEPDMA5,
    _reserved319: [u8; 0x04],
    doepdmab5: DOEPDMAB5,
    doepctl6: DOEPCTL6,
    _reserved321: [u8; 0x04],
    doepint6: DOEPINT6,
    _reserved322: [u8; 0x04],
    doeptsiz6: DOEPTSIZ6,
    doepdma6: DOEPDMA6,
    _reserved324: [u8; 0x04],
    doepdmab6: DOEPDMAB6,
    doepctl7: DOEPCTL7,
    _reserved326: [u8; 0x04],
    doepint7: DOEPINT7,
    _reserved327: [u8; 0x04],
    doeptsiz7: DOEPTSIZ7,
    doepdma7: DOEPDMA7,
    _reserved329: [u8; 0x04],
    doepdmab7: DOEPDMAB7,
    doepctl8: DOEPCTL8,
    _reserved331: [u8; 0x04],
    doepint8: DOEPINT8,
    _reserved332: [u8; 0x04],
    doeptsiz8: DOEPTSIZ8,
    doepdma8: DOEPDMA8,
    _reserved334: [u8; 0x04],
    doepdmab8: DOEPDMAB8,
    doepctl9: DOEPCTL9,
    _reserved336: [u8; 0x04],
    doepint9: DOEPINT9,
    _reserved337: [u8; 0x04],
    doeptsiz9: DOEPTSIZ9,
    doepdma9: DOEPDMA9,
    _reserved339: [u8; 0x04],
    doepdmab9: DOEPDMAB9,
    doepctl10: DOEPCTL10,
    _reserved341: [u8; 0x04],
    doepint10: DOEPINT10,
    _reserved342: [u8; 0x04],
    doeptsiz10: DOEPTSIZ10,
    doepdma10: DOEPDMA10,
    _reserved344: [u8; 0x04],
    doepdmab10: DOEPDMAB10,
    doepctl11: DOEPCTL11,
    _reserved346: [u8; 0x04],
    doepint11: DOEPINT11,
    _reserved347: [u8; 0x04],
    doeptsiz11: DOEPTSIZ11,
    doepdma11: DOEPDMA11,
    _reserved349: [u8; 0x04],
    doepdmab11: DOEPDMAB11,
    doepctl12: DOEPCTL12,
    _reserved351: [u8; 0x04],
    doepint12: DOEPINT12,
    _reserved352: [u8; 0x04],
    doeptsiz12: DOEPTSIZ12,
    doepdma12: DOEPDMA12,
    _reserved354: [u8; 0x04],
    doepdmab12: DOEPDMAB12,
    doepctl13: DOEPCTL13,
    _reserved356: [u8; 0x04],
    doepint13: DOEPINT13,
    _reserved357: [u8; 0x04],
    doeptsiz13: DOEPTSIZ13,
    doepdma13: DOEPDMA13,
    _reserved359: [u8; 0x04],
    doepdmab13: DOEPDMAB13,
    doepctl14: DOEPCTL14,
    _reserved361: [u8; 0x04],
    doepint14: DOEPINT14,
    _reserved362: [u8; 0x04],
    doeptsiz14: DOEPTSIZ14,
    doepdma14: DOEPDMA14,
    _reserved364: [u8; 0x04],
    doepdmab14: DOEPDMAB14,
    doepctl15: DOEPCTL15,
    _reserved366: [u8; 0x04],
    doepint15: DOEPINT15,
    _reserved367: [u8; 0x04],
    doeptsiz15: DOEPTSIZ15,
    doepdma15: DOEPDMA15,
    _reserved369: [u8; 0x04],
    doepdmab15: DOEPDMAB15,
    _reserved370: [u8; 0x0100],
    pcgcctl: PCGCCTL,
    _reserved371: [u8; 0xfc],
    gstarfxdis: GSTARFXDIS,
}
impl RegisterBlock {
    #[doc = "0x00 - The OTG Control and Status register controls the behavior and reflects the status of the OTG function of the controller."]
    #[inline(always)]
    pub const fn gotgctl(&self) -> &GOTGCTL {
        &self.gotgctl
    }
    #[doc = "0x04 - The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
    #[inline(always)]
    pub const fn gotgint(&self) -> &GOTGINT {
        &self.gotgint
    }
    #[doc = "0x08 - This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
    #[inline(always)]
    pub const fn gahbcfg(&self) -> &GAHBCFG {
        &self.gahbcfg
    }
    #[doc = "0x0c - This register can be used to configure the core after power-on or when changing to Host mode or Device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. If you are using the HSIC interface, HSIC PHY must be in reset while programming this register. Do not make changes to this register after the initial programming."]
    #[inline(always)]
    pub const fn gusbcfg(&self) -> &GUSBCFG {
        &self.gusbcfg
    }
    #[doc = "0x10 - The application uses this register to reset various hardware features inside the controller."]
    #[inline(always)]
    pub const fn grstctl(&self) -> &GRSTCTL {
        &self.grstctl
    }
    #[doc = "0x14 - This register interrupts the application for system-level events in the current mode (Device mode or Host mode). Some of the bits in this register are valid only in Host mode, while others are valid in Device mode only. This register also indicates the current mode. To clear the interrupt status bits of type R_SS_WC, the application must write 1'b1 to the bit. The FIFO status interrupts are read only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization. Note: Read the reset value of GINTSTS.CurMod only after the following conditions: - If IDDIG_FILTER is disabled, read only after PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires."]
    #[inline(always)]
    pub const fn gintsts(&self) -> &GINTSTS {
        &self.gintsts
    }
    #[doc = "0x18 - This register works with the Interrupt Register (GINTSTS) to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the GINTSTS register bit corresponding to that interrupt is still set. Note: The fields of this register change depending on host or device mode."]
    #[inline(always)]
    pub const fn gintmsk(&self) -> &GINTMSK {
        &self.gintmsk
    }
    #[doc = "0x1c - A read to the Receive Status Debug Read register returns the contents of the top of the Receive FIFO. The receive status contents must be interpreted differently in Host and Device modes. The core ignores the receive status read when the receive FIFO is empty and returns a value of 32'h0000_0000. Note: - Use of these fields vary based on whether the core is functioning as a host or a device. - Do not read this register's reset value before configuring the core because the read value is 'X' in the simulation."]
    #[inline(always)]
    pub const fn grxstsr(&self) -> &GRXSTSR {
        &self.grxstsr
    }
    #[doc = "0x20 - A read to the Receive Status Read and Pop register returns the contents of the top of the Receive FIFO and additionally pops the top data entry out of the RxFIFO. The receive status contents must be interpreted differently in Host and Device modes. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 32'h0000_0000. The application must only pop the Receive Status FIFO when the Receive FIFO Non-Empty bit of the Core Interrupt register (GINTSTS.RxFLvl) is asserted. Note: - Use of these fields vary based on whether the core is functioning as a host or a device. - Do not read this register's reset value before configuring the core because the read value is 'X' in the simulation."]
    #[inline(always)]
    pub const fn grxstsp(&self) -> &GRXSTSP {
        &self.grxstsp
    }
    #[doc = "0x24 - The application can program the RAM size that must be allocated to the RxFIFO."]
    #[inline(always)]
    pub const fn grxfsiz(&self) -> &GRXFSIZ {
        &self.grxfsiz
    }
    #[doc = "0x28 - The application can program the RAM size and the memory start address for the Non-periodic TxFIFO Note: The fields of this register change depending on host or device mode."]
    #[inline(always)]
    pub const fn gnptxfsiz(&self) -> &GNPTXFSIZ {
        &self.gnptxfsiz
    }
    #[doc = "0x2c - In Device mode, this register is valid only in Shared FIFO operation. This read-only register contains the free space information for the Non-periodic TxFIFO and the Non-periodic Transmit Request Queue."]
    #[inline(always)]
    pub const fn gnptxsts(&self) -> &GNPTXSTS {
        &self.gnptxsts
    }
    #[doc = "0x34 - The application can use this register to access PHY registers. It is implemented only if Enable PHY Vendor Control Interface was selected during coreConsultant configuration (parameter OTG_VENDOR_CTL_INTERFACE = 1). For a UTMI+ PHY, the DWC_otg core uses the UTMI+ Vendor Control interface for PHY register access. For a ULPI PHY, the core uses the ULPI interface for PHY register access. The application sets Vendor Control register for PHY register access and times the PHY register access. The application polls the VStatus Done bit in this register for the completion of the PHY register access."]
    #[inline(always)]
    pub const fn gpvndctl(&self) -> &GPVNDCTL {
        &self.gpvndctl
    }
    #[doc = "0x40 - This read-only register contains the release number of the core being used."]
    #[inline(always)]
    pub const fn gsnpsid(&self) -> &GSNPSID {
        &self.gsnpsid
    }
    #[doc = "0x44 - This register contains the logical endpoint direction(s) selected using coreConsultant."]
    #[inline(always)]
    pub const fn ghwcfg1(&self) -> &GHWCFG1 {
        &self.ghwcfg1
    }
    #[doc = "0x48 - This register contains configuration options selected using coreConsultant."]
    #[inline(always)]
    pub const fn ghwcfg2(&self) -> &GHWCFG2 {
        &self.ghwcfg2
    }
    #[doc = "0x4c - This register contains configuration options selected using coreConsultant."]
    #[inline(always)]
    pub const fn ghwcfg3(&self) -> &GHWCFG3 {
        &self.ghwcfg3
    }
    #[doc = "0x50 - This register contains configuration options selected using coreConsultant. Note: Bit \\[31\\] is available only when Scatter/Gather DMA mode is enabled. When Scatter/Gather DMA mode is disabled, this field is reserved."]
    #[inline(always)]
    pub const fn ghwcfg4(&self) -> &GHWCFG4 {
        &self.ghwcfg4
    }
    #[doc = "0x5c - Register to configure the DFIFOs for the controller."]
    #[inline(always)]
    pub const fn gdfifocfg(&self) -> &GDFIFOCFG {
        &self.gdfifocfg
    }
    #[doc = "0x68 - This register works with the Interrupt Register (GINTSTS2) to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the GINTSTS2 register bit corresponding to that interrupt is still set. Note: The fields of this register change depending on host or device mode."]
    #[inline(always)]
    pub const fn gintmsk2(&self) -> &GINTMSK2 {
        &self.gintmsk2
    }
    #[doc = "0x6c - This register interrupts the application for system-level events in the current mode (Device mode or Host mode). Some of the bits in this register are valid only in Host mode, while others are valid in Device mode only. This register also indicates the current mode. To clear the interrupt status bits of type R_SS_WC, the application must write 1'b1 to the bit. The application must clear the GINTSTS2 register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
    #[inline(always)]
    pub const fn gintsts2(&self) -> &GINTSTS2 {
        &self.gintsts2
    }
    #[doc = "0x100 - This register holds the size and the memory start address of the Periodic TxFIFO. Note: Read the reset value of this register only after the following conditions: - If IDDIG_FILTER is disabled, read only after PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires."]
    #[inline(always)]
    pub const fn hptxfsiz(&self) -> &HPTXFSIZ {
        &self.hptxfsiz
    }
    #[doc = "0x104 - This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
    #[inline(always)]
    pub const fn dieptxf1(&self) -> &DIEPTXF1 {
        &self.dieptxf1
    }
    #[doc = "0x108 - This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
    #[inline(always)]
    pub const fn dieptxf2(&self) -> &DIEPTXF2 {
        &self.dieptxf2
    }
    #[doc = "0x10c - This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
    #[inline(always)]
    pub const fn dieptxf3(&self) -> &DIEPTXF3 {
        &self.dieptxf3
    }
    #[doc = "0x110 - This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
    #[inline(always)]
    pub const fn dieptxf4(&self) -> &DIEPTXF4 {
        &self.dieptxf4
    }
    #[doc = "0x114 - This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
    #[inline(always)]
    pub const fn dieptxf5(&self) -> &DIEPTXF5 {
        &self.dieptxf5
    }
    #[doc = "0x118 - This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
    #[inline(always)]
    pub const fn dieptxf6(&self) -> &DIEPTXF6 {
        &self.dieptxf6
    }
    #[doc = "0x11c - This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
    #[inline(always)]
    pub const fn dieptxf7(&self) -> &DIEPTXF7 {
        &self.dieptxf7
    }
    #[doc = "0x400 - This register is used to configure the controller in Host mode."]
    #[inline(always)]
    pub const fn hcfg(&self) -> &HCFG {
        &self.hcfg
    }
    #[doc = "0x404 - This register is used to control the interval between two consecutive SOFs."]
    #[inline(always)]
    pub const fn hfir(&self) -> &HFIR {
        &self.hfir
    }
    #[doc = "0x408 - This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current (micro)frame. Note: Read the reset value of this register only after the following conditions: - If IDDIG_FILTER is disabled, read only when the PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires."]
    #[inline(always)]
    pub const fn hfnum(&self) -> &HFNUM {
        &self.hfnum
    }
    #[doc = "0x410 - This register contains information about the Periodic Transmit Queue in the Host controller."]
    #[inline(always)]
    pub const fn hptxsts(&self) -> &HPTXSTS {
        &self.hptxsts
    }
    #[doc = "0x414 - When a significant event occurs on a channel, the Host All Channels Interrupt register interrupts the application using the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt). This is shown in the Interrupt Hierarchy figure in the databook. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Host Channel-n Interrupt register."]
    #[inline(always)]
    pub const fn haint(&self) -> &HAINT {
        &self.haint
    }
    #[doc = "0x418 - The Host All Channel Interrupt Mask register works with the Host All Channel Interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
    #[inline(always)]
    pub const fn haintmsk(&self) -> &HAINTMSK {
        &self.haintmsk
    }
    #[doc = "0x41c - This register is present only in case of Scatter/Gather DMA. It is implemented as flops. This register holds the starting address of the Frame list information."]
    #[inline(always)]
    pub const fn hflbaddr(&self) -> &HFLBADDR {
        &self.hflbaddr
    }
    #[doc = "0x440 - This register is available only in Host mode. Currently, the OTG Host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in the Interrupt Hierarchy figure in the databook. The R_SS_WC bits in this register can trigger an interrupt to the application through the Host Port Interrupt bit of the Core Interrupt register (GINTSTS.PrtInt). On a Port Interrupt, the application must read this register and clear the bit that caused the interrupt. For the R_SS_WC bits, the application must write a 1 to the bit to clear the interrupt."]
    #[inline(always)]
    pub const fn hprt(&self) -> &HPRT {
        &self.hprt
    }
    #[doc = "0x500 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar0(&self) -> &HCCHAR0 {
        &self.hcchar0
    }
    #[doc = "0x504 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt0(&self) -> &HCSPLT0 {
        &self.hcsplt0
    }
    #[doc = "0x508 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint0(&self) -> &HCINT0 {
        &self.hcint0
    }
    #[doc = "0x50c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk0(&self) -> &HCINTMSK0 {
        &self.hcintmsk0
    }
    #[doc = "0x510 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz0(&self) -> &HCTSIZ0 {
        &self.hctsiz0
    }
    #[doc = "0x514 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma0(&self) -> &HCDMA0 {
        &self.hcdma0
    }
    #[doc = "0x51c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab0(&self) -> &HCDMAB0 {
        &self.hcdmab0
    }
    #[doc = "0x520 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar1(&self) -> &HCCHAR1 {
        &self.hcchar1
    }
    #[doc = "0x524 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt1(&self) -> &HCSPLT1 {
        &self.hcsplt1
    }
    #[doc = "0x528 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint1(&self) -> &HCINT1 {
        &self.hcint1
    }
    #[doc = "0x52c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk1(&self) -> &HCINTMSK1 {
        &self.hcintmsk1
    }
    #[doc = "0x530 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz1(&self) -> &HCTSIZ1 {
        &self.hctsiz1
    }
    #[doc = "0x534 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma1(&self) -> &HCDMA1 {
        &self.hcdma1
    }
    #[doc = "0x53c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab1(&self) -> &HCDMAB1 {
        &self.hcdmab1
    }
    #[doc = "0x540 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar2(&self) -> &HCCHAR2 {
        &self.hcchar2
    }
    #[doc = "0x544 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt2(&self) -> &HCSPLT2 {
        &self.hcsplt2
    }
    #[doc = "0x548 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint2(&self) -> &HCINT2 {
        &self.hcint2
    }
    #[doc = "0x54c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk2(&self) -> &HCINTMSK2 {
        &self.hcintmsk2
    }
    #[doc = "0x550 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz2(&self) -> &HCTSIZ2 {
        &self.hctsiz2
    }
    #[doc = "0x554 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma2(&self) -> &HCDMA2 {
        &self.hcdma2
    }
    #[doc = "0x55c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab2(&self) -> &HCDMAB2 {
        &self.hcdmab2
    }
    #[doc = "0x560 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar3(&self) -> &HCCHAR3 {
        &self.hcchar3
    }
    #[doc = "0x564 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt3(&self) -> &HCSPLT3 {
        &self.hcsplt3
    }
    #[doc = "0x568 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint3(&self) -> &HCINT3 {
        &self.hcint3
    }
    #[doc = "0x56c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk3(&self) -> &HCINTMSK3 {
        &self.hcintmsk3
    }
    #[doc = "0x570 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz3(&self) -> &HCTSIZ3 {
        &self.hctsiz3
    }
    #[doc = "0x574 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma3(&self) -> &HCDMA3 {
        &self.hcdma3
    }
    #[doc = "0x57c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab3(&self) -> &HCDMAB3 {
        &self.hcdmab3
    }
    #[doc = "0x580 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar4(&self) -> &HCCHAR4 {
        &self.hcchar4
    }
    #[doc = "0x584 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt4(&self) -> &HCSPLT4 {
        &self.hcsplt4
    }
    #[doc = "0x588 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint4(&self) -> &HCINT4 {
        &self.hcint4
    }
    #[doc = "0x58c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk4(&self) -> &HCINTMSK4 {
        &self.hcintmsk4
    }
    #[doc = "0x590 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz4(&self) -> &HCTSIZ4 {
        &self.hctsiz4
    }
    #[doc = "0x594 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma4(&self) -> &HCDMA4 {
        &self.hcdma4
    }
    #[doc = "0x59c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab4(&self) -> &HCDMAB4 {
        &self.hcdmab4
    }
    #[doc = "0x5a0 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar5(&self) -> &HCCHAR5 {
        &self.hcchar5
    }
    #[doc = "0x5a4 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt5(&self) -> &HCSPLT5 {
        &self.hcsplt5
    }
    #[doc = "0x5a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint5(&self) -> &HCINT5 {
        &self.hcint5
    }
    #[doc = "0x5ac - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk5(&self) -> &HCINTMSK5 {
        &self.hcintmsk5
    }
    #[doc = "0x5b0 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz5(&self) -> &HCTSIZ5 {
        &self.hctsiz5
    }
    #[doc = "0x5b4 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma5(&self) -> &HCDMA5 {
        &self.hcdma5
    }
    #[doc = "0x5bc - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab5(&self) -> &HCDMAB5 {
        &self.hcdmab5
    }
    #[doc = "0x5c0 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar6(&self) -> &HCCHAR6 {
        &self.hcchar6
    }
    #[doc = "0x5c4 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt6(&self) -> &HCSPLT6 {
        &self.hcsplt6
    }
    #[doc = "0x5c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint6(&self) -> &HCINT6 {
        &self.hcint6
    }
    #[doc = "0x5cc - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk6(&self) -> &HCINTMSK6 {
        &self.hcintmsk6
    }
    #[doc = "0x5d0 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz6(&self) -> &HCTSIZ6 {
        &self.hctsiz6
    }
    #[doc = "0x5d4 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma6(&self) -> &HCDMA6 {
        &self.hcdma6
    }
    #[doc = "0x5dc - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab6(&self) -> &HCDMAB6 {
        &self.hcdmab6
    }
    #[doc = "0x5e0 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar7(&self) -> &HCCHAR7 {
        &self.hcchar7
    }
    #[doc = "0x5e4 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt7(&self) -> &HCSPLT7 {
        &self.hcsplt7
    }
    #[doc = "0x5e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint7(&self) -> &HCINT7 {
        &self.hcint7
    }
    #[doc = "0x5ec - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk7(&self) -> &HCINTMSK7 {
        &self.hcintmsk7
    }
    #[doc = "0x5f0 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz7(&self) -> &HCTSIZ7 {
        &self.hctsiz7
    }
    #[doc = "0x5f4 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma7(&self) -> &HCDMA7 {
        &self.hcdma7
    }
    #[doc = "0x5fc - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab7(&self) -> &HCDMAB7 {
        &self.hcdmab7
    }
    #[doc = "0x600 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar8(&self) -> &HCCHAR8 {
        &self.hcchar8
    }
    #[doc = "0x604 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt8(&self) -> &HCSPLT8 {
        &self.hcsplt8
    }
    #[doc = "0x608 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint8(&self) -> &HCINT8 {
        &self.hcint8
    }
    #[doc = "0x60c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk8(&self) -> &HCINTMSK8 {
        &self.hcintmsk8
    }
    #[doc = "0x610 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz8(&self) -> &HCTSIZ8 {
        &self.hctsiz8
    }
    #[doc = "0x614 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma8(&self) -> &HCDMA8 {
        &self.hcdma8
    }
    #[doc = "0x61c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab8(&self) -> &HCDMAB8 {
        &self.hcdmab8
    }
    #[doc = "0x620 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar9(&self) -> &HCCHAR9 {
        &self.hcchar9
    }
    #[doc = "0x624 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt9(&self) -> &HCSPLT9 {
        &self.hcsplt9
    }
    #[doc = "0x628 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint9(&self) -> &HCINT9 {
        &self.hcint9
    }
    #[doc = "0x62c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk9(&self) -> &HCINTMSK9 {
        &self.hcintmsk9
    }
    #[doc = "0x630 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz9(&self) -> &HCTSIZ9 {
        &self.hctsiz9
    }
    #[doc = "0x634 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma9(&self) -> &HCDMA9 {
        &self.hcdma9
    }
    #[doc = "0x63c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab9(&self) -> &HCDMAB9 {
        &self.hcdmab9
    }
    #[doc = "0x640 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar10(&self) -> &HCCHAR10 {
        &self.hcchar10
    }
    #[doc = "0x644 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt10(&self) -> &HCSPLT10 {
        &self.hcsplt10
    }
    #[doc = "0x648 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint10(&self) -> &HCINT10 {
        &self.hcint10
    }
    #[doc = "0x64c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk10(&self) -> &HCINTMSK10 {
        &self.hcintmsk10
    }
    #[doc = "0x650 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz10(&self) -> &HCTSIZ10 {
        &self.hctsiz10
    }
    #[doc = "0x654 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma10(&self) -> &HCDMA10 {
        &self.hcdma10
    }
    #[doc = "0x65c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab10(&self) -> &HCDMAB10 {
        &self.hcdmab10
    }
    #[doc = "0x660 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar11(&self) -> &HCCHAR11 {
        &self.hcchar11
    }
    #[doc = "0x664 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt11(&self) -> &HCSPLT11 {
        &self.hcsplt11
    }
    #[doc = "0x668 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint11(&self) -> &HCINT11 {
        &self.hcint11
    }
    #[doc = "0x66c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk11(&self) -> &HCINTMSK11 {
        &self.hcintmsk11
    }
    #[doc = "0x670 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz11(&self) -> &HCTSIZ11 {
        &self.hctsiz11
    }
    #[doc = "0x674 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma11(&self) -> &HCDMA11 {
        &self.hcdma11
    }
    #[doc = "0x67c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab11(&self) -> &HCDMAB11 {
        &self.hcdmab11
    }
    #[doc = "0x680 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar12(&self) -> &HCCHAR12 {
        &self.hcchar12
    }
    #[doc = "0x684 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt12(&self) -> &HCSPLT12 {
        &self.hcsplt12
    }
    #[doc = "0x688 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint12(&self) -> &HCINT12 {
        &self.hcint12
    }
    #[doc = "0x68c - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk12(&self) -> &HCINTMSK12 {
        &self.hcintmsk12
    }
    #[doc = "0x690 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz12(&self) -> &HCTSIZ12 {
        &self.hctsiz12
    }
    #[doc = "0x694 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma12(&self) -> &HCDMA12 {
        &self.hcdma12
    }
    #[doc = "0x69c - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab12(&self) -> &HCDMAB12 {
        &self.hcdmab12
    }
    #[doc = "0x6a0 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar13(&self) -> &HCCHAR13 {
        &self.hcchar13
    }
    #[doc = "0x6a4 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt13(&self) -> &HCSPLT13 {
        &self.hcsplt13
    }
    #[doc = "0x6a8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint13(&self) -> &HCINT13 {
        &self.hcint13
    }
    #[doc = "0x6ac - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk13(&self) -> &HCINTMSK13 {
        &self.hcintmsk13
    }
    #[doc = "0x6b0 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz13(&self) -> &HCTSIZ13 {
        &self.hctsiz13
    }
    #[doc = "0x6b4 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma13(&self) -> &HCDMA13 {
        &self.hcdma13
    }
    #[doc = "0x6bc - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab13(&self) -> &HCDMAB13 {
        &self.hcdmab13
    }
    #[doc = "0x6c0 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar14(&self) -> &HCCHAR14 {
        &self.hcchar14
    }
    #[doc = "0x6c4 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt14(&self) -> &HCSPLT14 {
        &self.hcsplt14
    }
    #[doc = "0x6c8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint14(&self) -> &HCINT14 {
        &self.hcint14
    }
    #[doc = "0x6cc - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk14(&self) -> &HCINTMSK14 {
        &self.hcintmsk14
    }
    #[doc = "0x6d0 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz14(&self) -> &HCTSIZ14 {
        &self.hctsiz14
    }
    #[doc = "0x6d4 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma14(&self) -> &HCDMA14 {
        &self.hcdma14
    }
    #[doc = "0x6dc - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab14(&self) -> &HCDMAB14 {
        &self.hcdmab14
    }
    #[doc = "0x6e0 - This register contains the characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcchar15(&self) -> &HCCHAR15 {
        &self.hcchar15
    }
    #[doc = "0x6e4 - This register contains the Split characteristics of the Host Channel."]
    #[inline(always)]
    pub const fn hcsplt15(&self) -> &HCSPLT15 {
        &self.hcsplt15
    }
    #[doc = "0x6e8 - This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
    #[inline(always)]
    pub const fn hcint15(&self) -> &HCINT15 {
        &self.hcint15
    }
    #[doc = "0x6ec - This register reflects the mask for each channel status described in the previous section."]
    #[inline(always)]
    pub const fn hcintmsk15(&self) -> &HCINTMSK15 {
        &self.hcintmsk15
    }
    #[doc = "0x6f0 - This register reflects the transfer size for the Host Channel."]
    #[inline(always)]
    pub const fn hctsiz15(&self) -> &HCTSIZ15 {
        &self.hctsiz15
    }
    #[doc = "0x6f4 - This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
    #[inline(always)]
    pub const fn hcdma15(&self) -> &HCDMA15 {
        &self.hcdma15
    }
    #[doc = "0x6fc - This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
    #[inline(always)]
    pub const fn hcdmab15(&self) -> &HCDMAB15 {
        &self.hcdmab15
    }
    #[doc = "0x800 - This register configures the core in Device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
    #[inline(always)]
    pub const fn dcfg(&self) -> &DCFG {
        &self.dcfg
    }
    #[doc = "0x804 - This register is used to control the characteristics of the Device controller."]
    #[inline(always)]
    pub const fn dctl(&self) -> &DCTL {
        &self.dctl
    }
    #[doc = "0x808 - This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from Device All Interrupts (DAINT) register."]
    #[inline(always)]
    pub const fn dsts(&self) -> &DSTS {
        &self.dsts
    }
    #[doc = "0x810 - This register works with each of the Device IN Endpoint Interrupt (DIEPINTn) registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTn register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn diepmsk(&self) -> &DIEPMSK {
        &self.diepmsk
    }
    #[doc = "0x814 - This register works with each of the Device OUT Endpoint Interrupt (DOEPINTn) registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTn register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn doepmsk(&self) -> &DOEPMSK {
        &self.doepmsk
    }
    #[doc = "0x818 - When a significant event occurs on an endpoint, a Device All Endpoints Interrupt register interrupts the application using the Device OUT Endpoints Interrupt bit or Device IN Endpoints Interrupt bit of the Core Interrupt register (GINTSTS.OEPInt or GINTSTS.IEPInt, respectively). This is shown in Figure 5-2. There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-n Interrupt register (DIEPINTn/DOEPINTn)."]
    #[inline(always)]
    pub const fn daint(&self) -> &DAINT {
        &self.daint
    }
    #[doc = "0x81c - The Device Endpoint Interrupt Mask register works with the Device Endpoint Interrupt register to interrupt the application when an event occurs on a device endpoint. However, the Device All Endpoints Interrupt (DAINT) register bit corresponding to that interrupt is still set."]
    #[inline(always)]
    pub const fn daintmsk(&self) -> &DAINTMSK {
        &self.daintmsk
    }
    #[doc = "0x828 - This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
    #[inline(always)]
    pub const fn dvbusdis(&self) -> &DVBUSDIS {
        &self.dvbusdis
    }
    #[doc = "0x82c - This register contains the VBUS pulsing time during SRP."]
    #[inline(always)]
    pub const fn dvbuspulse(&self) -> &DVBUSPULSE {
        &self.dvbuspulse
    }
    #[doc = "0x830 - This register contains the Receive and Transmit Threshold characteristics of the Device controller."]
    #[inline(always)]
    pub const fn dthrctl(&self) -> &DTHRCTL {
        &self.dthrctl
    }
    #[doc = "0x834 - This register is valid only in Dedicated FIFO operation (OTG_EN_DED_TX_FIFO = 1). This register is used to control the IN endpoint FIFO empty interrupt generation (DIEPINTn.TxfEmp)."]
    #[inline(always)]
    pub const fn diepempmsk(&self) -> &DIEPEMPMSK {
        &self.diepempmsk
    }
    #[doc = "0x838 - This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT on page 121=1. There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-n Interrupt register (DIEPINTn/DOEPINTn). The interrupt is automatically cleared once the DOEPINTn/DIEPINTn interrupt is cleared by the application."]
    #[inline(always)]
    pub const fn deachint(&self) -> &DEACHINT {
        &self.deachint
    }
    #[doc = "0x83c - This register is available only when parameter OTG_MULTI_PROC_INTRPT=1. The Device Each Endpoint Interrupt Mask register works with the Device Each Endpoint Interrupt register to interrupt the application when an event occurs on a device endpoint. However, the Device Each Endpoints Interrupt (DEACHINT) register bit corresponding to that interrupt remains set."]
    #[inline(always)]
    pub const fn deachintmsk(&self) -> &DEACHINTMSK {
        &self.deachintmsk
    }
    #[doc = "0x840 - This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT on page 121=1. These registers are endpoint-specific mask registers for (DIEPINTn). The IN endpoint interrupt for a specific status in the DIEPINTn register can be masked by writing 0 to the corresponding bit in this register. Status bits are masked by default. - Mask interrupt: 1'b0 - Unmask interrupt: 1'b1"]
    #[inline(always)]
    pub const fn diepeachmsk0(&self) -> &DIEPEACHMSK0 {
        &self.diepeachmsk0
    }
    #[doc = "0x844 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk1(&self) -> &DIEPEACHMSK1 {
        &self.diepeachmsk1
    }
    #[doc = "0x848 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk2(&self) -> &DIEPEACHMSK2 {
        &self.diepeachmsk2
    }
    #[doc = "0x84c - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk3(&self) -> &DIEPEACHMSK3 {
        &self.diepeachmsk3
    }
    #[doc = "0x850 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk4(&self) -> &DIEPEACHMSK4 {
        &self.diepeachmsk4
    }
    #[doc = "0x854 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk5(&self) -> &DIEPEACHMSK5 {
        &self.diepeachmsk5
    }
    #[doc = "0x858 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk6(&self) -> &DIEPEACHMSK6 {
        &self.diepeachmsk6
    }
    #[doc = "0x85c - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk7(&self) -> &DIEPEACHMSK7 {
        &self.diepeachmsk7
    }
    #[doc = "0x860 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk8(&self) -> &DIEPEACHMSK8 {
        &self.diepeachmsk8
    }
    #[doc = "0x864 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk9(&self) -> &DIEPEACHMSK9 {
        &self.diepeachmsk9
    }
    #[doc = "0x868 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk10(&self) -> &DIEPEACHMSK10 {
        &self.diepeachmsk10
    }
    #[doc = "0x86c - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk11(&self) -> &DIEPEACHMSK11 {
        &self.diepeachmsk11
    }
    #[doc = "0x870 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk12(&self) -> &DIEPEACHMSK12 {
        &self.diepeachmsk12
    }
    #[doc = "0x874 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk13(&self) -> &DIEPEACHMSK13 {
        &self.diepeachmsk13
    }
    #[doc = "0x878 - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk14(&self) -> &DIEPEACHMSK14 {
        &self.diepeachmsk14
    }
    #[doc = "0x87c - This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepeachmsk15(&self) -> &DIEPEACHMSK15 {
        &self.diepeachmsk15
    }
    #[doc = "0x880 - This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT=1. These registers are endpoint specific mask registers for (DOEPINTn). The OUT endpoint interrupt for a specific status in the DOEPINTn register can be masked by writing 0 to the corresponding bit in this register. Status bits are masked by default."]
    #[inline(always)]
    pub const fn doepeachmsk0(&self) -> &DOEPEACHMSK0 {
        &self.doepeachmsk0
    }
    #[doc = "0x884 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk1(&self) -> &DOEPEACHMSK1 {
        &self.doepeachmsk1
    }
    #[doc = "0x888 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk2(&self) -> &DOEPEACHMSK2 {
        &self.doepeachmsk2
    }
    #[doc = "0x88c - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk3(&self) -> &DOEPEACHMSK3 {
        &self.doepeachmsk3
    }
    #[doc = "0x890 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk4(&self) -> &DOEPEACHMSK4 {
        &self.doepeachmsk4
    }
    #[doc = "0x894 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk5(&self) -> &DOEPEACHMSK5 {
        &self.doepeachmsk5
    }
    #[doc = "0x898 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk6(&self) -> &DOEPEACHMSK6 {
        &self.doepeachmsk6
    }
    #[doc = "0x89c - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk7(&self) -> &DOEPEACHMSK7 {
        &self.doepeachmsk7
    }
    #[doc = "0x8a0 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk8(&self) -> &DOEPEACHMSK8 {
        &self.doepeachmsk8
    }
    #[doc = "0x8a4 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk9(&self) -> &DOEPEACHMSK9 {
        &self.doepeachmsk9
    }
    #[doc = "0x8a8 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk10(&self) -> &DOEPEACHMSK10 {
        &self.doepeachmsk10
    }
    #[doc = "0x8ac - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk11(&self) -> &DOEPEACHMSK11 {
        &self.doepeachmsk11
    }
    #[doc = "0x8b0 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk12(&self) -> &DOEPEACHMSK12 {
        &self.doepeachmsk12
    }
    #[doc = "0x8b4 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk13(&self) -> &DOEPEACHMSK13 {
        &self.doepeachmsk13
    }
    #[doc = "0x8b8 - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk14(&self) -> &DOEPEACHMSK14 {
        &self.doepeachmsk14
    }
    #[doc = "0x8bc - This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn doepeachmsk15(&self) -> &DOEPEACHMSK15 {
        &self.doepeachmsk15
    }
    #[doc = "0x900 - This register is used to control the characteristics of the IN Endpoint 0 of the Device controller."]
    #[inline(always)]
    pub const fn diepctl0(&self) -> &DIEPCTL0 {
        &self.diepctl0
    }
    #[doc = "0x908 - This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the OUT Endpoints Interrupt bit or IN Endpoints Interrupt bit of the Core Interrupt register (GINTSTS.OEPInt or GINTSTS.IEPInt, respectively) is set. Before the application can read this register, it must first read the Device All Endpoints Interrupt (DAINT) register to get the exact endpoint number for the Device Endpoint-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers"]
    #[inline(always)]
    pub const fn diepint0(&self) -> &DIEPINT0 {
        &self.diepint0
    }
    #[doc = "0x910 - The application must modify this register before enabling endpoint 0. Once endpoint 0 is enabled using Endpoint Enable bit of the Device Control Endpoint 0 Control registers (DIEPCTL0.EPEna/DOEPCTL0.EPEna), the core modifies this register. The application can only read this register once the core has cleared the Endpoint Enable bit. Nonzero endpoints use the registers for endpoints 115. When Scatter/Gather DMA mode is enabled, this register must not be programmed by the application. If the application reads this register when Scatter/Gather DMA mode is enabled, the core returns all zeros."]
    #[inline(always)]
    pub const fn dieptsiz0(&self) -> &DIEPTSIZ0 {
        &self.dieptsiz0
    }
    #[doc = "0x914 - This register contains the DMA Address for the IN Endpoint 0 of the Device controller."]
    #[inline(always)]
    pub const fn diepdma0(&self) -> &DIEPDMA0 {
        &self.diepdma0
    }
    #[doc = "0x918 - This register contains information about the IN Endpoint Transmit FIFO of the Device controller."]
    #[inline(always)]
    pub const fn dtxfsts0(&self) -> &DTXFSTS0 {
        &self.dtxfsts0
    }
    #[doc = "0x91c - This register contains the DMA Buffer Address for the IN Endpoint 0 of the Device controller."]
    #[inline(always)]
    pub const fn diepdmab0(&self) -> &DIEPDMAB0 {
        &self.diepdmab0
    }
    #[doc = "0x920 - This register is used to control the characteristics of Endpoint 1. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl1(&self) -> &DIEPCTL1 {
        &self.diepctl1
    }
    #[doc = "0x928 - This register contains the interrupts for the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint1(&self) -> &DIEPINT1 {
        &self.diepint1
    }
    #[doc = "0x930 - This register reflects the Transfer Size of the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz1(&self) -> &DIEPTSIZ1 {
        &self.dieptsiz1
    }
    #[doc = "0x934 - This register contains the DMA Address for the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma1(&self) -> &DIEPDMA1 {
        &self.diepdma1
    }
    #[doc = "0x938 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts1(&self) -> &DTXFSTS1 {
        &self.dtxfsts1
    }
    #[doc = "0x93c - This register contains the DMA Buffer Address of the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab1(&self) -> &DIEPDMAB1 {
        &self.diepdmab1
    }
    #[doc = "0x940 - This register is used to control the characteristics of Endpoint 2. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl2(&self) -> &DIEPCTL2 {
        &self.diepctl2
    }
    #[doc = "0x948 - This register contains the interrupts for the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint2(&self) -> &DIEPINT2 {
        &self.diepint2
    }
    #[doc = "0x950 - This register reflects the Transfer Size of the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz2(&self) -> &DIEPTSIZ2 {
        &self.dieptsiz2
    }
    #[doc = "0x954 - This register contains the DMA Address for the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma2(&self) -> &DIEPDMA2 {
        &self.diepdma2
    }
    #[doc = "0x958 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts2(&self) -> &DTXFSTS2 {
        &self.dtxfsts2
    }
    #[doc = "0x95c - This register contains the DMA Buffer Address of the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab2(&self) -> &DIEPDMAB2 {
        &self.diepdmab2
    }
    #[doc = "0x960 - This register is used to control the characteristics of Endpoint 3. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl3(&self) -> &DIEPCTL3 {
        &self.diepctl3
    }
    #[doc = "0x968 - This register contains the interrupts for the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint3(&self) -> &DIEPINT3 {
        &self.diepint3
    }
    #[doc = "0x970 - This register reflects the Transfer Size of the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz3(&self) -> &DIEPTSIZ3 {
        &self.dieptsiz3
    }
    #[doc = "0x974 - This register contains the DMA Address for the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma3(&self) -> &DIEPDMA3 {
        &self.diepdma3
    }
    #[doc = "0x978 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts3(&self) -> &DTXFSTS3 {
        &self.dtxfsts3
    }
    #[doc = "0x97c - This register contains the DMA Buffer Address of the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab3(&self) -> &DIEPDMAB3 {
        &self.diepdmab3
    }
    #[doc = "0x980 - This register is used to control the characteristics of Endpoint 4. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl4(&self) -> &DIEPCTL4 {
        &self.diepctl4
    }
    #[doc = "0x988 - This register contains the interrupts for the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint4(&self) -> &DIEPINT4 {
        &self.diepint4
    }
    #[doc = "0x990 - This register reflects the Transfer Size of the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz4(&self) -> &DIEPTSIZ4 {
        &self.dieptsiz4
    }
    #[doc = "0x994 - This register contains the DMA Address for the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma4(&self) -> &DIEPDMA4 {
        &self.diepdma4
    }
    #[doc = "0x998 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts4(&self) -> &DTXFSTS4 {
        &self.dtxfsts4
    }
    #[doc = "0x99c - This register contains the DMA Buffer Address of the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab4(&self) -> &DIEPDMAB4 {
        &self.diepdmab4
    }
    #[doc = "0x9a0 - This register is used to control the characteristics of Endpoint 5. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl5(&self) -> &DIEPCTL5 {
        &self.diepctl5
    }
    #[doc = "0x9a8 - This register contains the interrupts for the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint5(&self) -> &DIEPINT5 {
        &self.diepint5
    }
    #[doc = "0x9b0 - This register reflects the Transfer Size of the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz5(&self) -> &DIEPTSIZ5 {
        &self.dieptsiz5
    }
    #[doc = "0x9b4 - This register contains the DMA Address for the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma5(&self) -> &DIEPDMA5 {
        &self.diepdma5
    }
    #[doc = "0x9b8 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts5(&self) -> &DTXFSTS5 {
        &self.dtxfsts5
    }
    #[doc = "0x9bc - This register contains the DMA Buffer Address of the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab5(&self) -> &DIEPDMAB5 {
        &self.diepdmab5
    }
    #[doc = "0x9c0 - This register is used to control the characteristics of Endpoint 6. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl6(&self) -> &DIEPCTL6 {
        &self.diepctl6
    }
    #[doc = "0x9c8 - This register contains the interrupts for the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint6(&self) -> &DIEPINT6 {
        &self.diepint6
    }
    #[doc = "0x9d0 - This register reflects the Transfer Size of the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz6(&self) -> &DIEPTSIZ6 {
        &self.dieptsiz6
    }
    #[doc = "0x9d4 - This register contains the DMA Address for the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma6(&self) -> &DIEPDMA6 {
        &self.diepdma6
    }
    #[doc = "0x9d8 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts6(&self) -> &DTXFSTS6 {
        &self.dtxfsts6
    }
    #[doc = "0x9dc - This register contains the DMA Buffer Address of the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab6(&self) -> &DIEPDMAB6 {
        &self.diepdmab6
    }
    #[doc = "0x9e0 - This register is used to control the characteristics of Endpoint 7. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl7(&self) -> &DIEPCTL7 {
        &self.diepctl7
    }
    #[doc = "0x9e8 - This register contains the interrupts for the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint7(&self) -> &DIEPINT7 {
        &self.diepint7
    }
    #[doc = "0x9f0 - This register reflects the Transfer Size of the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz7(&self) -> &DIEPTSIZ7 {
        &self.dieptsiz7
    }
    #[doc = "0x9f4 - This register contains the DMA Address for the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma7(&self) -> &DIEPDMA7 {
        &self.diepdma7
    }
    #[doc = "0x9f8 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts7(&self) -> &DTXFSTS7 {
        &self.dtxfsts7
    }
    #[doc = "0x9fc - This register contains the DMA Buffer Address of the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab7(&self) -> &DIEPDMAB7 {
        &self.diepdmab7
    }
    #[doc = "0xa00 - This register is used to control the characteristics of Endpoint 8. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl8(&self) -> &DIEPCTL8 {
        &self.diepctl8
    }
    #[doc = "0xa08 - This register contains the interrupts for the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint8(&self) -> &DIEPINT8 {
        &self.diepint8
    }
    #[doc = "0xa10 - This register reflects the Transfer Size of the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz8(&self) -> &DIEPTSIZ8 {
        &self.dieptsiz8
    }
    #[doc = "0xa14 - This register contains the DMA Address for the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma8(&self) -> &DIEPDMA8 {
        &self.diepdma8
    }
    #[doc = "0xa18 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts8(&self) -> &DTXFSTS8 {
        &self.dtxfsts8
    }
    #[doc = "0xa1c - This register contains the DMA Buffer Address of the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab8(&self) -> &DIEPDMAB8 {
        &self.diepdmab8
    }
    #[doc = "0xa20 - This register is used to control the characteristics of Endpoint 9. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl9(&self) -> &DIEPCTL9 {
        &self.diepctl9
    }
    #[doc = "0xa28 - This register contains the interrupts for the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint9(&self) -> &DIEPINT9 {
        &self.diepint9
    }
    #[doc = "0xa30 - This register reflects the Transfer Size of the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz9(&self) -> &DIEPTSIZ9 {
        &self.dieptsiz9
    }
    #[doc = "0xa34 - This register contains the DMA Address for the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma9(&self) -> &DIEPDMA9 {
        &self.diepdma9
    }
    #[doc = "0xa38 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts9(&self) -> &DTXFSTS9 {
        &self.dtxfsts9
    }
    #[doc = "0xa3c - This register contains the DMA Buffer Address of the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab9(&self) -> &DIEPDMAB9 {
        &self.diepdmab9
    }
    #[doc = "0xa40 - This register is used to control the characteristics of Endpoint 10. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl10(&self) -> &DIEPCTL10 {
        &self.diepctl10
    }
    #[doc = "0xa48 - This register contains the interrupts for the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint10(&self) -> &DIEPINT10 {
        &self.diepint10
    }
    #[doc = "0xa50 - This register reflects the Transfer Size of the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz10(&self) -> &DIEPTSIZ10 {
        &self.dieptsiz10
    }
    #[doc = "0xa54 - This register contains the DMA Address for the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma10(&self) -> &DIEPDMA10 {
        &self.diepdma10
    }
    #[doc = "0xa58 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts10(&self) -> &DTXFSTS10 {
        &self.dtxfsts10
    }
    #[doc = "0xa5c - This register contains the DMA Buffer Address of the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab10(&self) -> &DIEPDMAB10 {
        &self.diepdmab10
    }
    #[doc = "0xa60 - This register is used to control the characteristics of Endpoint 11. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl11(&self) -> &DIEPCTL11 {
        &self.diepctl11
    }
    #[doc = "0xa68 - This register contains the interrupts for the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint11(&self) -> &DIEPINT11 {
        &self.diepint11
    }
    #[doc = "0xa70 - This register reflects the Transfer Size of the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz11(&self) -> &DIEPTSIZ11 {
        &self.dieptsiz11
    }
    #[doc = "0xa74 - This register contains the DMA Address for the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma11(&self) -> &DIEPDMA11 {
        &self.diepdma11
    }
    #[doc = "0xa78 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts11(&self) -> &DTXFSTS11 {
        &self.dtxfsts11
    }
    #[doc = "0xa7c - This register contains the DMA Buffer Address of the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab11(&self) -> &DIEPDMAB11 {
        &self.diepdmab11
    }
    #[doc = "0xa80 - This register is used to control the characteristics of Endpoint 12. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl12(&self) -> &DIEPCTL12 {
        &self.diepctl12
    }
    #[doc = "0xa88 - This register contains the interrupts for the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint12(&self) -> &DIEPINT12 {
        &self.diepint12
    }
    #[doc = "0xa90 - This register reflects the Transfer Size of the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz12(&self) -> &DIEPTSIZ12 {
        &self.dieptsiz12
    }
    #[doc = "0xa94 - This register contains the DMA Address for the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma12(&self) -> &DIEPDMA12 {
        &self.diepdma12
    }
    #[doc = "0xa98 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts12(&self) -> &DTXFSTS12 {
        &self.dtxfsts12
    }
    #[doc = "0xa9c - This register contains the DMA Buffer Address of the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab12(&self) -> &DIEPDMAB12 {
        &self.diepdmab12
    }
    #[doc = "0xaa0 - This register is used to control the characteristics of Endpoint 13. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl13(&self) -> &DIEPCTL13 {
        &self.diepctl13
    }
    #[doc = "0xaa8 - This register contains the interrupts for the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint13(&self) -> &DIEPINT13 {
        &self.diepint13
    }
    #[doc = "0xab0 - This register reflects the Transfer Size of the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz13(&self) -> &DIEPTSIZ13 {
        &self.dieptsiz13
    }
    #[doc = "0xab4 - This register contains the DMA Address for the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma13(&self) -> &DIEPDMA13 {
        &self.diepdma13
    }
    #[doc = "0xab8 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts13(&self) -> &DTXFSTS13 {
        &self.dtxfsts13
    }
    #[doc = "0xabc - This register contains the DMA Buffer Address of the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab13(&self) -> &DIEPDMAB13 {
        &self.diepdmab13
    }
    #[doc = "0xac0 - This register is used to control the characteristics of Endpoint 14. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl14(&self) -> &DIEPCTL14 {
        &self.diepctl14
    }
    #[doc = "0xac8 - This register contains the interrupts for the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint14(&self) -> &DIEPINT14 {
        &self.diepint14
    }
    #[doc = "0xad0 - This register reflects the Transfer Size of the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz14(&self) -> &DIEPTSIZ14 {
        &self.dieptsiz14
    }
    #[doc = "0xad4 - This register contains the DMA Address for the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma14(&self) -> &DIEPDMA14 {
        &self.diepdma14
    }
    #[doc = "0xad8 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts14(&self) -> &DTXFSTS14 {
        &self.dtxfsts14
    }
    #[doc = "0xadc - This register contains the DMA Buffer Address of the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab14(&self) -> &DIEPDMAB14 {
        &self.diepdmab14
    }
    #[doc = "0xae0 - This register is used to control the characteristics of Endpoint 15. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepctl15(&self) -> &DIEPCTL15 {
        &self.diepctl15
    }
    #[doc = "0xae8 - This register contains the interrupts for the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepint15(&self) -> &DIEPINT15 {
        &self.diepint15
    }
    #[doc = "0xaf0 - This register reflects the Transfer Size of the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dieptsiz15(&self) -> &DIEPTSIZ15 {
        &self.dieptsiz15
    }
    #[doc = "0xaf4 - This register contains the DMA Address for the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdma15(&self) -> &DIEPDMA15 {
        &self.diepdma15
    }
    #[doc = "0xaf8 - This register reflects the status of the IN Endpoint Transmit FIFO Status Register 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn dtxfsts15(&self) -> &DTXFSTS15 {
        &self.dtxfsts15
    }
    #[doc = "0xafc - This register contains the DMA Buffer Address of the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
    #[inline(always)]
    pub const fn diepdmab15(&self) -> &DIEPDMAB15 {
        &self.diepdmab15
    }
    #[doc = "0xb00 - This register is used to control the characteristics of the OUT Endpoint 0 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl0(&self) -> &DOEPCTL0 {
        &self.doepctl0
    }
    #[doc = "0xb08 - This register contains the interrupts for the OUT Endpoint 0 of the Device controller."]
    #[inline(always)]
    pub const fn doepint0(&self) -> &DOEPINT0 {
        &self.doepint0
    }
    #[doc = "0xb10 - This register contains the Transfer Size for the OUT Endpoint 0 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz0(&self) -> &DOEPTSIZ0 {
        &self.doeptsiz0
    }
    #[doc = "0xb14 - This register contains the DMA Address for the OUT Endpoint 0 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma0(&self) -> &DOEPDMA0 {
        &self.doepdma0
    }
    #[doc = "0xb1c - This register contains the DMA Buffer Address for the OUT Endpoint 0 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab0(&self) -> &DOEPDMAB0 {
        &self.doepdmab0
    }
    #[doc = "0xb20 - This register is used to control the characteristics of OUT Endpoint 1 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl1(&self) -> &DOEPCTL1 {
        &self.doepctl1
    }
    #[doc = "0xb28 - This register contains the interrupts for the OUT Endpoint 1 of the Device controller."]
    #[inline(always)]
    pub const fn doepint1(&self) -> &DOEPINT1 {
        &self.doepint1
    }
    #[doc = "0xb30 - This register contains the Transfer Size for the OUT Endpoint 1 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz1(&self) -> &DOEPTSIZ1 {
        &self.doeptsiz1
    }
    #[doc = "0xb34 - This register contains the DMA Address for the OUT Endpoint 1 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma1(&self) -> &DOEPDMA1 {
        &self.doepdma1
    }
    #[doc = "0xb3c - This register contains the DMA Buffer Address for the OUT Endpoint 1 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab1(&self) -> &DOEPDMAB1 {
        &self.doepdmab1
    }
    #[doc = "0xb40 - This register is used to control the characteristics of OUT Endpoint 2 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl2(&self) -> &DOEPCTL2 {
        &self.doepctl2
    }
    #[doc = "0xb48 - This register contains the interrupts for the OUT Endpoint 2 of the Device controller."]
    #[inline(always)]
    pub const fn doepint2(&self) -> &DOEPINT2 {
        &self.doepint2
    }
    #[doc = "0xb50 - This register contains the Transfer Size for the OUT Endpoint 2 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz2(&self) -> &DOEPTSIZ2 {
        &self.doeptsiz2
    }
    #[doc = "0xb54 - This register contains the DMA Address for the OUT Endpoint 2 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma2(&self) -> &DOEPDMA2 {
        &self.doepdma2
    }
    #[doc = "0xb5c - This register contains the DMA Buffer Address for the OUT Endpoint 2 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab2(&self) -> &DOEPDMAB2 {
        &self.doepdmab2
    }
    #[doc = "0xb60 - This register is used to control the characteristics of OUT Endpoint 3 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl3(&self) -> &DOEPCTL3 {
        &self.doepctl3
    }
    #[doc = "0xb68 - This register contains the interrupts for the OUT Endpoint 3 of the Device controller."]
    #[inline(always)]
    pub const fn doepint3(&self) -> &DOEPINT3 {
        &self.doepint3
    }
    #[doc = "0xb70 - This register contains the Transfer Size for the OUT Endpoint 3 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz3(&self) -> &DOEPTSIZ3 {
        &self.doeptsiz3
    }
    #[doc = "0xb74 - This register contains the DMA Address for the OUT Endpoint 3 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma3(&self) -> &DOEPDMA3 {
        &self.doepdma3
    }
    #[doc = "0xb7c - This register contains the DMA Buffer Address for the OUT Endpoint 3 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab3(&self) -> &DOEPDMAB3 {
        &self.doepdmab3
    }
    #[doc = "0xb80 - This register is used to control the characteristics of OUT Endpoint 4 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl4(&self) -> &DOEPCTL4 {
        &self.doepctl4
    }
    #[doc = "0xb88 - This register contains the interrupts for the OUT Endpoint 4 of the Device controller."]
    #[inline(always)]
    pub const fn doepint4(&self) -> &DOEPINT4 {
        &self.doepint4
    }
    #[doc = "0xb90 - This register contains the Transfer Size for the OUT Endpoint 4 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz4(&self) -> &DOEPTSIZ4 {
        &self.doeptsiz4
    }
    #[doc = "0xb94 - This register contains the DMA Address for the OUT Endpoint 4 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma4(&self) -> &DOEPDMA4 {
        &self.doepdma4
    }
    #[doc = "0xb9c - This register contains the DMA Buffer Address for the OUT Endpoint 4 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab4(&self) -> &DOEPDMAB4 {
        &self.doepdmab4
    }
    #[doc = "0xba0 - This register is used to control the characteristics of OUT Endpoint 5 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl5(&self) -> &DOEPCTL5 {
        &self.doepctl5
    }
    #[doc = "0xba8 - This register contains the interrupts for the OUT Endpoint 5 of the Device controller."]
    #[inline(always)]
    pub const fn doepint5(&self) -> &DOEPINT5 {
        &self.doepint5
    }
    #[doc = "0xbb0 - This register contains the Transfer Size for the OUT Endpoint 5 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz5(&self) -> &DOEPTSIZ5 {
        &self.doeptsiz5
    }
    #[doc = "0xbb4 - This register contains the DMA Address for the OUT Endpoint 5 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma5(&self) -> &DOEPDMA5 {
        &self.doepdma5
    }
    #[doc = "0xbbc - This register contains the DMA Buffer Address for the OUT Endpoint 5 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab5(&self) -> &DOEPDMAB5 {
        &self.doepdmab5
    }
    #[doc = "0xbc0 - This register is used to control the characteristics of OUT Endpoint 6 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl6(&self) -> &DOEPCTL6 {
        &self.doepctl6
    }
    #[doc = "0xbc8 - This register contains the interrupts for the OUT Endpoint 6 of the Device controller."]
    #[inline(always)]
    pub const fn doepint6(&self) -> &DOEPINT6 {
        &self.doepint6
    }
    #[doc = "0xbd0 - This register contains the Transfer Size for the OUT Endpoint 6 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz6(&self) -> &DOEPTSIZ6 {
        &self.doeptsiz6
    }
    #[doc = "0xbd4 - This register contains the DMA Address for the OUT Endpoint 6 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma6(&self) -> &DOEPDMA6 {
        &self.doepdma6
    }
    #[doc = "0xbdc - This register contains the DMA Buffer Address for the OUT Endpoint 6 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab6(&self) -> &DOEPDMAB6 {
        &self.doepdmab6
    }
    #[doc = "0xbe0 - This register is used to control the characteristics of OUT Endpoint 7 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl7(&self) -> &DOEPCTL7 {
        &self.doepctl7
    }
    #[doc = "0xbe8 - This register contains the interrupts for the OUT Endpoint 7 of the Device controller."]
    #[inline(always)]
    pub const fn doepint7(&self) -> &DOEPINT7 {
        &self.doepint7
    }
    #[doc = "0xbf0 - This register contains the Transfer Size for the OUT Endpoint 7 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz7(&self) -> &DOEPTSIZ7 {
        &self.doeptsiz7
    }
    #[doc = "0xbf4 - This register contains the DMA Address for the OUT Endpoint 7 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma7(&self) -> &DOEPDMA7 {
        &self.doepdma7
    }
    #[doc = "0xbfc - This register contains the DMA Buffer Address for the OUT Endpoint 7 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab7(&self) -> &DOEPDMAB7 {
        &self.doepdmab7
    }
    #[doc = "0xc00 - This register is used to control the characteristics of OUT Endpoint 8 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl8(&self) -> &DOEPCTL8 {
        &self.doepctl8
    }
    #[doc = "0xc08 - This register contains the interrupts for the OUT Endpoint 8 of the Device controller."]
    #[inline(always)]
    pub const fn doepint8(&self) -> &DOEPINT8 {
        &self.doepint8
    }
    #[doc = "0xc10 - This register contains the Transfer Size for the OUT Endpoint 8 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz8(&self) -> &DOEPTSIZ8 {
        &self.doeptsiz8
    }
    #[doc = "0xc14 - This register contains the DMA Address for the OUT Endpoint 8 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma8(&self) -> &DOEPDMA8 {
        &self.doepdma8
    }
    #[doc = "0xc1c - This register contains the DMA Buffer Address for the OUT Endpoint 8 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab8(&self) -> &DOEPDMAB8 {
        &self.doepdmab8
    }
    #[doc = "0xc20 - This register is used to control the characteristics of OUT Endpoint 9 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl9(&self) -> &DOEPCTL9 {
        &self.doepctl9
    }
    #[doc = "0xc28 - This register contains the interrupts for the OUT Endpoint 9 of the Device controller."]
    #[inline(always)]
    pub const fn doepint9(&self) -> &DOEPINT9 {
        &self.doepint9
    }
    #[doc = "0xc30 - This register contains the Transfer Size for the OUT Endpoint 9 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz9(&self) -> &DOEPTSIZ9 {
        &self.doeptsiz9
    }
    #[doc = "0xc34 - This register contains the DMA Address for the OUT Endpoint 9 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma9(&self) -> &DOEPDMA9 {
        &self.doepdma9
    }
    #[doc = "0xc3c - This register contains the DMA Buffer Address for the OUT Endpoint 9 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab9(&self) -> &DOEPDMAB9 {
        &self.doepdmab9
    }
    #[doc = "0xc40 - This register is used to control the characteristics of OUT Endpoint 10 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl10(&self) -> &DOEPCTL10 {
        &self.doepctl10
    }
    #[doc = "0xc48 - This register contains the interrupts for the OUT Endpoint 10 of the Device controller."]
    #[inline(always)]
    pub const fn doepint10(&self) -> &DOEPINT10 {
        &self.doepint10
    }
    #[doc = "0xc50 - This register contains the Transfer Size for the OUT Endpoint 10 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz10(&self) -> &DOEPTSIZ10 {
        &self.doeptsiz10
    }
    #[doc = "0xc54 - This register contains the DMA Address for the OUT Endpoint 10 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma10(&self) -> &DOEPDMA10 {
        &self.doepdma10
    }
    #[doc = "0xc5c - This register contains the DMA Buffer Address for the OUT Endpoint 10 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab10(&self) -> &DOEPDMAB10 {
        &self.doepdmab10
    }
    #[doc = "0xc60 - This register is used to control the characteristics of OUT Endpoint 11 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl11(&self) -> &DOEPCTL11 {
        &self.doepctl11
    }
    #[doc = "0xc68 - This register contains the interrupts for the OUT Endpoint 11 of the Device controller."]
    #[inline(always)]
    pub const fn doepint11(&self) -> &DOEPINT11 {
        &self.doepint11
    }
    #[doc = "0xc70 - This register contains the Transfer Size for the OUT Endpoint 11 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz11(&self) -> &DOEPTSIZ11 {
        &self.doeptsiz11
    }
    #[doc = "0xc74 - This register contains the DMA Address for the OUT Endpoint 11 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma11(&self) -> &DOEPDMA11 {
        &self.doepdma11
    }
    #[doc = "0xc7c - This register contains the DMA Buffer Address for the OUT Endpoint 11 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab11(&self) -> &DOEPDMAB11 {
        &self.doepdmab11
    }
    #[doc = "0xc80 - This register is used to control the characteristics of OUT Endpoint 12 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl12(&self) -> &DOEPCTL12 {
        &self.doepctl12
    }
    #[doc = "0xc88 - This register contains the interrupts for the OUT Endpoint 12 of the Device controller."]
    #[inline(always)]
    pub const fn doepint12(&self) -> &DOEPINT12 {
        &self.doepint12
    }
    #[doc = "0xc90 - This register contains the Transfer Size for the OUT Endpoint 12 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz12(&self) -> &DOEPTSIZ12 {
        &self.doeptsiz12
    }
    #[doc = "0xc94 - This register contains the DMA Address for the OUT Endpoint 12 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma12(&self) -> &DOEPDMA12 {
        &self.doepdma12
    }
    #[doc = "0xc9c - This register contains the DMA Buffer Address for the OUT Endpoint 12 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab12(&self) -> &DOEPDMAB12 {
        &self.doepdmab12
    }
    #[doc = "0xca0 - This register is used to control the characteristics of OUT Endpoint 13 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl13(&self) -> &DOEPCTL13 {
        &self.doepctl13
    }
    #[doc = "0xca8 - This register contains the interrupts for the OUT Endpoint 13 of the Device controller."]
    #[inline(always)]
    pub const fn doepint13(&self) -> &DOEPINT13 {
        &self.doepint13
    }
    #[doc = "0xcb0 - This register contains the Transfer Size for the OUT Endpoint 13 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz13(&self) -> &DOEPTSIZ13 {
        &self.doeptsiz13
    }
    #[doc = "0xcb4 - This register contains the DMA Address for the OUT Endpoint 13 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma13(&self) -> &DOEPDMA13 {
        &self.doepdma13
    }
    #[doc = "0xcbc - This register contains the DMA Buffer Address for the OUT Endpoint 13 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab13(&self) -> &DOEPDMAB13 {
        &self.doepdmab13
    }
    #[doc = "0xcc0 - This register is used to control the characteristics of OUT Endpoint 14 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl14(&self) -> &DOEPCTL14 {
        &self.doepctl14
    }
    #[doc = "0xcc8 - This register contains the interrupts for the OUT Endpoint 14 of the Device controller."]
    #[inline(always)]
    pub const fn doepint14(&self) -> &DOEPINT14 {
        &self.doepint14
    }
    #[doc = "0xcd0 - This register contains the Transfer Size for the OUT Endpoint 14 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz14(&self) -> &DOEPTSIZ14 {
        &self.doeptsiz14
    }
    #[doc = "0xcd4 - This register contains the DMA Address for the OUT Endpoint 14 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma14(&self) -> &DOEPDMA14 {
        &self.doepdma14
    }
    #[doc = "0xcdc - This register contains the DMA Buffer Address for the OUT Endpoint 14 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab14(&self) -> &DOEPDMAB14 {
        &self.doepdmab14
    }
    #[doc = "0xce0 - This register is used to control the characteristics of OUT Endpoint 15 of the Device controller."]
    #[inline(always)]
    pub const fn doepctl15(&self) -> &DOEPCTL15 {
        &self.doepctl15
    }
    #[doc = "0xce8 - This register contains the interrupts for the OUT Endpoint 15 of the Device controller."]
    #[inline(always)]
    pub const fn doepint15(&self) -> &DOEPINT15 {
        &self.doepint15
    }
    #[doc = "0xcf0 - This register contains the Transfer Size for the OUT Endpoint 15 of the Device controller."]
    #[inline(always)]
    pub const fn doeptsiz15(&self) -> &DOEPTSIZ15 {
        &self.doeptsiz15
    }
    #[doc = "0xcf4 - This register contains the DMA Address for the OUT Endpoint 15 of the Device controller."]
    #[inline(always)]
    pub const fn doepdma15(&self) -> &DOEPDMA15 {
        &self.doepdma15
    }
    #[doc = "0xcfc - This register contains the DMA Buffer Address for the OUT Endpoint 15 of the Device controller."]
    #[inline(always)]
    pub const fn doepdmab15(&self) -> &DOEPDMAB15 {
        &self.doepdmab15
    }
    #[doc = "0xe00 - This register is used to control the Power and Clock Gating characteristics of the controller."]
    #[inline(always)]
    pub const fn pcgcctl(&self) -> &PCGCCTL {
        &self.pcgcctl
    }
    #[doc = "0xf00 - This register is used to disable STAR fixes added in the controller. The application can set the register fields to operate with the functionality before the fix was done."]
    #[inline(always)]
    pub const fn gstarfxdis(&self) -> &GSTARFXDIS {
        &self.gstarfxdis
    }
}
#[doc = "GOTGCTL (rw) register accessor: The OTG Control and Status register controls the behavior and reflects the status of the OTG function of the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgctl`] module"]
pub type GOTGCTL = crate::Reg<gotgctl::GOTGCTL_SPEC>;
#[doc = "The OTG Control and Status register controls the behavior and reflects the status of the OTG function of the controller."]
pub mod gotgctl;
#[doc = "GOTGINT (rw) register accessor: The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`gotgint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gotgint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gotgint`] module"]
pub type GOTGINT = crate::Reg<gotgint::GOTGINT_SPEC>;
#[doc = "The application reads this register whenever there is an OTG interrupt and clears the bits in this register to clear the OTG interrupt."]
pub mod gotgint;
#[doc = "GAHBCFG (rw) register accessor: This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB.\n\nYou can [`read`](crate::Reg::read) this register and get [`gahbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gahbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gahbcfg`] module"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "This register can be used to configure the core after power-on or a change in mode. This register mainly contains AHB system-related configuration parameters. Do not change this register after the initial programming. The application must program this register before starting any transactions on either the AHB or the USB."]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: This register can be used to configure the core after power-on or when changing to Host mode or Device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. If you are using the HSIC interface, HSIC PHY must be in reset while programming this register. Do not make changes to this register after the initial programming.\n\nYou can [`read`](crate::Reg::read) this register and get [`gusbcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gusbcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gusbcfg`] module"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "This register can be used to configure the core after power-on or when changing to Host mode or Device mode. It contains USB and USB-PHY related configuration parameters. The application must program this register before starting any transactions on either the AHB or the USB. If you are using the HSIC interface, HSIC PHY must be in reset while programming this register. Do not make changes to this register after the initial programming."]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: The application uses this register to reset various hardware features inside the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`grstctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grstctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grstctl`] module"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "The application uses this register to reset various hardware features inside the controller."]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: This register interrupts the application for system-level events in the current mode (Device mode or Host mode). Some of the bits in this register are valid only in Host mode, while others are valid in Device mode only. This register also indicates the current mode. To clear the interrupt status bits of type R_SS_WC, the application must write 1'b1 to the bit. The FIFO status interrupts are read only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization. Note: Read the reset value of GINTSTS.CurMod only after the following conditions: - If IDDIG_FILTER is disabled, read only after PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires.\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts`] module"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "This register interrupts the application for system-level events in the current mode (Device mode or Host mode). Some of the bits in this register are valid only in Host mode, while others are valid in Device mode only. This register also indicates the current mode. To clear the interrupt status bits of type R_SS_WC, the application must write 1'b1 to the bit. The FIFO status interrupts are read only; once software reads from or writes to the FIFO while servicing these interrupts, FIFO interrupt conditions are cleared automatically. The application must clear the GINTSTS register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization. Note: Read the reset value of GINTSTS.CurMod only after the following conditions: - If IDDIG_FILTER is disabled, read only after PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires."]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: This register works with the Interrupt Register (GINTSTS) to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the GINTSTS register bit corresponding to that interrupt is still set. Note: The fields of this register change depending on host or device mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk`] module"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "This register works with the Interrupt Register (GINTSTS) to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the GINTSTS register bit corresponding to that interrupt is still set. Note: The fields of this register change depending on host or device mode."]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: A read to the Receive Status Debug Read register returns the contents of the top of the Receive FIFO. The receive status contents must be interpreted differently in Host and Device modes. The core ignores the receive status read when the receive FIFO is empty and returns a value of 32'h0000_0000. Note: - Use of these fields vary based on whether the core is functioning as a host or a device. - Do not read this register's reset value before configuring the core because the read value is 'X' in the simulation.\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsr`] module"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "A read to the Receive Status Debug Read register returns the contents of the top of the Receive FIFO. The receive status contents must be interpreted differently in Host and Device modes. The core ignores the receive status read when the receive FIFO is empty and returns a value of 32'h0000_0000. Note: - Use of these fields vary based on whether the core is functioning as a host or a device. - Do not read this register's reset value before configuring the core because the read value is 'X' in the simulation."]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: A read to the Receive Status Read and Pop register returns the contents of the top of the Receive FIFO and additionally pops the top data entry out of the RxFIFO. The receive status contents must be interpreted differently in Host and Device modes. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 32'h0000_0000. The application must only pop the Receive Status FIFO when the Receive FIFO Non-Empty bit of the Core Interrupt register (GINTSTS.RxFLvl) is asserted. Note: - Use of these fields vary based on whether the core is functioning as a host or a device. - Do not read this register's reset value before configuring the core because the read value is 'X' in the simulation.\n\nYou can [`read`](crate::Reg::read) this register and get [`grxstsp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxstsp`] module"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "A read to the Receive Status Read and Pop register returns the contents of the top of the Receive FIFO and additionally pops the top data entry out of the RxFIFO. The receive status contents must be interpreted differently in Host and Device modes. The core ignores the receive status pop/read when the receive FIFO is empty and returns a value of 32'h0000_0000. The application must only pop the Receive Status FIFO when the Receive FIFO Non-Empty bit of the Core Interrupt register (GINTSTS.RxFLvl) is asserted. Note: - Use of these fields vary based on whether the core is functioning as a host or a device. - Do not read this register's reset value before configuring the core because the read value is 'X' in the simulation."]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: The application can program the RAM size that must be allocated to the RxFIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`grxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`grxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@grxfsiz`] module"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "The application can program the RAM size that must be allocated to the RxFIFO."]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: The application can program the RAM size and the memory start address for the Non-periodic TxFIFO Note: The fields of this register change depending on host or device mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gnptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxfsiz`] module"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "The application can program the RAM size and the memory start address for the Non-periodic TxFIFO Note: The fields of this register change depending on host or device mode."]
pub mod gnptxfsiz;
#[doc = "GNPTXSTS (r) register accessor: In Device mode, this register is valid only in Shared FIFO operation. This read-only register contains the free space information for the Non-periodic TxFIFO and the Non-periodic Transmit Request Queue.\n\nYou can [`read`](crate::Reg::read) this register and get [`gnptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gnptxsts`] module"]
pub type GNPTXSTS = crate::Reg<gnptxsts::GNPTXSTS_SPEC>;
#[doc = "In Device mode, this register is valid only in Shared FIFO operation. This read-only register contains the free space information for the Non-periodic TxFIFO and the Non-periodic Transmit Request Queue."]
pub mod gnptxsts;
#[doc = "GPVNDCTL (rw) register accessor: The application can use this register to access PHY registers. It is implemented only if Enable PHY Vendor Control Interface was selected during coreConsultant configuration (parameter OTG_VENDOR_CTL_INTERFACE = 1). For a UTMI+ PHY, the DWC_otg core uses the UTMI+ Vendor Control interface for PHY register access. For a ULPI PHY, the core uses the ULPI interface for PHY register access. The application sets Vendor Control register for PHY register access and times the PHY register access. The application polls the VStatus Done bit in this register for the completion of the PHY register access.\n\nYou can [`read`](crate::Reg::read) this register and get [`gpvndctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpvndctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpvndctl`] module"]
pub type GPVNDCTL = crate::Reg<gpvndctl::GPVNDCTL_SPEC>;
#[doc = "The application can use this register to access PHY registers. It is implemented only if Enable PHY Vendor Control Interface was selected during coreConsultant configuration (parameter OTG_VENDOR_CTL_INTERFACE = 1). For a UTMI+ PHY, the DWC_otg core uses the UTMI+ Vendor Control interface for PHY register access. For a ULPI PHY, the core uses the ULPI interface for PHY register access. The application sets Vendor Control register for PHY register access and times the PHY register access. The application polls the VStatus Done bit in this register for the completion of the PHY register access."]
pub mod gpvndctl;
#[doc = "GSNPSID (r) register accessor: This read-only register contains the release number of the core being used.\n\nYou can [`read`](crate::Reg::read) this register and get [`gsnpsid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gsnpsid`] module"]
pub type GSNPSID = crate::Reg<gsnpsid::GSNPSID_SPEC>;
#[doc = "This read-only register contains the release number of the core being used."]
pub mod gsnpsid;
#[doc = "GHWCFG1 (r) register accessor: This register contains the logical endpoint direction(s) selected using coreConsultant.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwcfg1`] module"]
pub type GHWCFG1 = crate::Reg<ghwcfg1::GHWCFG1_SPEC>;
#[doc = "This register contains the logical endpoint direction(s) selected using coreConsultant."]
pub mod ghwcfg1;
#[doc = "GHWCFG2 (r) register accessor: This register contains configuration options selected using coreConsultant.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwcfg2`] module"]
pub type GHWCFG2 = crate::Reg<ghwcfg2::GHWCFG2_SPEC>;
#[doc = "This register contains configuration options selected using coreConsultant."]
pub mod ghwcfg2;
#[doc = "GHWCFG3 (r) register accessor: This register contains configuration options selected using coreConsultant.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwcfg3`] module"]
pub type GHWCFG3 = crate::Reg<ghwcfg3::GHWCFG3_SPEC>;
#[doc = "This register contains configuration options selected using coreConsultant."]
pub mod ghwcfg3;
#[doc = "GHWCFG4 (r) register accessor: This register contains configuration options selected using coreConsultant. Note: Bit \\[31\\] is available only when Scatter/Gather DMA mode is enabled. When Scatter/Gather DMA mode is disabled, this field is reserved.\n\nYou can [`read`](crate::Reg::read) this register and get [`ghwcfg4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ghwcfg4`] module"]
pub type GHWCFG4 = crate::Reg<ghwcfg4::GHWCFG4_SPEC>;
#[doc = "This register contains configuration options selected using coreConsultant. Note: Bit \\[31\\] is available only when Scatter/Gather DMA mode is enabled. When Scatter/Gather DMA mode is disabled, this field is reserved."]
pub mod ghwcfg4;
#[doc = "GDFIFOCFG (rw) register accessor: Register to configure the DFIFOs for the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`gdfifocfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gdfifocfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gdfifocfg`] module"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Register to configure the DFIFOs for the controller."]
pub mod gdfifocfg;
#[doc = "GINTMSK2 (rw) register accessor: This register works with the Interrupt Register (GINTSTS2) to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the GINTSTS2 register bit corresponding to that interrupt is still set. Note: The fields of this register change depending on host or device mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`gintmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintmsk2`] module"]
pub type GINTMSK2 = crate::Reg<gintmsk2::GINTMSK2_SPEC>;
#[doc = "This register works with the Interrupt Register (GINTSTS2) to interrupt the application. When an interrupt bit is masked, the interrupt associated with that bit is not generated. However, the GINTSTS2 register bit corresponding to that interrupt is still set. Note: The fields of this register change depending on host or device mode."]
pub mod gintmsk2;
#[doc = "GINTSTS2 (rw) register accessor: This register interrupts the application for system-level events in the current mode (Device mode or Host mode). Some of the bits in this register are valid only in Host mode, while others are valid in Device mode only. This register also indicates the current mode. To clear the interrupt status bits of type R_SS_WC, the application must write 1'b1 to the bit. The application must clear the GINTSTS2 register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization.\n\nYou can [`read`](crate::Reg::read) this register and get [`gintsts2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gintsts2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gintsts2`] module"]
pub type GINTSTS2 = crate::Reg<gintsts2::GINTSTS2_SPEC>;
#[doc = "This register interrupts the application for system-level events in the current mode (Device mode or Host mode). Some of the bits in this register are valid only in Host mode, while others are valid in Device mode only. This register also indicates the current mode. To clear the interrupt status bits of type R_SS_WC, the application must write 1'b1 to the bit. The application must clear the GINTSTS2 register at initialization before unmasking the interrupt bit to avoid any interrupts generated prior to initialization."]
pub mod gintsts2;
#[doc = "HPTXFSIZ (rw) register accessor: This register holds the size and the memory start address of the Periodic TxFIFO. Note: Read the reset value of this register only after the following conditions: - If IDDIG_FILTER is disabled, read only after PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires.\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxfsiz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hptxfsiz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxfsiz`] module"]
pub type HPTXFSIZ = crate::Reg<hptxfsiz::HPTXFSIZ_SPEC>;
#[doc = "This register holds the size and the memory start address of the Periodic TxFIFO. Note: Read the reset value of this register only after the following conditions: - If IDDIG_FILTER is disabled, read only after PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires."]
pub mod hptxfsiz;
#[doc = "DIEPTXF1 (rw) register accessor: This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf1`] module"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf2`] module"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf3`] module"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
pub mod dieptxf3;
#[doc = "DIEPTXF4 (rw) register accessor: This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf4`] module"]
pub type DIEPTXF4 = crate::Reg<dieptxf4::DIEPTXF4_SPEC>;
#[doc = "This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
pub mod dieptxf4;
#[doc = "DIEPTXF5 (rw) register accessor: This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf5`] module"]
pub type DIEPTXF5 = crate::Reg<dieptxf5::DIEPTXF5_SPEC>;
#[doc = "This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
pub mod dieptxf5;
#[doc = "DIEPTXF6 (rw) register accessor: This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf6`] module"]
pub type DIEPTXF6 = crate::Reg<dieptxf6::DIEPTXF6_SPEC>;
#[doc = "This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
pub mod dieptxf6;
#[doc = "DIEPTXF7 (rw) register accessor: This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptxf7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptxf7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptxf7`] module"]
pub type DIEPTXF7 = crate::Reg<dieptxf7::DIEPTXF7_SPEC>;
#[doc = "This register is valid only in dedicated FIFO mode (OTG_EN_DED_TX_FIFO=1). It holds the size and memory start address of IN endpoint TxFIFOs implemented in Device mode. Each FIFO holds the data for one IN endpoint. This register is repeated for instantiated IN endpoint FIFOs 1 to 15. For IN endpoint FIFO 0, use GNPTXFSIZ register for programming the size and memory start address."]
pub mod dieptxf7;
#[doc = "HCFG (rw) register accessor: This register is used to configure the controller in Host mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfg`] module"]
pub type HCFG = crate::Reg<hcfg::HCFG_SPEC>;
#[doc = "This register is used to configure the controller in Host mode."]
pub mod hcfg;
#[doc = "HFIR (rw) register accessor: This register is used to control the interval between two consecutive SOFs.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hfir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfir`] module"]
pub type HFIR = crate::Reg<hfir::HFIR_SPEC>;
#[doc = "This register is used to control the interval between two consecutive SOFs."]
pub mod hfir;
#[doc = "HFNUM (r) register accessor: This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current (micro)frame. Note: Read the reset value of this register only after the following conditions: - If IDDIG_FILTER is disabled, read only when the PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires.\n\nYou can [`read`](crate::Reg::read) this register and get [`hfnum::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hfnum`] module"]
pub type HFNUM = crate::Reg<hfnum::HFNUM_SPEC>;
#[doc = "This register indicates the current frame number. It also indicates the time remaining (in terms of the number of PHY clocks) in the current (micro)frame. Note: Read the reset value of this register only after the following conditions: - If IDDIG_FILTER is disabled, read only when the PHY clock is stable. - If IDDIG_FILTER is enabled, read only after the filter timer expires."]
pub mod hfnum;
#[doc = "HPTXSTS (r) register accessor: This register contains information about the Periodic Transmit Queue in the Host controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`hptxsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hptxsts`] module"]
pub type HPTXSTS = crate::Reg<hptxsts::HPTXSTS_SPEC>;
#[doc = "This register contains information about the Periodic Transmit Queue in the Host controller."]
pub mod hptxsts;
#[doc = "HAINT (r) register accessor: When a significant event occurs on a channel, the Host All Channels Interrupt register interrupts the application using the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt). This is shown in the Interrupt Hierarchy figure in the databook. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Host Channel-n Interrupt register.\n\nYou can [`read`](crate::Reg::read) this register and get [`haint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haint`] module"]
pub type HAINT = crate::Reg<haint::HAINT_SPEC>;
#[doc = "When a significant event occurs on a channel, the Host All Channels Interrupt register interrupts the application using the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt). This is shown in the Interrupt Hierarchy figure in the databook. There is one interrupt bit per channel, up to a maximum of 16 bits. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Host Channel-n Interrupt register."]
pub mod haint;
#[doc = "HAINTMSK (rw) register accessor: The Host All Channel Interrupt Mask register works with the Host All Channel Interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits.\n\nYou can [`read`](crate::Reg::read) this register and get [`haintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`haintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@haintmsk`] module"]
pub type HAINTMSK = crate::Reg<haintmsk::HAINTMSK_SPEC>;
#[doc = "The Host All Channel Interrupt Mask register works with the Host All Channel Interrupt register to interrupt the application when an event occurs on a channel. There is one interrupt mask bit per channel, up to a maximum of 16 bits."]
pub mod haintmsk;
#[doc = "HFLBAddr (rw) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented as flops. This register holds the starting address of the Frame list information.\n\nYou can [`read`](crate::Reg::read) this register and get [`hflbaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hflbaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hflbaddr`] module"]
#[doc(alias = "HFLBAddr")]
pub type HFLBADDR = crate::Reg<hflbaddr::HFLBADDR_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented as flops. This register holds the starting address of the Frame list information."]
pub mod hflbaddr;
#[doc = "HPRT (rw) register accessor: This register is available only in Host mode. Currently, the OTG Host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in the Interrupt Hierarchy figure in the databook. The R_SS_WC bits in this register can trigger an interrupt to the application through the Host Port Interrupt bit of the Core Interrupt register (GINTSTS.PrtInt). On a Port Interrupt, the application must read this register and clear the bit that caused the interrupt. For the R_SS_WC bits, the application must write a 1 to the bit to clear the interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`hprt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hprt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hprt`] module"]
pub type HPRT = crate::Reg<hprt::HPRT_SPEC>;
#[doc = "This register is available only in Host mode. Currently, the OTG Host supports only one port. A single register holds USB port-related information such as USB reset, enable, suspend, resume, connect status, and test mode for each port. It is shown in the Interrupt Hierarchy figure in the databook. The R_SS_WC bits in this register can trigger an interrupt to the application through the Host Port Interrupt bit of the Core Interrupt register (GINTSTS.PrtInt). On a Port Interrupt, the application must read this register and clear the bit that caused the interrupt. For the R_SS_WC bits, the application must write a 1 to the bit to clear the interrupt."]
pub mod hprt;
#[doc = "HCCHAR0 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar0`] module"]
pub type HCCHAR0 = crate::Reg<hcchar0::HCCHAR0_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar0;
#[doc = "HCSPLT0 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt0`] module"]
pub type HCSPLT0 = crate::Reg<hcsplt0::HCSPLT0_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt0;
#[doc = "HCINT0 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint0`] module"]
pub type HCINT0 = crate::Reg<hcint0::HCINT0_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint0;
#[doc = "HCINTMSK0 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk0`] module"]
pub type HCINTMSK0 = crate::Reg<hcintmsk0::HCINTMSK0_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk0;
#[doc = "HCTSIZ0 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz0`] module"]
pub type HCTSIZ0 = crate::Reg<hctsiz0::HCTSIZ0_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz0;
#[doc = "HCDMA0 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma0`] module"]
pub type HCDMA0 = crate::Reg<hcdma0::HCDMA0_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma0;
#[doc = "HCDMAB0 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab0`] module"]
pub type HCDMAB0 = crate::Reg<hcdmab0::HCDMAB0_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab0;
#[doc = "HCCHAR1 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar1`] module"]
pub type HCCHAR1 = crate::Reg<hcchar1::HCCHAR1_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar1;
#[doc = "HCSPLT1 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt1`] module"]
pub type HCSPLT1 = crate::Reg<hcsplt1::HCSPLT1_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt1;
#[doc = "HCINT1 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint1`] module"]
pub type HCINT1 = crate::Reg<hcint1::HCINT1_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint1;
#[doc = "HCINTMSK1 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk1`] module"]
pub type HCINTMSK1 = crate::Reg<hcintmsk1::HCINTMSK1_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk1;
#[doc = "HCTSIZ1 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz1`] module"]
pub type HCTSIZ1 = crate::Reg<hctsiz1::HCTSIZ1_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz1;
#[doc = "HCDMA1 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma1`] module"]
pub type HCDMA1 = crate::Reg<hcdma1::HCDMA1_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma1;
#[doc = "HCDMAB1 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab1`] module"]
pub type HCDMAB1 = crate::Reg<hcdmab1::HCDMAB1_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab1;
#[doc = "HCCHAR2 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar2`] module"]
pub type HCCHAR2 = crate::Reg<hcchar2::HCCHAR2_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar2;
#[doc = "HCSPLT2 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt2`] module"]
pub type HCSPLT2 = crate::Reg<hcsplt2::HCSPLT2_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt2;
#[doc = "HCINT2 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint2`] module"]
pub type HCINT2 = crate::Reg<hcint2::HCINT2_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint2;
#[doc = "HCINTMSK2 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk2`] module"]
pub type HCINTMSK2 = crate::Reg<hcintmsk2::HCINTMSK2_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk2;
#[doc = "HCTSIZ2 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz2`] module"]
pub type HCTSIZ2 = crate::Reg<hctsiz2::HCTSIZ2_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz2;
#[doc = "HCDMA2 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma2`] module"]
pub type HCDMA2 = crate::Reg<hcdma2::HCDMA2_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma2;
#[doc = "HCDMAB2 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab2`] module"]
pub type HCDMAB2 = crate::Reg<hcdmab2::HCDMAB2_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab2;
#[doc = "HCCHAR3 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar3`] module"]
pub type HCCHAR3 = crate::Reg<hcchar3::HCCHAR3_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar3;
#[doc = "HCSPLT3 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt3`] module"]
pub type HCSPLT3 = crate::Reg<hcsplt3::HCSPLT3_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt3;
#[doc = "HCINT3 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint3`] module"]
pub type HCINT3 = crate::Reg<hcint3::HCINT3_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint3;
#[doc = "HCINTMSK3 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk3`] module"]
pub type HCINTMSK3 = crate::Reg<hcintmsk3::HCINTMSK3_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk3;
#[doc = "HCTSIZ3 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz3`] module"]
pub type HCTSIZ3 = crate::Reg<hctsiz3::HCTSIZ3_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz3;
#[doc = "HCDMA3 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma3`] module"]
pub type HCDMA3 = crate::Reg<hcdma3::HCDMA3_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma3;
#[doc = "HCDMAB3 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab3`] module"]
pub type HCDMAB3 = crate::Reg<hcdmab3::HCDMAB3_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab3;
#[doc = "HCCHAR4 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar4`] module"]
pub type HCCHAR4 = crate::Reg<hcchar4::HCCHAR4_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar4;
#[doc = "HCSPLT4 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt4`] module"]
pub type HCSPLT4 = crate::Reg<hcsplt4::HCSPLT4_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt4;
#[doc = "HCINT4 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint4`] module"]
pub type HCINT4 = crate::Reg<hcint4::HCINT4_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint4;
#[doc = "HCINTMSK4 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk4`] module"]
pub type HCINTMSK4 = crate::Reg<hcintmsk4::HCINTMSK4_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk4;
#[doc = "HCTSIZ4 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz4`] module"]
pub type HCTSIZ4 = crate::Reg<hctsiz4::HCTSIZ4_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz4;
#[doc = "HCDMA4 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma4`] module"]
pub type HCDMA4 = crate::Reg<hcdma4::HCDMA4_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma4;
#[doc = "HCDMAB4 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab4`] module"]
pub type HCDMAB4 = crate::Reg<hcdmab4::HCDMAB4_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab4;
#[doc = "HCCHAR5 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar5`] module"]
pub type HCCHAR5 = crate::Reg<hcchar5::HCCHAR5_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar5;
#[doc = "HCSPLT5 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt5`] module"]
pub type HCSPLT5 = crate::Reg<hcsplt5::HCSPLT5_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt5;
#[doc = "HCINT5 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint5`] module"]
pub type HCINT5 = crate::Reg<hcint5::HCINT5_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint5;
#[doc = "HCINTMSK5 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk5`] module"]
pub type HCINTMSK5 = crate::Reg<hcintmsk5::HCINTMSK5_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk5;
#[doc = "HCTSIZ5 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz5`] module"]
pub type HCTSIZ5 = crate::Reg<hctsiz5::HCTSIZ5_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz5;
#[doc = "HCDMA5 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma5`] module"]
pub type HCDMA5 = crate::Reg<hcdma5::HCDMA5_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma5;
#[doc = "HCDMAB5 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab5`] module"]
pub type HCDMAB5 = crate::Reg<hcdmab5::HCDMAB5_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab5;
#[doc = "HCCHAR6 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar6`] module"]
pub type HCCHAR6 = crate::Reg<hcchar6::HCCHAR6_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar6;
#[doc = "HCSPLT6 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt6`] module"]
pub type HCSPLT6 = crate::Reg<hcsplt6::HCSPLT6_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt6;
#[doc = "HCINT6 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint6`] module"]
pub type HCINT6 = crate::Reg<hcint6::HCINT6_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint6;
#[doc = "HCINTMSK6 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk6`] module"]
pub type HCINTMSK6 = crate::Reg<hcintmsk6::HCINTMSK6_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk6;
#[doc = "HCTSIZ6 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz6`] module"]
pub type HCTSIZ6 = crate::Reg<hctsiz6::HCTSIZ6_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz6;
#[doc = "HCDMA6 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma6`] module"]
pub type HCDMA6 = crate::Reg<hcdma6::HCDMA6_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma6;
#[doc = "HCDMAB6 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab6`] module"]
pub type HCDMAB6 = crate::Reg<hcdmab6::HCDMAB6_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab6;
#[doc = "HCCHAR7 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar7`] module"]
pub type HCCHAR7 = crate::Reg<hcchar7::HCCHAR7_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar7;
#[doc = "HCSPLT7 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt7`] module"]
pub type HCSPLT7 = crate::Reg<hcsplt7::HCSPLT7_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt7;
#[doc = "HCINT7 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint7`] module"]
pub type HCINT7 = crate::Reg<hcint7::HCINT7_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint7;
#[doc = "HCINTMSK7 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk7`] module"]
pub type HCINTMSK7 = crate::Reg<hcintmsk7::HCINTMSK7_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk7;
#[doc = "HCTSIZ7 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz7`] module"]
pub type HCTSIZ7 = crate::Reg<hctsiz7::HCTSIZ7_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz7;
#[doc = "HCDMA7 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma7`] module"]
pub type HCDMA7 = crate::Reg<hcdma7::HCDMA7_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma7;
#[doc = "HCDMAB7 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab7`] module"]
pub type HCDMAB7 = crate::Reg<hcdmab7::HCDMAB7_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab7;
#[doc = "HCCHAR8 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar8`] module"]
pub type HCCHAR8 = crate::Reg<hcchar8::HCCHAR8_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar8;
#[doc = "HCSPLT8 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt8`] module"]
pub type HCSPLT8 = crate::Reg<hcsplt8::HCSPLT8_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt8;
#[doc = "HCINT8 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint8`] module"]
pub type HCINT8 = crate::Reg<hcint8::HCINT8_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint8;
#[doc = "HCINTMSK8 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk8`] module"]
pub type HCINTMSK8 = crate::Reg<hcintmsk8::HCINTMSK8_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk8;
#[doc = "HCTSIZ8 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz8`] module"]
pub type HCTSIZ8 = crate::Reg<hctsiz8::HCTSIZ8_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz8;
#[doc = "HCDMA8 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma8`] module"]
pub type HCDMA8 = crate::Reg<hcdma8::HCDMA8_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma8;
#[doc = "HCDMAB8 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab8`] module"]
pub type HCDMAB8 = crate::Reg<hcdmab8::HCDMAB8_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab8;
#[doc = "HCCHAR9 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar9`] module"]
pub type HCCHAR9 = crate::Reg<hcchar9::HCCHAR9_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar9;
#[doc = "HCSPLT9 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt9`] module"]
pub type HCSPLT9 = crate::Reg<hcsplt9::HCSPLT9_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt9;
#[doc = "HCINT9 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint9`] module"]
pub type HCINT9 = crate::Reg<hcint9::HCINT9_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint9;
#[doc = "HCINTMSK9 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk9`] module"]
pub type HCINTMSK9 = crate::Reg<hcintmsk9::HCINTMSK9_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk9;
#[doc = "HCTSIZ9 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz9`] module"]
pub type HCTSIZ9 = crate::Reg<hctsiz9::HCTSIZ9_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz9;
#[doc = "HCDMA9 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma9`] module"]
pub type HCDMA9 = crate::Reg<hcdma9::HCDMA9_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma9;
#[doc = "HCDMAB9 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab9`] module"]
pub type HCDMAB9 = crate::Reg<hcdmab9::HCDMAB9_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab9;
#[doc = "HCCHAR10 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar10`] module"]
pub type HCCHAR10 = crate::Reg<hcchar10::HCCHAR10_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar10;
#[doc = "HCSPLT10 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt10`] module"]
pub type HCSPLT10 = crate::Reg<hcsplt10::HCSPLT10_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt10;
#[doc = "HCINT10 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint10`] module"]
pub type HCINT10 = crate::Reg<hcint10::HCINT10_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint10;
#[doc = "HCINTMSK10 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk10`] module"]
pub type HCINTMSK10 = crate::Reg<hcintmsk10::HCINTMSK10_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk10;
#[doc = "HCTSIZ10 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz10`] module"]
pub type HCTSIZ10 = crate::Reg<hctsiz10::HCTSIZ10_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz10;
#[doc = "HCDMA10 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma10`] module"]
pub type HCDMA10 = crate::Reg<hcdma10::HCDMA10_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma10;
#[doc = "HCDMAB10 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab10`] module"]
pub type HCDMAB10 = crate::Reg<hcdmab10::HCDMAB10_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab10;
#[doc = "HCCHAR11 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar11`] module"]
pub type HCCHAR11 = crate::Reg<hcchar11::HCCHAR11_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar11;
#[doc = "HCSPLT11 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt11`] module"]
pub type HCSPLT11 = crate::Reg<hcsplt11::HCSPLT11_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt11;
#[doc = "HCINT11 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint11`] module"]
pub type HCINT11 = crate::Reg<hcint11::HCINT11_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint11;
#[doc = "HCINTMSK11 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk11`] module"]
pub type HCINTMSK11 = crate::Reg<hcintmsk11::HCINTMSK11_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk11;
#[doc = "HCTSIZ11 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz11`] module"]
pub type HCTSIZ11 = crate::Reg<hctsiz11::HCTSIZ11_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz11;
#[doc = "HCDMA11 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma11`] module"]
pub type HCDMA11 = crate::Reg<hcdma11::HCDMA11_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma11;
#[doc = "HCDMAB11 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab11`] module"]
pub type HCDMAB11 = crate::Reg<hcdmab11::HCDMAB11_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab11;
#[doc = "HCCHAR12 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar12`] module"]
pub type HCCHAR12 = crate::Reg<hcchar12::HCCHAR12_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar12;
#[doc = "HCSPLT12 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt12`] module"]
pub type HCSPLT12 = crate::Reg<hcsplt12::HCSPLT12_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt12;
#[doc = "HCINT12 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint12`] module"]
pub type HCINT12 = crate::Reg<hcint12::HCINT12_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint12;
#[doc = "HCINTMSK12 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk12`] module"]
pub type HCINTMSK12 = crate::Reg<hcintmsk12::HCINTMSK12_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk12;
#[doc = "HCTSIZ12 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz12`] module"]
pub type HCTSIZ12 = crate::Reg<hctsiz12::HCTSIZ12_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz12;
#[doc = "HCDMA12 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma12`] module"]
pub type HCDMA12 = crate::Reg<hcdma12::HCDMA12_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma12;
#[doc = "HCDMAB12 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab12`] module"]
pub type HCDMAB12 = crate::Reg<hcdmab12::HCDMAB12_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab12;
#[doc = "HCCHAR13 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar13`] module"]
pub type HCCHAR13 = crate::Reg<hcchar13::HCCHAR13_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar13;
#[doc = "HCSPLT13 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt13`] module"]
pub type HCSPLT13 = crate::Reg<hcsplt13::HCSPLT13_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt13;
#[doc = "HCINT13 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint13`] module"]
pub type HCINT13 = crate::Reg<hcint13::HCINT13_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint13;
#[doc = "HCINTMSK13 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk13`] module"]
pub type HCINTMSK13 = crate::Reg<hcintmsk13::HCINTMSK13_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk13;
#[doc = "HCTSIZ13 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz13`] module"]
pub type HCTSIZ13 = crate::Reg<hctsiz13::HCTSIZ13_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz13;
#[doc = "HCDMA13 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma13`] module"]
pub type HCDMA13 = crate::Reg<hcdma13::HCDMA13_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma13;
#[doc = "HCDMAB13 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab13`] module"]
pub type HCDMAB13 = crate::Reg<hcdmab13::HCDMAB13_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab13;
#[doc = "HCCHAR14 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar14`] module"]
pub type HCCHAR14 = crate::Reg<hcchar14::HCCHAR14_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar14;
#[doc = "HCSPLT14 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt14`] module"]
pub type HCSPLT14 = crate::Reg<hcsplt14::HCSPLT14_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt14;
#[doc = "HCINT14 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint14`] module"]
pub type HCINT14 = crate::Reg<hcint14::HCINT14_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint14;
#[doc = "HCINTMSK14 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk14`] module"]
pub type HCINTMSK14 = crate::Reg<hcintmsk14::HCINTMSK14_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk14;
#[doc = "HCTSIZ14 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz14`] module"]
pub type HCTSIZ14 = crate::Reg<hctsiz14::HCTSIZ14_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz14;
#[doc = "HCDMA14 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma14`] module"]
pub type HCDMA14 = crate::Reg<hcdma14::HCDMA14_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma14;
#[doc = "HCDMAB14 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab14`] module"]
pub type HCDMAB14 = crate::Reg<hcdmab14::HCDMAB14_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab14;
#[doc = "HCCHAR15 (rw) register accessor: This register contains the characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcchar15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcchar15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcchar15`] module"]
pub type HCCHAR15 = crate::Reg<hcchar15::HCCHAR15_SPEC>;
#[doc = "This register contains the characteristics of the Host Channel."]
pub mod hcchar15;
#[doc = "HCSPLT15 (rw) register accessor: This register contains the Split characteristics of the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcsplt15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcsplt15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcsplt15`] module"]
pub type HCSPLT15 = crate::Reg<hcsplt15::HCSPLT15_SPEC>;
#[doc = "This register contains the Split characteristics of the Host Channel."]
pub mod hcsplt15;
#[doc = "HCINT15 (rw) register accessor: This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcint15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcint15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcint15`] module"]
pub type HCINT15 = crate::Reg<hcint15::HCINT15_SPEC>;
#[doc = "This register indicates the status of a channel with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the Host Channels Interrupt bit of the Core Interrupt register (GINTSTS.HChInt) is set. Before the application can read this register, it must first read the Host All Channels Interrupt (HAINT) register to get the exact channel number for the Host Channel-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the HAINT and GINTSTS registers."]
pub mod hcint15;
#[doc = "HCINTMSK15 (rw) register accessor: This register reflects the mask for each channel status described in the previous section.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcintmsk15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcintmsk15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcintmsk15`] module"]
pub type HCINTMSK15 = crate::Reg<hcintmsk15::HCINTMSK15_SPEC>;
#[doc = "This register reflects the mask for each channel status described in the previous section."]
pub mod hcintmsk15;
#[doc = "HCTSIZ15 (rw) register accessor: This register reflects the transfer size for the Host Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`hctsiz15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hctsiz15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hctsiz15`] module"]
pub type HCTSIZ15 = crate::Reg<hctsiz15::HCTSIZ15_SPEC>;
#[doc = "This register reflects the transfer size for the Host Channel."]
pub mod hctsiz15;
#[doc = "HCDMA15 (rw) register accessor: This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdma15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcdma15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdma15`] module"]
pub type HCDMA15 = crate::Reg<hcdma15::HCDMA15_SPEC>;
#[doc = "This register is used by the OTG host in the internal DMA mode to maintain the current buffer pointer for IN/OUT transactions. The starting DMA address must be DWORD-aligned."]
pub mod hcdma15;
#[doc = "HCDMAB15 (r) register accessor: This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdmab15`] module"]
pub type HCDMAB15 = crate::Reg<hcdmab15::HCDMAB15_SPEC>;
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address."]
pub mod hcdmab15;
#[doc = "DCFG (rw) register accessor: This register configures the core in Device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcfg`] module"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "This register configures the core in Device mode after power-on or after certain control commands or enumeration. Do not make changes to this register after initial programming."]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: This register is used to control the characteristics of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`dctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dctl`] module"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "This register is used to control the characteristics of the Device controller."]
pub mod dctl;
#[doc = "DSTS (r) register accessor: This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from Device All Interrupts (DAINT) register.\n\nYou can [`read`](crate::Reg::read) this register and get [`dsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsts`] module"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "This register indicates the status of the core with respect to USB-related events. It must be read on interrupts from Device All Interrupts (DAINT) register."]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: This register works with each of the Device IN Endpoint Interrupt (DIEPINTn) registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTn register can be masked by writing to the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepmsk`] module"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "This register works with each of the Device IN Endpoint Interrupt (DIEPINTn) registers for all endpoints to generate an interrupt per IN endpoint. The IN endpoint interrupt for a specific status in the DIEPINTn register can be masked by writing to the corresponding bit in this register. Status bits are masked by default."]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: This register works with each of the Device OUT Endpoint Interrupt (DOEPINTn) registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTn register can be masked by writing into the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepmsk`] module"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "This register works with each of the Device OUT Endpoint Interrupt (DOEPINTn) registers for all endpoints to generate an interrupt per OUT endpoint. The OUT endpoint interrupt for a specific status in the DOEPINTn register can be masked by writing into the corresponding bit in this register. Status bits are masked by default."]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: When a significant event occurs on an endpoint, a Device All Endpoints Interrupt register interrupts the application using the Device OUT Endpoints Interrupt bit or Device IN Endpoints Interrupt bit of the Core Interrupt register (GINTSTS.OEPInt or GINTSTS.IEPInt, respectively). This is shown in Figure 5-2. There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-n Interrupt register (DIEPINTn/DOEPINTn).\n\nYou can [`read`](crate::Reg::read) this register and get [`daint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daint`] module"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "When a significant event occurs on an endpoint, a Device All Endpoints Interrupt register interrupts the application using the Device OUT Endpoints Interrupt bit or Device IN Endpoints Interrupt bit of the Core Interrupt register (GINTSTS.OEPInt or GINTSTS.IEPInt, respectively). This is shown in Figure 5-2. There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-n Interrupt register (DIEPINTn/DOEPINTn)."]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: The Device Endpoint Interrupt Mask register works with the Device Endpoint Interrupt register to interrupt the application when an event occurs on a device endpoint. However, the Device All Endpoints Interrupt (DAINT) register bit corresponding to that interrupt is still set.\n\nYou can [`read`](crate::Reg::read) this register and get [`daintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@daintmsk`] module"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "The Device Endpoint Interrupt Mask register works with the Device Endpoint Interrupt register to interrupt the application when an event occurs on a device endpoint. However, the Device All Endpoints Interrupt (DAINT) register bit corresponding to that interrupt is still set."]
pub mod daintmsk;
#[doc = "DVBUSDIS (rw) register accessor: This register specifies the VBUS discharge time after VBUS pulsing during SRP.\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbusdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbusdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbusdis`] module"]
pub type DVBUSDIS = crate::Reg<dvbusdis::DVBUSDIS_SPEC>;
#[doc = "This register specifies the VBUS discharge time after VBUS pulsing during SRP."]
pub mod dvbusdis;
#[doc = "DVBUSPULSE (rw) register accessor: This register contains the VBUS pulsing time during SRP.\n\nYou can [`read`](crate::Reg::read) this register and get [`dvbuspulse::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvbuspulse::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dvbuspulse`] module"]
pub type DVBUSPULSE = crate::Reg<dvbuspulse::DVBUSPULSE_SPEC>;
#[doc = "This register contains the VBUS pulsing time during SRP."]
pub mod dvbuspulse;
#[doc = "DTHRCTL (rw) register accessor: This register contains the Receive and Transmit Threshold characteristics of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`dthrctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dthrctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dthrctl`] module"]
pub type DTHRCTL = crate::Reg<dthrctl::DTHRCTL_SPEC>;
#[doc = "This register contains the Receive and Transmit Threshold characteristics of the Device controller."]
pub mod dthrctl;
#[doc = "DIEPEMPMSK (rw) register accessor: This register is valid only in Dedicated FIFO operation (OTG_EN_DED_TX_FIFO = 1). This register is used to control the IN endpoint FIFO empty interrupt generation (DIEPINTn.TxfEmp).\n\nYou can [`read`](crate::Reg::read) this register and get [`diepempmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepempmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepempmsk`] module"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "This register is valid only in Dedicated FIFO operation (OTG_EN_DED_TX_FIFO = 1). This register is used to control the IN endpoint FIFO empty interrupt generation (DIEPINTn.TxfEmp)."]
pub mod diepempmsk;
#[doc = "DEACHINT (r) register accessor: This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT on page 121=1. There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-n Interrupt register (DIEPINTn/DOEPINTn). The interrupt is automatically cleared once the DOEPINTn/DIEPINTn interrupt is cleared by the application.\n\nYou can [`read`](crate::Reg::read) this register and get [`deachint::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deachint`] module"]
pub type DEACHINT = crate::Reg<deachint::DEACHINT_SPEC>;
#[doc = "This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT on page 121=1. There is one interrupt bit per endpoint, up to a maximum of 16 bits for OUT endpoints and 16 bits for IN endpoints. For a bidirectional endpoint, the corresponding IN and OUT interrupt bits are used. Bits in this register are set and cleared when the application sets and clears bits in the corresponding Device Endpoint-n Interrupt register (DIEPINTn/DOEPINTn). The interrupt is automatically cleared once the DOEPINTn/DIEPINTn interrupt is cleared by the application."]
pub mod deachint;
#[doc = "DEACHINTMSK (rw) register accessor: This register is available only when parameter OTG_MULTI_PROC_INTRPT=1. The Device Each Endpoint Interrupt Mask register works with the Device Each Endpoint Interrupt register to interrupt the application when an event occurs on a device endpoint. However, the Device Each Endpoints Interrupt (DEACHINT) register bit corresponding to that interrupt remains set.\n\nYou can [`read`](crate::Reg::read) this register and get [`deachintmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`deachintmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@deachintmsk`] module"]
pub type DEACHINTMSK = crate::Reg<deachintmsk::DEACHINTMSK_SPEC>;
#[doc = "This register is available only when parameter OTG_MULTI_PROC_INTRPT=1. The Device Each Endpoint Interrupt Mask register works with the Device Each Endpoint Interrupt register to interrupt the application when an event occurs on a device endpoint. However, the Device Each Endpoints Interrupt (DEACHINT) register bit corresponding to that interrupt remains set."]
pub mod deachintmsk;
#[doc = "DIEPEACHMSK0 (rw) register accessor: This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT on page 121=1. These registers are endpoint-specific mask registers for (DIEPINTn). The IN endpoint interrupt for a specific status in the DIEPINTn register can be masked by writing 0 to the corresponding bit in this register. Status bits are masked by default. - Mask interrupt: 1'b0 - Unmask interrupt: 1'b1\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk0`] module"]
pub type DIEPEACHMSK0 = crate::Reg<diepeachmsk0::DIEPEACHMSK0_SPEC>;
#[doc = "This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT on page 121=1. These registers are endpoint-specific mask registers for (DIEPINTn). The IN endpoint interrupt for a specific status in the DIEPINTn register can be masked by writing 0 to the corresponding bit in this register. Status bits are masked by default. - Mask interrupt: 1'b0 - Unmask interrupt: 1'b1"]
pub mod diepeachmsk0;
#[doc = "DIEPEACHMSK1 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk1`] module"]
pub type DIEPEACHMSK1 = crate::Reg<diepeachmsk1::DIEPEACHMSK1_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk1;
#[doc = "DIEPEACHMSK2 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk2`] module"]
pub type DIEPEACHMSK2 = crate::Reg<diepeachmsk2::DIEPEACHMSK2_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk2;
#[doc = "DIEPEACHMSK3 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk3`] module"]
pub type DIEPEACHMSK3 = crate::Reg<diepeachmsk3::DIEPEACHMSK3_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk3;
#[doc = "DIEPEACHMSK4 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk4`] module"]
pub type DIEPEACHMSK4 = crate::Reg<diepeachmsk4::DIEPEACHMSK4_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk4;
#[doc = "DIEPEACHMSK5 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk5`] module"]
pub type DIEPEACHMSK5 = crate::Reg<diepeachmsk5::DIEPEACHMSK5_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk5;
#[doc = "DIEPEACHMSK6 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk6`] module"]
pub type DIEPEACHMSK6 = crate::Reg<diepeachmsk6::DIEPEACHMSK6_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk6;
#[doc = "DIEPEACHMSK7 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk7`] module"]
pub type DIEPEACHMSK7 = crate::Reg<diepeachmsk7::DIEPEACHMSK7_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk7;
#[doc = "DIEPEACHMSK8 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk8`] module"]
pub type DIEPEACHMSK8 = crate::Reg<diepeachmsk8::DIEPEACHMSK8_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk8;
#[doc = "DIEPEACHMSK9 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk9`] module"]
pub type DIEPEACHMSK9 = crate::Reg<diepeachmsk9::DIEPEACHMSK9_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk9;
#[doc = "DIEPEACHMSK10 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk10`] module"]
pub type DIEPEACHMSK10 = crate::Reg<diepeachmsk10::DIEPEACHMSK10_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk10;
#[doc = "DIEPEACHMSK11 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk11`] module"]
pub type DIEPEACHMSK11 = crate::Reg<diepeachmsk11::DIEPEACHMSK11_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk11;
#[doc = "DIEPEACHMSK12 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk12`] module"]
pub type DIEPEACHMSK12 = crate::Reg<diepeachmsk12::DIEPEACHMSK12_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk12;
#[doc = "DIEPEACHMSK13 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk13`] module"]
pub type DIEPEACHMSK13 = crate::Reg<diepeachmsk13::DIEPEACHMSK13_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk13;
#[doc = "DIEPEACHMSK14 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk14`] module"]
pub type DIEPEACHMSK14 = crate::Reg<diepeachmsk14::DIEPEACHMSK14_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk14;
#[doc = "DIEPEACHMSK15 (rw) register accessor: This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepeachmsk15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepeachmsk15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepeachmsk15`] module"]
pub type DIEPEACHMSK15 = crate::Reg<diepeachmsk15::DIEPEACHMSK15_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepeachmsk15;
#[doc = "DOEPEACHMSK0 (rw) register accessor: This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT=1. These registers are endpoint specific mask registers for (DOEPINTn). The OUT endpoint interrupt for a specific status in the DOEPINTn register can be masked by writing 0 to the corresponding bit in this register. Status bits are masked by default.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk0`] module"]
pub type DOEPEACHMSK0 = crate::Reg<doepeachmsk0::DOEPEACHMSK0_SPEC>;
#[doc = "This register is available in device mode and only when parameter OTG_MULTI_PROC_INTRPT=1. These registers are endpoint specific mask registers for (DOEPINTn). The OUT endpoint interrupt for a specific status in the DOEPINTn register can be masked by writing 0 to the corresponding bit in this register. Status bits are masked by default."]
pub mod doepeachmsk0;
#[doc = "DOEPEACHMSK1 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk1`] module"]
pub type DOEPEACHMSK1 = crate::Reg<doepeachmsk1::DOEPEACHMSK1_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk1;
#[doc = "DOEPEACHMSK2 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk2`] module"]
pub type DOEPEACHMSK2 = crate::Reg<doepeachmsk2::DOEPEACHMSK2_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk2;
#[doc = "DOEPEACHMSK3 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk3`] module"]
pub type DOEPEACHMSK3 = crate::Reg<doepeachmsk3::DOEPEACHMSK3_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk3;
#[doc = "DOEPEACHMSK4 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk4`] module"]
pub type DOEPEACHMSK4 = crate::Reg<doepeachmsk4::DOEPEACHMSK4_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk4;
#[doc = "DOEPEACHMSK5 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk5`] module"]
pub type DOEPEACHMSK5 = crate::Reg<doepeachmsk5::DOEPEACHMSK5_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk5;
#[doc = "DOEPEACHMSK6 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk6`] module"]
pub type DOEPEACHMSK6 = crate::Reg<doepeachmsk6::DOEPEACHMSK6_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk6;
#[doc = "DOEPEACHMSK7 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk7`] module"]
pub type DOEPEACHMSK7 = crate::Reg<doepeachmsk7::DOEPEACHMSK7_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk7;
#[doc = "DOEPEACHMSK8 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk8`] module"]
pub type DOEPEACHMSK8 = crate::Reg<doepeachmsk8::DOEPEACHMSK8_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk8;
#[doc = "DOEPEACHMSK9 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk9`] module"]
pub type DOEPEACHMSK9 = crate::Reg<doepeachmsk9::DOEPEACHMSK9_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk9;
#[doc = "DOEPEACHMSK10 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk10`] module"]
pub type DOEPEACHMSK10 = crate::Reg<doepeachmsk10::DOEPEACHMSK10_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk10;
#[doc = "DOEPEACHMSK11 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk11`] module"]
pub type DOEPEACHMSK11 = crate::Reg<doepeachmsk11::DOEPEACHMSK11_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk11;
#[doc = "DOEPEACHMSK12 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk12`] module"]
pub type DOEPEACHMSK12 = crate::Reg<doepeachmsk12::DOEPEACHMSK12_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk12;
#[doc = "DOEPEACHMSK13 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk13`] module"]
pub type DOEPEACHMSK13 = crate::Reg<doepeachmsk13::DOEPEACHMSK13_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk13;
#[doc = "DOEPEACHMSK14 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk14`] module"]
pub type DOEPEACHMSK14 = crate::Reg<doepeachmsk14::DOEPEACHMSK14_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk14;
#[doc = "DOEPEACHMSK15 (rw) register accessor: This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepeachmsk15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepeachmsk15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepeachmsk15`] module"]
pub type DOEPEACHMSK15 = crate::Reg<doepeachmsk15::DOEPEACHMSK15_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoints of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod doepeachmsk15;
#[doc = "DIEPCTL0 (rw) register accessor: This register is used to control the characteristics of the IN Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl0`] module"]
pub type DIEPCTL0 = crate::Reg<diepctl0::DIEPCTL0_SPEC>;
#[doc = "This register is used to control the characteristics of the IN Endpoint 0 of the Device controller."]
pub mod diepctl0;
#[doc = "DIEPINT0 (rw) register accessor: This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the OUT Endpoints Interrupt bit or IN Endpoints Interrupt bit of the Core Interrupt register (GINTSTS.OEPInt or GINTSTS.IEPInt, respectively) is set. Before the application can read this register, it must first read the Device All Endpoints Interrupt (DAINT) register to get the exact endpoint number for the Device Endpoint-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint0`] module"]
pub type DIEPINT0 = crate::Reg<diepint0::DIEPINT0_SPEC>;
#[doc = "This register indicates the status of an endpoint with respect to USB- and AHB-related events. It is shown in the Interrupt Hierarchy figure in the databook. The application must read this register when the OUT Endpoints Interrupt bit or IN Endpoints Interrupt bit of the Core Interrupt register (GINTSTS.OEPInt or GINTSTS.IEPInt, respectively) is set. Before the application can read this register, it must first read the Device All Endpoints Interrupt (DAINT) register to get the exact endpoint number for the Device Endpoint-n Interrupt register. The application must clear the appropriate bit in this register to clear the corresponding bits in the DAINT and GINTSTS registers"]
pub mod diepint0;
#[doc = "DIEPTSIZ0 (rw) register accessor: The application must modify this register before enabling endpoint 0. Once endpoint 0 is enabled using Endpoint Enable bit of the Device Control Endpoint 0 Control registers (DIEPCTL0.EPEna/DOEPCTL0.EPEna), the core modifies this register. The application can only read this register once the core has cleared the Endpoint Enable bit. Nonzero endpoints use the registers for endpoints 115. When Scatter/Gather DMA mode is enabled, this register must not be programmed by the application. If the application reads this register when Scatter/Gather DMA mode is enabled, the core returns all zeros.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz0`] module"]
pub type DIEPTSIZ0 = crate::Reg<dieptsiz0::DIEPTSIZ0_SPEC>;
#[doc = "The application must modify this register before enabling endpoint 0. Once endpoint 0 is enabled using Endpoint Enable bit of the Device Control Endpoint 0 Control registers (DIEPCTL0.EPEna/DOEPCTL0.EPEna), the core modifies this register. The application can only read this register once the core has cleared the Endpoint Enable bit. Nonzero endpoints use the registers for endpoints 115. When Scatter/Gather DMA mode is enabled, this register must not be programmed by the application. If the application reads this register when Scatter/Gather DMA mode is enabled, the core returns all zeros."]
pub mod dieptsiz0;
#[doc = "DIEPDMA0 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma0`] module"]
pub type DIEPDMA0 = crate::Reg<diepdma0::DIEPDMA0_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 0 of the Device controller."]
pub mod diepdma0;
#[doc = "DTXFSTS0 (r) register accessor: This register contains information about the IN Endpoint Transmit FIFO of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts0`] module"]
pub type DTXFSTS0 = crate::Reg<dtxfsts0::DTXFSTS0_SPEC>;
#[doc = "This register contains information about the IN Endpoint Transmit FIFO of the Device controller."]
pub mod dtxfsts0;
#[doc = "DIEPDMAB0 (r) register accessor: This register contains the DMA Buffer Address for the IN Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab0`] module"]
pub type DIEPDMAB0 = crate::Reg<diepdmab0::DIEPDMAB0_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the IN Endpoint 0 of the Device controller."]
pub mod diepdmab0;
#[doc = "DIEPCTL1 (rw) register accessor: This register is used to control the characteristics of Endpoint 1. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl1`] module"]
pub type DIEPCTL1 = crate::Reg<diepctl1::DIEPCTL1_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 1. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl1;
#[doc = "DIEPINT1 (rw) register accessor: This register contains the interrupts for the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint1`] module"]
pub type DIEPINT1 = crate::Reg<diepint1::DIEPINT1_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint1;
#[doc = "DIEPTSIZ1 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz1`] module"]
pub type DIEPTSIZ1 = crate::Reg<dieptsiz1::DIEPTSIZ1_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz1;
#[doc = "DIEPDMA1 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma1`] module"]
pub type DIEPDMA1 = crate::Reg<diepdma1::DIEPDMA1_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma1;
#[doc = "DTXFSTS1 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts1`] module"]
pub type DTXFSTS1 = crate::Reg<dtxfsts1::DTXFSTS1_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts1;
#[doc = "DIEPDMAB1 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab1`] module"]
pub type DIEPDMAB1 = crate::Reg<diepdmab1::DIEPDMAB1_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 1 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab1;
#[doc = "DIEPCTL2 (rw) register accessor: This register is used to control the characteristics of Endpoint 2. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl2`] module"]
pub type DIEPCTL2 = crate::Reg<diepctl2::DIEPCTL2_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 2. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl2;
#[doc = "DIEPINT2 (rw) register accessor: This register contains the interrupts for the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint2`] module"]
pub type DIEPINT2 = crate::Reg<diepint2::DIEPINT2_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint2;
#[doc = "DIEPTSIZ2 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz2`] module"]
pub type DIEPTSIZ2 = crate::Reg<dieptsiz2::DIEPTSIZ2_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz2;
#[doc = "DIEPDMA2 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma2`] module"]
pub type DIEPDMA2 = crate::Reg<diepdma2::DIEPDMA2_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma2;
#[doc = "DTXFSTS2 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts2`] module"]
pub type DTXFSTS2 = crate::Reg<dtxfsts2::DTXFSTS2_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts2;
#[doc = "DIEPDMAB2 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab2`] module"]
pub type DIEPDMAB2 = crate::Reg<diepdmab2::DIEPDMAB2_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 2 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab2;
#[doc = "DIEPCTL3 (rw) register accessor: This register is used to control the characteristics of Endpoint 3. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl3`] module"]
pub type DIEPCTL3 = crate::Reg<diepctl3::DIEPCTL3_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 3. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl3;
#[doc = "DIEPINT3 (rw) register accessor: This register contains the interrupts for the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint3`] module"]
pub type DIEPINT3 = crate::Reg<diepint3::DIEPINT3_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint3;
#[doc = "DIEPTSIZ3 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz3`] module"]
pub type DIEPTSIZ3 = crate::Reg<dieptsiz3::DIEPTSIZ3_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz3;
#[doc = "DIEPDMA3 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma3`] module"]
pub type DIEPDMA3 = crate::Reg<diepdma3::DIEPDMA3_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma3;
#[doc = "DTXFSTS3 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts3`] module"]
pub type DTXFSTS3 = crate::Reg<dtxfsts3::DTXFSTS3_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts3;
#[doc = "DIEPDMAB3 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab3`] module"]
pub type DIEPDMAB3 = crate::Reg<diepdmab3::DIEPDMAB3_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 3 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab3;
#[doc = "DIEPCTL4 (rw) register accessor: This register is used to control the characteristics of Endpoint 4. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl4`] module"]
pub type DIEPCTL4 = crate::Reg<diepctl4::DIEPCTL4_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 4. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl4;
#[doc = "DIEPINT4 (rw) register accessor: This register contains the interrupts for the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint4`] module"]
pub type DIEPINT4 = crate::Reg<diepint4::DIEPINT4_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint4;
#[doc = "DIEPTSIZ4 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz4`] module"]
pub type DIEPTSIZ4 = crate::Reg<dieptsiz4::DIEPTSIZ4_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz4;
#[doc = "DIEPDMA4 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma4`] module"]
pub type DIEPDMA4 = crate::Reg<diepdma4::DIEPDMA4_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma4;
#[doc = "DTXFSTS4 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts4`] module"]
pub type DTXFSTS4 = crate::Reg<dtxfsts4::DTXFSTS4_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts4;
#[doc = "DIEPDMAB4 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab4`] module"]
pub type DIEPDMAB4 = crate::Reg<diepdmab4::DIEPDMAB4_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 4 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab4;
#[doc = "DIEPCTL5 (rw) register accessor: This register is used to control the characteristics of Endpoint 5. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl5`] module"]
pub type DIEPCTL5 = crate::Reg<diepctl5::DIEPCTL5_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 5. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl5;
#[doc = "DIEPINT5 (rw) register accessor: This register contains the interrupts for the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint5`] module"]
pub type DIEPINT5 = crate::Reg<diepint5::DIEPINT5_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint5;
#[doc = "DIEPTSIZ5 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz5`] module"]
pub type DIEPTSIZ5 = crate::Reg<dieptsiz5::DIEPTSIZ5_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz5;
#[doc = "DIEPDMA5 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma5`] module"]
pub type DIEPDMA5 = crate::Reg<diepdma5::DIEPDMA5_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma5;
#[doc = "DTXFSTS5 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts5`] module"]
pub type DTXFSTS5 = crate::Reg<dtxfsts5::DTXFSTS5_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts5;
#[doc = "DIEPDMAB5 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab5`] module"]
pub type DIEPDMAB5 = crate::Reg<diepdmab5::DIEPDMAB5_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 5 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab5;
#[doc = "DIEPCTL6 (rw) register accessor: This register is used to control the characteristics of Endpoint 6. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl6`] module"]
pub type DIEPCTL6 = crate::Reg<diepctl6::DIEPCTL6_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 6. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl6;
#[doc = "DIEPINT6 (rw) register accessor: This register contains the interrupts for the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint6`] module"]
pub type DIEPINT6 = crate::Reg<diepint6::DIEPINT6_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint6;
#[doc = "DIEPTSIZ6 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz6`] module"]
pub type DIEPTSIZ6 = crate::Reg<dieptsiz6::DIEPTSIZ6_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz6;
#[doc = "DIEPDMA6 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma6`] module"]
pub type DIEPDMA6 = crate::Reg<diepdma6::DIEPDMA6_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma6;
#[doc = "DTXFSTS6 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts6`] module"]
pub type DTXFSTS6 = crate::Reg<dtxfsts6::DTXFSTS6_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts6;
#[doc = "DIEPDMAB6 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab6`] module"]
pub type DIEPDMAB6 = crate::Reg<diepdmab6::DIEPDMAB6_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 6 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab6;
#[doc = "DIEPCTL7 (rw) register accessor: This register is used to control the characteristics of Endpoint 7. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl7`] module"]
pub type DIEPCTL7 = crate::Reg<diepctl7::DIEPCTL7_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 7. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl7;
#[doc = "DIEPINT7 (rw) register accessor: This register contains the interrupts for the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint7`] module"]
pub type DIEPINT7 = crate::Reg<diepint7::DIEPINT7_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint7;
#[doc = "DIEPTSIZ7 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz7`] module"]
pub type DIEPTSIZ7 = crate::Reg<dieptsiz7::DIEPTSIZ7_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz7;
#[doc = "DIEPDMA7 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma7`] module"]
pub type DIEPDMA7 = crate::Reg<diepdma7::DIEPDMA7_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma7;
#[doc = "DTXFSTS7 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts7`] module"]
pub type DTXFSTS7 = crate::Reg<dtxfsts7::DTXFSTS7_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts7;
#[doc = "DIEPDMAB7 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab7`] module"]
pub type DIEPDMAB7 = crate::Reg<diepdmab7::DIEPDMAB7_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 7 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab7;
#[doc = "DIEPCTL8 (rw) register accessor: This register is used to control the characteristics of Endpoint 8. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl8`] module"]
pub type DIEPCTL8 = crate::Reg<diepctl8::DIEPCTL8_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 8. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl8;
#[doc = "DIEPINT8 (rw) register accessor: This register contains the interrupts for the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint8`] module"]
pub type DIEPINT8 = crate::Reg<diepint8::DIEPINT8_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint8;
#[doc = "DIEPTSIZ8 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz8`] module"]
pub type DIEPTSIZ8 = crate::Reg<dieptsiz8::DIEPTSIZ8_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz8;
#[doc = "DIEPDMA8 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma8`] module"]
pub type DIEPDMA8 = crate::Reg<diepdma8::DIEPDMA8_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma8;
#[doc = "DTXFSTS8 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts8`] module"]
pub type DTXFSTS8 = crate::Reg<dtxfsts8::DTXFSTS8_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts8;
#[doc = "DIEPDMAB8 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab8`] module"]
pub type DIEPDMAB8 = crate::Reg<diepdmab8::DIEPDMAB8_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 8 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab8;
#[doc = "DIEPCTL9 (rw) register accessor: This register is used to control the characteristics of Endpoint 9. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl9`] module"]
pub type DIEPCTL9 = crate::Reg<diepctl9::DIEPCTL9_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 9. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl9;
#[doc = "DIEPINT9 (rw) register accessor: This register contains the interrupts for the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint9`] module"]
pub type DIEPINT9 = crate::Reg<diepint9::DIEPINT9_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint9;
#[doc = "DIEPTSIZ9 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz9`] module"]
pub type DIEPTSIZ9 = crate::Reg<dieptsiz9::DIEPTSIZ9_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz9;
#[doc = "DIEPDMA9 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma9`] module"]
pub type DIEPDMA9 = crate::Reg<diepdma9::DIEPDMA9_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma9;
#[doc = "DTXFSTS9 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts9`] module"]
pub type DTXFSTS9 = crate::Reg<dtxfsts9::DTXFSTS9_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts9;
#[doc = "DIEPDMAB9 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab9`] module"]
pub type DIEPDMAB9 = crate::Reg<diepdmab9::DIEPDMAB9_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab9;
#[doc = "DIEPCTL10 (rw) register accessor: This register is used to control the characteristics of Endpoint 10. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl10`] module"]
pub type DIEPCTL10 = crate::Reg<diepctl10::DIEPCTL10_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 10. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl10;
#[doc = "DIEPINT10 (rw) register accessor: This register contains the interrupts for the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint10`] module"]
pub type DIEPINT10 = crate::Reg<diepint10::DIEPINT10_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint10;
#[doc = "DIEPTSIZ10 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz10`] module"]
pub type DIEPTSIZ10 = crate::Reg<dieptsiz10::DIEPTSIZ10_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz10;
#[doc = "DIEPDMA10 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma10`] module"]
pub type DIEPDMA10 = crate::Reg<diepdma10::DIEPDMA10_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma10;
#[doc = "DTXFSTS10 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts10`] module"]
pub type DTXFSTS10 = crate::Reg<dtxfsts10::DTXFSTS10_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts10;
#[doc = "DIEPDMAB10 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab10`] module"]
pub type DIEPDMAB10 = crate::Reg<diepdmab10::DIEPDMAB10_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 10 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab10;
#[doc = "DIEPCTL11 (rw) register accessor: This register is used to control the characteristics of Endpoint 11. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl11`] module"]
pub type DIEPCTL11 = crate::Reg<diepctl11::DIEPCTL11_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 11. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl11;
#[doc = "DIEPINT11 (rw) register accessor: This register contains the interrupts for the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint11`] module"]
pub type DIEPINT11 = crate::Reg<diepint11::DIEPINT11_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint11;
#[doc = "DIEPTSIZ11 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz11`] module"]
pub type DIEPTSIZ11 = crate::Reg<dieptsiz11::DIEPTSIZ11_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz11;
#[doc = "DIEPDMA11 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma11`] module"]
pub type DIEPDMA11 = crate::Reg<diepdma11::DIEPDMA11_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma11;
#[doc = "DTXFSTS11 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts11`] module"]
pub type DTXFSTS11 = crate::Reg<dtxfsts11::DTXFSTS11_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts11;
#[doc = "DIEPDMAB11 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab11`] module"]
pub type DIEPDMAB11 = crate::Reg<diepdmab11::DIEPDMAB11_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 11 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab11;
#[doc = "DIEPCTL12 (rw) register accessor: This register is used to control the characteristics of Endpoint 12. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl12`] module"]
pub type DIEPCTL12 = crate::Reg<diepctl12::DIEPCTL12_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 12. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl12;
#[doc = "DIEPINT12 (rw) register accessor: This register contains the interrupts for the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint12`] module"]
pub type DIEPINT12 = crate::Reg<diepint12::DIEPINT12_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint12;
#[doc = "DIEPTSIZ12 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz12`] module"]
pub type DIEPTSIZ12 = crate::Reg<dieptsiz12::DIEPTSIZ12_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz12;
#[doc = "DIEPDMA12 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma12`] module"]
pub type DIEPDMA12 = crate::Reg<diepdma12::DIEPDMA12_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma12;
#[doc = "DTXFSTS12 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts12`] module"]
pub type DTXFSTS12 = crate::Reg<dtxfsts12::DTXFSTS12_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts12;
#[doc = "DIEPDMAB12 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab12`] module"]
pub type DIEPDMAB12 = crate::Reg<diepdmab12::DIEPDMAB12_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 12 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab12;
#[doc = "DIEPCTL13 (rw) register accessor: This register is used to control the characteristics of Endpoint 13. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl13`] module"]
pub type DIEPCTL13 = crate::Reg<diepctl13::DIEPCTL13_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 13. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl13;
#[doc = "DIEPINT13 (rw) register accessor: This register contains the interrupts for the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint13`] module"]
pub type DIEPINT13 = crate::Reg<diepint13::DIEPINT13_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint13;
#[doc = "DIEPTSIZ13 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz13`] module"]
pub type DIEPTSIZ13 = crate::Reg<dieptsiz13::DIEPTSIZ13_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz13;
#[doc = "DIEPDMA13 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma13`] module"]
pub type DIEPDMA13 = crate::Reg<diepdma13::DIEPDMA13_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma13;
#[doc = "DTXFSTS13 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts13`] module"]
pub type DTXFSTS13 = crate::Reg<dtxfsts13::DTXFSTS13_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts13;
#[doc = "DIEPDMAB13 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab13`] module"]
pub type DIEPDMAB13 = crate::Reg<diepdmab13::DIEPDMAB13_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 13 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab13;
#[doc = "DIEPCTL14 (rw) register accessor: This register is used to control the characteristics of Endpoint 14. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl14`] module"]
pub type DIEPCTL14 = crate::Reg<diepctl14::DIEPCTL14_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 14. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl14;
#[doc = "DIEPINT14 (rw) register accessor: This register contains the interrupts for the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint14`] module"]
pub type DIEPINT14 = crate::Reg<diepint14::DIEPINT14_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint14;
#[doc = "DIEPTSIZ14 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz14`] module"]
pub type DIEPTSIZ14 = crate::Reg<dieptsiz14::DIEPTSIZ14_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz14;
#[doc = "DIEPDMA14 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma14`] module"]
pub type DIEPDMA14 = crate::Reg<diepdma14::DIEPDMA14_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma14;
#[doc = "DTXFSTS14 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts14`] module"]
pub type DTXFSTS14 = crate::Reg<dtxfsts14::DTXFSTS14_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts14;
#[doc = "DIEPDMAB14 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab14`] module"]
pub type DIEPDMAB14 = crate::Reg<diepdmab14::DIEPDMAB14_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 14 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab14;
#[doc = "DIEPCTL15 (rw) register accessor: This register is used to control the characteristics of Endpoint 15. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepctl15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepctl15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepctl15`] module"]
pub type DIEPCTL15 = crate::Reg<diepctl15::DIEPCTL15_SPEC>;
#[doc = "This register is used to control the characteristics of Endpoint 15. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepctl15;
#[doc = "DIEPINT15 (rw) register accessor: This register contains the interrupts for the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepint15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepint15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepint15`] module"]
pub type DIEPINT15 = crate::Reg<diepint15::DIEPINT15_SPEC>;
#[doc = "This register contains the interrupts for the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepint15;
#[doc = "DIEPTSIZ15 (rw) register accessor: This register reflects the Transfer Size of the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dieptsiz15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dieptsiz15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dieptsiz15`] module"]
pub type DIEPTSIZ15 = crate::Reg<dieptsiz15::DIEPTSIZ15_SPEC>;
#[doc = "This register reflects the Transfer Size of the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dieptsiz15;
#[doc = "DIEPDMA15 (rw) register accessor: This register contains the DMA Address for the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdma15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`diepdma15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdma15`] module"]
pub type DIEPDMA15 = crate::Reg<diepdma15::DIEPDMA15_SPEC>;
#[doc = "This register contains the DMA Address for the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdma15;
#[doc = "DTXFSTS15 (r) register accessor: This register reflects the status of the IN Endpoint Transmit FIFO Status Register 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtxfsts15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dtxfsts15`] module"]
pub type DTXFSTS15 = crate::Reg<dtxfsts15::DTXFSTS15_SPEC>;
#[doc = "This register reflects the status of the IN Endpoint Transmit FIFO Status Register 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod dtxfsts15;
#[doc = "DIEPDMAB15 (r) register accessor: This register contains the DMA Buffer Address of the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@diepdmab15`] module"]
pub type DIEPDMAB15 = crate::Reg<diepdmab15::DIEPDMAB15_SPEC>;
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 15 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint."]
pub mod diepdmab15;
#[doc = "DOEPCTL0 (rw) register accessor: This register is used to control the characteristics of the OUT Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl0`] module"]
pub type DOEPCTL0 = crate::Reg<doepctl0::DOEPCTL0_SPEC>;
#[doc = "This register is used to control the characteristics of the OUT Endpoint 0 of the Device controller."]
pub mod doepctl0;
#[doc = "DOEPINT0 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint0`] module"]
pub type DOEPINT0 = crate::Reg<doepint0::DOEPINT0_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 0 of the Device controller."]
pub mod doepint0;
#[doc = "DOEPTSIZ0 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz0`] module"]
pub type DOEPTSIZ0 = crate::Reg<doeptsiz0::DOEPTSIZ0_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 0 of the Device controller."]
pub mod doeptsiz0;
#[doc = "DOEPDMA0 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma0`] module"]
pub type DOEPDMA0 = crate::Reg<doepdma0::DOEPDMA0_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 0 of the Device controller."]
pub mod doepdma0;
#[doc = "DOEPDMAB0 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 0 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab0`] module"]
pub type DOEPDMAB0 = crate::Reg<doepdmab0::DOEPDMAB0_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 0 of the Device controller."]
pub mod doepdmab0;
#[doc = "DOEPCTL1 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 1 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl1`] module"]
pub type DOEPCTL1 = crate::Reg<doepctl1::DOEPCTL1_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 1 of the Device controller."]
pub mod doepctl1;
#[doc = "DOEPINT1 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 1 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint1`] module"]
pub type DOEPINT1 = crate::Reg<doepint1::DOEPINT1_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 1 of the Device controller."]
pub mod doepint1;
#[doc = "DOEPTSIZ1 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 1 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz1`] module"]
pub type DOEPTSIZ1 = crate::Reg<doeptsiz1::DOEPTSIZ1_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 1 of the Device controller."]
pub mod doeptsiz1;
#[doc = "DOEPDMA1 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 1 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma1`] module"]
pub type DOEPDMA1 = crate::Reg<doepdma1::DOEPDMA1_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 1 of the Device controller."]
pub mod doepdma1;
#[doc = "DOEPDMAB1 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 1 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab1`] module"]
pub type DOEPDMAB1 = crate::Reg<doepdmab1::DOEPDMAB1_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 1 of the Device controller."]
pub mod doepdmab1;
#[doc = "DOEPCTL2 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 2 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl2`] module"]
pub type DOEPCTL2 = crate::Reg<doepctl2::DOEPCTL2_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 2 of the Device controller."]
pub mod doepctl2;
#[doc = "DOEPINT2 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 2 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint2`] module"]
pub type DOEPINT2 = crate::Reg<doepint2::DOEPINT2_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 2 of the Device controller."]
pub mod doepint2;
#[doc = "DOEPTSIZ2 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 2 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz2`] module"]
pub type DOEPTSIZ2 = crate::Reg<doeptsiz2::DOEPTSIZ2_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 2 of the Device controller."]
pub mod doeptsiz2;
#[doc = "DOEPDMA2 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 2 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma2`] module"]
pub type DOEPDMA2 = crate::Reg<doepdma2::DOEPDMA2_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 2 of the Device controller."]
pub mod doepdma2;
#[doc = "DOEPDMAB2 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 2 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab2`] module"]
pub type DOEPDMAB2 = crate::Reg<doepdmab2::DOEPDMAB2_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 2 of the Device controller."]
pub mod doepdmab2;
#[doc = "DOEPCTL3 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 3 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl3`] module"]
pub type DOEPCTL3 = crate::Reg<doepctl3::DOEPCTL3_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 3 of the Device controller."]
pub mod doepctl3;
#[doc = "DOEPINT3 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 3 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint3`] module"]
pub type DOEPINT3 = crate::Reg<doepint3::DOEPINT3_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 3 of the Device controller."]
pub mod doepint3;
#[doc = "DOEPTSIZ3 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 3 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz3`] module"]
pub type DOEPTSIZ3 = crate::Reg<doeptsiz3::DOEPTSIZ3_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 3 of the Device controller."]
pub mod doeptsiz3;
#[doc = "DOEPDMA3 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 3 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma3`] module"]
pub type DOEPDMA3 = crate::Reg<doepdma3::DOEPDMA3_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 3 of the Device controller."]
pub mod doepdma3;
#[doc = "DOEPDMAB3 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 3 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab3`] module"]
pub type DOEPDMAB3 = crate::Reg<doepdmab3::DOEPDMAB3_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 3 of the Device controller."]
pub mod doepdmab3;
#[doc = "DOEPCTL4 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 4 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl4`] module"]
pub type DOEPCTL4 = crate::Reg<doepctl4::DOEPCTL4_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 4 of the Device controller."]
pub mod doepctl4;
#[doc = "DOEPINT4 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 4 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint4`] module"]
pub type DOEPINT4 = crate::Reg<doepint4::DOEPINT4_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 4 of the Device controller."]
pub mod doepint4;
#[doc = "DOEPTSIZ4 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 4 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz4`] module"]
pub type DOEPTSIZ4 = crate::Reg<doeptsiz4::DOEPTSIZ4_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 4 of the Device controller."]
pub mod doeptsiz4;
#[doc = "DOEPDMA4 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 4 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma4`] module"]
pub type DOEPDMA4 = crate::Reg<doepdma4::DOEPDMA4_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 4 of the Device controller."]
pub mod doepdma4;
#[doc = "DOEPDMAB4 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 4 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab4`] module"]
pub type DOEPDMAB4 = crate::Reg<doepdmab4::DOEPDMAB4_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 4 of the Device controller."]
pub mod doepdmab4;
#[doc = "DOEPCTL5 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 5 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl5`] module"]
pub type DOEPCTL5 = crate::Reg<doepctl5::DOEPCTL5_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 5 of the Device controller."]
pub mod doepctl5;
#[doc = "DOEPINT5 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 5 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint5`] module"]
pub type DOEPINT5 = crate::Reg<doepint5::DOEPINT5_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 5 of the Device controller."]
pub mod doepint5;
#[doc = "DOEPTSIZ5 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 5 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz5`] module"]
pub type DOEPTSIZ5 = crate::Reg<doeptsiz5::DOEPTSIZ5_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 5 of the Device controller."]
pub mod doeptsiz5;
#[doc = "DOEPDMA5 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 5 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma5`] module"]
pub type DOEPDMA5 = crate::Reg<doepdma5::DOEPDMA5_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 5 of the Device controller."]
pub mod doepdma5;
#[doc = "DOEPDMAB5 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 5 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab5`] module"]
pub type DOEPDMAB5 = crate::Reg<doepdmab5::DOEPDMAB5_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 5 of the Device controller."]
pub mod doepdmab5;
#[doc = "DOEPCTL6 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 6 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl6`] module"]
pub type DOEPCTL6 = crate::Reg<doepctl6::DOEPCTL6_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 6 of the Device controller."]
pub mod doepctl6;
#[doc = "DOEPINT6 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 6 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint6`] module"]
pub type DOEPINT6 = crate::Reg<doepint6::DOEPINT6_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 6 of the Device controller."]
pub mod doepint6;
#[doc = "DOEPTSIZ6 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 6 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz6`] module"]
pub type DOEPTSIZ6 = crate::Reg<doeptsiz6::DOEPTSIZ6_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 6 of the Device controller."]
pub mod doeptsiz6;
#[doc = "DOEPDMA6 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 6 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma6`] module"]
pub type DOEPDMA6 = crate::Reg<doepdma6::DOEPDMA6_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 6 of the Device controller."]
pub mod doepdma6;
#[doc = "DOEPDMAB6 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 6 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab6`] module"]
pub type DOEPDMAB6 = crate::Reg<doepdmab6::DOEPDMAB6_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 6 of the Device controller."]
pub mod doepdmab6;
#[doc = "DOEPCTL7 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 7 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl7`] module"]
pub type DOEPCTL7 = crate::Reg<doepctl7::DOEPCTL7_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 7 of the Device controller."]
pub mod doepctl7;
#[doc = "DOEPINT7 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 7 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint7`] module"]
pub type DOEPINT7 = crate::Reg<doepint7::DOEPINT7_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 7 of the Device controller."]
pub mod doepint7;
#[doc = "DOEPTSIZ7 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 7 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz7`] module"]
pub type DOEPTSIZ7 = crate::Reg<doeptsiz7::DOEPTSIZ7_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 7 of the Device controller."]
pub mod doeptsiz7;
#[doc = "DOEPDMA7 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 7 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma7`] module"]
pub type DOEPDMA7 = crate::Reg<doepdma7::DOEPDMA7_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 7 of the Device controller."]
pub mod doepdma7;
#[doc = "DOEPDMAB7 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 7 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab7`] module"]
pub type DOEPDMAB7 = crate::Reg<doepdmab7::DOEPDMAB7_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 7 of the Device controller."]
pub mod doepdmab7;
#[doc = "DOEPCTL8 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 8 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl8`] module"]
pub type DOEPCTL8 = crate::Reg<doepctl8::DOEPCTL8_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 8 of the Device controller."]
pub mod doepctl8;
#[doc = "DOEPINT8 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 8 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint8`] module"]
pub type DOEPINT8 = crate::Reg<doepint8::DOEPINT8_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 8 of the Device controller."]
pub mod doepint8;
#[doc = "DOEPTSIZ8 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 8 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz8`] module"]
pub type DOEPTSIZ8 = crate::Reg<doeptsiz8::DOEPTSIZ8_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 8 of the Device controller."]
pub mod doeptsiz8;
#[doc = "DOEPDMA8 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 8 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma8`] module"]
pub type DOEPDMA8 = crate::Reg<doepdma8::DOEPDMA8_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 8 of the Device controller."]
pub mod doepdma8;
#[doc = "DOEPDMAB8 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 8 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab8`] module"]
pub type DOEPDMAB8 = crate::Reg<doepdmab8::DOEPDMAB8_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 8 of the Device controller."]
pub mod doepdmab8;
#[doc = "DOEPCTL9 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 9 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl9`] module"]
pub type DOEPCTL9 = crate::Reg<doepctl9::DOEPCTL9_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 9 of the Device controller."]
pub mod doepctl9;
#[doc = "DOEPINT9 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 9 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint9`] module"]
pub type DOEPINT9 = crate::Reg<doepint9::DOEPINT9_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 9 of the Device controller."]
pub mod doepint9;
#[doc = "DOEPTSIZ9 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 9 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz9`] module"]
pub type DOEPTSIZ9 = crate::Reg<doeptsiz9::DOEPTSIZ9_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 9 of the Device controller."]
pub mod doeptsiz9;
#[doc = "DOEPDMA9 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 9 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma9`] module"]
pub type DOEPDMA9 = crate::Reg<doepdma9::DOEPDMA9_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 9 of the Device controller."]
pub mod doepdma9;
#[doc = "DOEPDMAB9 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 9 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab9`] module"]
pub type DOEPDMAB9 = crate::Reg<doepdmab9::DOEPDMAB9_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 9 of the Device controller."]
pub mod doepdmab9;
#[doc = "DOEPCTL10 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 10 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl10`] module"]
pub type DOEPCTL10 = crate::Reg<doepctl10::DOEPCTL10_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 10 of the Device controller."]
pub mod doepctl10;
#[doc = "DOEPINT10 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 10 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint10`] module"]
pub type DOEPINT10 = crate::Reg<doepint10::DOEPINT10_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 10 of the Device controller."]
pub mod doepint10;
#[doc = "DOEPTSIZ10 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 10 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz10`] module"]
pub type DOEPTSIZ10 = crate::Reg<doeptsiz10::DOEPTSIZ10_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 10 of the Device controller."]
pub mod doeptsiz10;
#[doc = "DOEPDMA10 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 10 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma10`] module"]
pub type DOEPDMA10 = crate::Reg<doepdma10::DOEPDMA10_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 10 of the Device controller."]
pub mod doepdma10;
#[doc = "DOEPDMAB10 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 10 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab10`] module"]
pub type DOEPDMAB10 = crate::Reg<doepdmab10::DOEPDMAB10_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 10 of the Device controller."]
pub mod doepdmab10;
#[doc = "DOEPCTL11 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 11 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl11`] module"]
pub type DOEPCTL11 = crate::Reg<doepctl11::DOEPCTL11_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 11 of the Device controller."]
pub mod doepctl11;
#[doc = "DOEPINT11 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 11 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint11`] module"]
pub type DOEPINT11 = crate::Reg<doepint11::DOEPINT11_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 11 of the Device controller."]
pub mod doepint11;
#[doc = "DOEPTSIZ11 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 11 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz11`] module"]
pub type DOEPTSIZ11 = crate::Reg<doeptsiz11::DOEPTSIZ11_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 11 of the Device controller."]
pub mod doeptsiz11;
#[doc = "DOEPDMA11 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 11 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma11`] module"]
pub type DOEPDMA11 = crate::Reg<doepdma11::DOEPDMA11_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 11 of the Device controller."]
pub mod doepdma11;
#[doc = "DOEPDMAB11 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 11 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab11`] module"]
pub type DOEPDMAB11 = crate::Reg<doepdmab11::DOEPDMAB11_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 11 of the Device controller."]
pub mod doepdmab11;
#[doc = "DOEPCTL12 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 12 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl12`] module"]
pub type DOEPCTL12 = crate::Reg<doepctl12::DOEPCTL12_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 12 of the Device controller."]
pub mod doepctl12;
#[doc = "DOEPINT12 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 12 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint12`] module"]
pub type DOEPINT12 = crate::Reg<doepint12::DOEPINT12_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 12 of the Device controller."]
pub mod doepint12;
#[doc = "DOEPTSIZ12 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 12 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz12`] module"]
pub type DOEPTSIZ12 = crate::Reg<doeptsiz12::DOEPTSIZ12_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 12 of the Device controller."]
pub mod doeptsiz12;
#[doc = "DOEPDMA12 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 12 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma12`] module"]
pub type DOEPDMA12 = crate::Reg<doepdma12::DOEPDMA12_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 12 of the Device controller."]
pub mod doepdma12;
#[doc = "DOEPDMAB12 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 12 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab12`] module"]
pub type DOEPDMAB12 = crate::Reg<doepdmab12::DOEPDMAB12_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 12 of the Device controller."]
pub mod doepdmab12;
#[doc = "DOEPCTL13 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 13 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl13`] module"]
pub type DOEPCTL13 = crate::Reg<doepctl13::DOEPCTL13_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 13 of the Device controller."]
pub mod doepctl13;
#[doc = "DOEPINT13 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 13 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint13`] module"]
pub type DOEPINT13 = crate::Reg<doepint13::DOEPINT13_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 13 of the Device controller."]
pub mod doepint13;
#[doc = "DOEPTSIZ13 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 13 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz13`] module"]
pub type DOEPTSIZ13 = crate::Reg<doeptsiz13::DOEPTSIZ13_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 13 of the Device controller."]
pub mod doeptsiz13;
#[doc = "DOEPDMA13 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 13 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma13`] module"]
pub type DOEPDMA13 = crate::Reg<doepdma13::DOEPDMA13_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 13 of the Device controller."]
pub mod doepdma13;
#[doc = "DOEPDMAB13 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 13 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab13`] module"]
pub type DOEPDMAB13 = crate::Reg<doepdmab13::DOEPDMAB13_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 13 of the Device controller."]
pub mod doepdmab13;
#[doc = "DOEPCTL14 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 14 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl14`] module"]
pub type DOEPCTL14 = crate::Reg<doepctl14::DOEPCTL14_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 14 of the Device controller."]
pub mod doepctl14;
#[doc = "DOEPINT14 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 14 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint14`] module"]
pub type DOEPINT14 = crate::Reg<doepint14::DOEPINT14_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 14 of the Device controller."]
pub mod doepint14;
#[doc = "DOEPTSIZ14 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 14 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz14`] module"]
pub type DOEPTSIZ14 = crate::Reg<doeptsiz14::DOEPTSIZ14_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 14 of the Device controller."]
pub mod doeptsiz14;
#[doc = "DOEPDMA14 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 14 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma14`] module"]
pub type DOEPDMA14 = crate::Reg<doepdma14::DOEPDMA14_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 14 of the Device controller."]
pub mod doepdma14;
#[doc = "DOEPDMAB14 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 14 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab14`] module"]
pub type DOEPDMAB14 = crate::Reg<doepdmab14::DOEPDMAB14_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 14 of the Device controller."]
pub mod doepdmab14;
#[doc = "DOEPCTL15 (rw) register accessor: This register is used to control the characteristics of OUT Endpoint 15 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepctl15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepctl15`] module"]
pub type DOEPCTL15 = crate::Reg<doepctl15::DOEPCTL15_SPEC>;
#[doc = "This register is used to control the characteristics of OUT Endpoint 15 of the Device controller."]
pub mod doepctl15;
#[doc = "DOEPINT15 (rw) register accessor: This register contains the interrupts for the OUT Endpoint 15 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepint15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepint15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepint15`] module"]
pub type DOEPINT15 = crate::Reg<doepint15::DOEPINT15_SPEC>;
#[doc = "This register contains the interrupts for the OUT Endpoint 15 of the Device controller."]
pub mod doepint15;
#[doc = "DOEPTSIZ15 (rw) register accessor: This register contains the Transfer Size for the OUT Endpoint 15 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doeptsiz15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doeptsiz15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doeptsiz15`] module"]
pub type DOEPTSIZ15 = crate::Reg<doeptsiz15::DOEPTSIZ15_SPEC>;
#[doc = "This register contains the Transfer Size for the OUT Endpoint 15 of the Device controller."]
pub mod doeptsiz15;
#[doc = "DOEPDMA15 (rw) register accessor: This register contains the DMA Address for the OUT Endpoint 15 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdma15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepdma15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdma15`] module"]
pub type DOEPDMA15 = crate::Reg<doepdma15::DOEPDMA15_SPEC>;
#[doc = "This register contains the DMA Address for the OUT Endpoint 15 of the Device controller."]
pub mod doepdma15;
#[doc = "DOEPDMAB15 (r) register accessor: This register contains the DMA Buffer Address for the OUT Endpoint 15 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doepdmab15`] module"]
pub type DOEPDMAB15 = crate::Reg<doepdmab15::DOEPDMAB15_SPEC>;
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 15 of the Device controller."]
pub mod doepdmab15;
#[doc = "PCGCCTL (rw) register accessor: This register is used to control the Power and Clock Gating characteristics of the controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`pcgcctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcgcctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcgcctl`] module"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "This register is used to control the Power and Clock Gating characteristics of the controller."]
pub mod pcgcctl;
#[doc = "GSTARFXDIS (rw) register accessor: This register is used to disable STAR fixes added in the controller. The application can set the register fields to operate with the functionality before the fix was done.\n\nYou can [`read`](crate::Reg::read) this register and get [`gstarfxdis::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gstarfxdis::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gstarfxdis`] module"]
pub type GSTARFXDIS = crate::Reg<gstarfxdis::GSTARFXDIS_SPEC>;
#[doc = "This register is used to disable STAR fixes added in the controller. The application can set the register fields to operate with the functionality before the fix was done."]
pub mod gstarfxdis;
